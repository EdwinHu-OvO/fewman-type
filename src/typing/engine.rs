use super::config::TypingConfig;
use super::timing::{delay_after, delay_before, delay_inside};
use super::token::{InputToken, TokenKind};
use super::tokenizer::tokenize_with_config;
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
        if should_exit.load(Ordering::SeqCst) {
            return false;
        }

        thread::sleep(delay_before(&token, config));
        if should_exit.load(Ordering::SeqCst) {
            return false;
        }

        match token.kind {
            TokenKind::Newline => press_return(enigo),
            _ => type_token(enigo, &token, index, config, should_exit),
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
) {
    let chars: Vec<char> = token.text.chars().collect();
    for (offset, ch) in chars.iter().enumerate() {
        if should_exit.load(Ordering::SeqCst) {
            return;
        }

        enigo.key_sequence(&ch.to_string());
        if offset + 1 < chars.len() {
            thread::sleep(delay_inside(token, index + offset, config));
        }
    }
}

fn press_return(enigo: &mut Enigo) {
    enigo.key_down(enigo::Key::Return);
    thread::sleep(Duration::from_millis(10));
    enigo.key_up(enigo::Key::Return);
}
