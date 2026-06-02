use super::char_class::{stable_jitter_num, stable_jitter_text};
use super::config::TypingConfig;
use super::dictionary::word_frequency_scale_per_mille;
use super::token::{InputToken, TokenKind};
use std::time::Duration;

pub(crate) fn backspace_delay(config: TypingConfig) -> Duration {
    Duration::from_millis(
        config
            .base_interval_ms
            .max(5)
            .saturating_mul(500)
            .saturating_div(1000)
            .min(800),
    )
}

pub(crate) fn first_backspace_delay(config: TypingConfig) -> Duration {
    Duration::from_millis(backspace_delay(config).as_millis() as u64 * 6 / 5)
}

pub(crate) fn typo_retype_delay(token: &InputToken, salt: usize, config: TypingConfig) -> Duration {
    let base = typo_retype_base_delay(token, salt, config);
    Duration::from_millis(((base.as_millis() as u64).saturating_mul(3) + 1) / 2)
}

fn typo_retype_base_delay(token: &InputToken, salt: usize, config: TypingConfig) -> Duration {
    match token.kind {
        TokenKind::CjkWord => Duration::from_millis(
            cjk_word_delay_budget_ms(token, config) / cjk_word_char_count(token),
        ),
        _ => delay_inside(token, salt, config),
    }
}

pub(crate) fn delay_inside(token: &InputToken, salt: usize, config: TypingConfig) -> Duration {
    let base = config.base_interval_ms.max(5);
    match token.kind {
        TokenKind::CjkWord => cjk_word_inner_delay(token, config),
        TokenKind::Word => Duration::from_millis((base / 2).max(8) + stable_jitter_num(salt) % 16),
        TokenKind::Whitespace => {
            Duration::from_millis((base / 3).max(8) + stable_jitter_num(salt) % 12)
        }
        TokenKind::Punctuation | TokenKind::Newline => Duration::from_millis(10),
    }
}

pub(crate) fn delay_before(token: &InputToken, config: TypingConfig) -> Duration {
    match token.kind {
        TokenKind::CjkWord => cjk_word_before_delay(token, config),
        _ => Duration::ZERO,
    }
}

pub(crate) fn delay_after(token: &InputToken, index: usize, config: TypingConfig) -> Duration {
    let base = config.base_interval_ms.max(5);
    match token.kind {
        TokenKind::CjkWord => Duration::ZERO,
        TokenKind::Word => {
            Duration::from_millis(base + stable_jitter_text(&token.text, index) % base.max(12))
        }
        TokenKind::Whitespace => {
            Duration::from_millis((base / 2).max(10) + stable_jitter_text(&token.text, index) % 24)
        }
        TokenKind::Punctuation => Duration::from_millis(
            base.saturating_mul(3) + stable_jitter_text(&token.text, index) % base.max(40),
        ),
        TokenKind::Newline => {
            Duration::from_millis(base.saturating_mul(4) + stable_jitter_num(index) % base.max(40))
        }
    }
}

fn cjk_word_inner_delay(token: &InputToken, config: TypingConfig) -> Duration {
    if config.skip_word_inner_delay {
        return Duration::ZERO;
    }

    Duration::from_millis(cjk_word_delay_budget_ms(token, config) / cjk_word_char_count(token))
}

fn cjk_word_before_delay(token: &InputToken, config: TypingConfig) -> Duration {
    let budget = cjk_word_delay_budget_ms(token, config);
    let char_count = cjk_word_char_count(token);
    if config.skip_word_inner_delay || char_count <= 1 {
        return Duration::from_millis(budget);
    }

    let inner_delay = budget / char_count;
    Duration::from_millis(budget.saturating_sub(inner_delay * (char_count - 1)))
}

pub(crate) fn cjk_word_delay_budget_ms(token: &InputToken, config: TypingConfig) -> u64 {
    let base_budget = cjk_word_char_count(token)
        .saturating_mul(config.base_interval_ms.max(5))
        .saturating_mul(cjk_word_delay_scale_tenths(token))
        / 10;

    base_budget.saturating_mul(word_frequency_scale_per_mille(&token.text)) / 1000
}

fn cjk_word_char_count(token: &InputToken) -> u64 {
    token.text.chars().count().max(1) as u64
}

fn cjk_word_delay_scale_tenths(token: &InputToken) -> u64 {
    match cjk_word_char_count(token) {
        1 if is_common_single_cjk(token) => 9,
        2 => 9,
        4.. => 15,
        _ => 10,
    }
}

fn is_common_single_cjk(token: &InputToken) -> bool {
    token.text.chars().next().is_some_and(|ch| {
        matches!(
            ch,
            '你' | '我' | '他' | '她' | '它' | '的' | '地' | '得' | '了' | '等'
        )
    })
}
