use super::TypingConfig;
use super::frequency::frequency_scale_per_mille;
use super::timing::{cjk_word_delay_budget_ms, delay_after, delay_before, delay_inside};
use super::token::{InputToken, TokenKind};
use super::tokenizer::tokenize_with_config;
use super::yaml_words::parse_yaml_word_entries;

fn tokenize(text: &str) -> Vec<InputToken> {
    tokenize_with_config(text, TypingConfig::default())
}

#[test]
fn normalizes_windows_newlines() {
    let tokens = tokenize("第一行\r\n第二行");
    assert_eq!(
        tokens
            .iter()
            .filter(|token| token.kind == TokenKind::Newline)
            .count(),
        1
    );
}

#[test]
fn segments_common_chinese_words() {
    let tokens = tokenize("中文自动输入器需要拆词");
    let words: Vec<_> = tokens.into_iter().map(|token| token.text).collect();
    assert_eq!(words, vec!["中文", "自动输入器", "需要", "拆词"]);
}

#[test]
fn segments_jieba_builtin_high_frequency_words() {
    let tokens = tokenize("我们这个中国");
    let words: Vec<_> = tokens.into_iter().map(|token| token.text).collect();
    assert_eq!(words, vec!["我们", "这个", "中国"]);
}

#[test]
fn trie_prefers_the_longest_overlapping_word() {
    let tokens = tokenize("自动输入器");
    let words: Vec<_> = tokens.into_iter().map(|token| token.text).collect();
    assert_eq!(words, vec!["自动输入器"]);
}

#[test]
fn keeps_ascii_words_together() {
    let tokens = tokenize("AutoTyper v1 测试");
    let words: Vec<_> = tokens.into_iter().map(|token| token.text).collect();
    assert_eq!(words, vec!["AutoTyper", " ", "v1", " ", "测试"]);
}

#[test]
fn can_disable_chinese_segmentation() {
    let tokens = tokenize_with_config(
        "中文",
        TypingConfig {
            cjk_segmentation: false,
            base_interval_ms: 50,
            skip_word_inner_delay: false,
            typo_simulation: false,
        },
    );
    let words: Vec<_> = tokens.into_iter().map(|token| token.text).collect();
    assert_eq!(words, vec!["中", "文"]);
}

#[test]
fn cjk_word_total_delay_matches_budget_before_the_word() {
    let token = InputToken::new("自动输入器", TokenKind::CjkWord);
    let config = TypingConfig {
        cjk_segmentation: true,
        base_interval_ms: 50,
        skip_word_inner_delay: false,
        typo_simulation: false,
    };
    let char_count = token.text.chars().count() as u128;
    let inner_total: u128 = (0..char_count.saturating_sub(1))
        .map(|offset| delay_inside(&token, offset as usize, config).as_millis())
        .sum();
    let before = delay_before(&token, config).as_millis();
    let after = delay_after(&token, 0, config).as_millis();
    assert_eq!(before + inner_total + after, char_count * 50 * 15 / 10);
    assert_eq!(after, 0);
}

#[test]
fn can_skip_cjk_word_inner_delay() {
    let token = InputToken::new("自动输入器", TokenKind::CjkWord);
    let config = TypingConfig {
        cjk_segmentation: true,
        base_interval_ms: 50,
        skip_word_inner_delay: true,
        typo_simulation: false,
    };
    let char_count = token.text.chars().count() as u128;
    assert_eq!(delay_inside(&token, 0, config), std::time::Duration::ZERO);
    assert_eq!(
        delay_before(&token, config).as_millis(),
        char_count * 50 * 15 / 10
    );
    assert_eq!(delay_after(&token, 0, config).as_millis(), 0);
}

#[test]
fn cjk_word_delay_uses_length_scales() {
    let config = TypingConfig {
        cjk_segmentation: true,
        base_interval_ms: 100,
        skip_word_inner_delay: false,
        typo_simulation: false,
    };
    assert_eq!(
        cjk_word_delay_budget_ms(&InputToken::new("龘", TokenKind::CjkWord), config),
        100
    );
    assert_eq!(
        cjk_word_delay_budget_ms(&InputToken::new("龘龘", TokenKind::CjkWord), config),
        180
    );
    assert_eq!(
        cjk_word_delay_budget_ms(&InputToken::new("龘龘龘", TokenKind::CjkWord), config),
        300
    );
    assert_eq!(
        cjk_word_delay_budget_ms(&InputToken::new("龘龘龘龘龘", TokenKind::CjkWord), config),
        750
    );
}

#[test]
fn parses_yaml_words_with_optional_frequency() {
    let entries = parse_yaml_word_entries(
        r#"words:
  - "旧格式"
  - text: "高频词"
    frequency: 12345
"#,
    );
    assert_eq!(entries[0].0, "旧格式");
    assert_eq!(entries[0].1.frequency, None);
    assert_eq!(entries[1].0, "高频词");
    assert_eq!(entries[1].1.frequency, Some(12345));
}

#[test]
fn frequency_scale_never_speeds_past_base_offset() {
    assert_eq!(frequency_scale_per_mille(1_000_000, 1, 1_000_000), 1000);
    assert_eq!(frequency_scale_per_mille(1, 1, 1_000_000), 1600);
    assert!(frequency_scale_per_mille(1_000, 1, 1_000_000) > 1000);
}
