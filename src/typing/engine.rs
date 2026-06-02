use super::config::TypingConfig;
use super::timing::{backspace_delay, delay_after, delay_before, delay_inside, typo_retype_delay};
use super::token::{InputToken, TokenKind};
use super::tokenizer::tokenize_with_config;
use super::typo::{plan_for_token, should_apply_typo};
use enigo::{Enigo, KeyboardControllable};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub fn type_text(
    enigo: &mut Enigo,
    text: &str,
    config: TypingConfig,
    should_exit: &AtomicBool,
) -> bool {
    for (index, token) in tokenize_with_config(text, config).into_iter().enumerate() {
        if cancelled(should_exit) {
            return false;
        }

        thread::sleep(delay_before(&token, config));
        if cancelled(should_exit) {
            return false;
        }

        match token.kind {
            TokenKind::Newline => press_return(enigo),
            _ => {
                if !type_token(enigo, &token, index, config, should_exit) {
                    return false;
                }
            }
        }

        thread::sleep(delay_after(&token, index, config));
    }

    true
}

fn type_token(
    enigo: &mut Enigo,
    token: &InputToken,
    index: usize,
    config: TypingConfig,
    should_exit: &AtomicBool,
) -> bool {
    if config.typo_simulation && should_apply_typo(&token.text, index, config.typo_rate_percent) {
        if let Some(plan) = plan_for_token(token, index) {
            return execute_typo_plan(enigo, token, index, config, should_exit, plan);
        }
    }

    type_chars_normal(enigo, token, index, config, should_exit)
}

fn execute_typo_plan(
    enigo: &mut Enigo,
    token: &InputToken,
    index: usize,
    config: TypingConfig,
    should_exit: &AtomicBool,
    plan: super::typo::TypoPlan,
) -> bool {
    // 1. Type wrong text normally
    let wrong_chars: Vec<char> = plan.wrong_text.chars().collect();
    for (offset, ch) in wrong_chars.iter().enumerate() {
        if cancelled(should_exit) {
            return false;
        }
        enigo.key_sequence(&ch.to_string());
        if offset + 1 < wrong_chars.len() {
            thread::sleep(delay_inside(token, index + offset, config));
        }
    }

    // Delay before backspacing (could use inner delay, here we use backspace delay for simplicity)
    thread::sleep(backspace_delay());

    // 2. Backspace
    for _ in 0..plan.backspaces {
        if cancelled(should_exit) {
            return false;
        }
        enigo.key_click(enigo::Key::Backspace);
        thread::sleep(backspace_delay());
    }

    // Delay before retyping (use typo_retype_delay to reflect the pause after correction)
    thread::sleep(typo_retype_delay(token, index, config));

    // 3. Retype correct suffix slowly
    let retype_chars: Vec<char> = plan.retype_text.chars().collect();
    for (offset, ch) in retype_chars.iter().enumerate() {
        if cancelled(should_exit) {
            return false;
        }
        enigo.key_sequence(&ch.to_string());
        if offset + 1 < retype_chars.len() {
            thread::sleep(typo_retype_delay(token, index + offset, config));
        }
    }

    true
}

fn type_chars_normal(
    enigo: &mut Enigo,
    token: &InputToken,
    index: usize,
    config: TypingConfig,
    should_exit: &AtomicBool,
) -> bool {
    let chars: Vec<char> = token.text.chars().collect();
    for (offset, ch) in chars.iter().enumerate() {
        if cancelled(should_exit) {
            return false;
        }

        enigo.key_sequence(&ch.to_string());
        if offset + 1 < chars.len() {
            thread::sleep(delay_inside(token, index + offset, config));
        }
    }
    true
}

fn press_return(enigo: &mut Enigo) {
    enigo.key_down(enigo::Key::Return);
    thread::sleep(Duration::from_millis(10));
    enigo.key_up(enigo::Key::Return);
}

fn cancelled(should_exit: &AtomicBool) -> bool {
    should_exit.load(Ordering::SeqCst)
}
