use super::config::TypingConfig;
use super::timing::{
    backspace_delay, delay_after, delay_before, delay_inside, first_backspace_delay,
    typo_retype_delay,
};
use super::token::{InputToken, TokenKind};
use super::tokenizer::tokenize_with_config;
use super::typo::{can_simulate_typo, plan_for_token, should_apply_typo};
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

        if !sleep_cancellable(delay_before(&token, config), should_exit) {
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

        if !sleep_cancellable(delay_after(&token, index, config), should_exit) {
            return false;
        }
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
    if can_simulate_typo(config) && should_apply_typo(&token.text, index, config.typo_rate_percent)
    {
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
    let wrong_chars: Vec<char> = plan.wrong_text.chars().collect();
    let keep_chars = wrong_chars.len().saturating_sub(plan.backspaces);
    let mut typed_wrong_chars: usize = 0;

    for (offset, ch) in wrong_chars.iter().enumerate() {
        if cancelled(should_exit) {
            cleanup_backspaces(enigo, typed_wrong_chars.saturating_sub(keep_chars));
            return false;
        }
        enigo.key_sequence(&ch.to_string());
        typed_wrong_chars += 1;
        if offset + 1 < wrong_chars.len() {
            if !sleep_cancellable(delay_inside(token, index + offset, config), should_exit) {
                cleanup_backspaces(enigo, typed_wrong_chars.saturating_sub(keep_chars));
                return false;
            }
        }
    }

    let mut remaining_backspaces = plan.backspaces;
    if !sleep_cancellable(first_backspace_delay(config), should_exit) {
        cleanup_backspaces(enigo, remaining_backspaces);
        return false;
    }

    while remaining_backspaces > 0 {
        if cancelled(should_exit) {
            cleanup_backspaces(enigo, remaining_backspaces);
            return false;
        }

        enigo.key_click(enigo::Key::Backspace);
        remaining_backspaces -= 1;

        if !sleep_cancellable(backspace_delay(config), should_exit) {
            cleanup_backspaces(enigo, remaining_backspaces);
            return false;
        }
    }

    if !sleep_cancellable(typo_retype_delay(token, index, config), should_exit) {
        return false;
    }

    let retype_chars: Vec<char> = plan.retype_text.chars().collect();
    for (offset, ch) in retype_chars.iter().enumerate() {
        if cancelled(should_exit) {
            return false;
        }
        enigo.key_sequence(&ch.to_string());
        if offset + 1 < retype_chars.len() {
            if !sleep_cancellable(
                typo_retype_delay(token, index + offset, config),
                should_exit,
            ) {
                return false;
            }
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
            if !sleep_cancellable(delay_inside(token, index + offset, config), should_exit) {
                return false;
            }
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

fn sleep_cancellable(duration: Duration, should_exit: &AtomicBool) -> bool {
    let step = Duration::from_millis(10);
    let mut remaining = duration;

    while !remaining.is_zero() {
        if cancelled(should_exit) {
            return false;
        }

        let chunk = remaining.min(step);
        thread::sleep(chunk);
        remaining -= chunk;
    }

    !cancelled(should_exit)
}

fn cleanup_backspaces(enigo: &mut Enigo, count: usize) {
    for _ in 0..count {
        enigo.key_click(enigo::Key::Backspace);
    }
}
