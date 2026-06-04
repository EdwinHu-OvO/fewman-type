use super::TypingConfig;
use super::input_plan::{InputAction, plan_input_actions};
use super::token::TokenKind;
use super::tokenizer::tokenize_with_config;

fn action_labels(text: &str) -> Vec<String> {
    let tokens = tokenize_with_config(text, TypingConfig::default());
    plan_input_actions(tokens, true)
        .iter()
        .map(action_label)
        .collect()
}

fn action_label(action: &InputAction) -> String {
    match action {
        InputAction::Token { token, token_index } => format!("{token_index}:{}", token.text),
        InputAction::MoveLeft => "←".to_string(),
        InputAction::MoveRight => "→".to_string(),
    }
}

#[test]
fn plans_simple_parentheses() {
    assert_eq!(
        action_labels("a(b)c"),
        vec!["0:a", "1:(", "3:)", "←", "2:b", "→", "4:c"]
    );
}

#[test]
fn plans_angle_brackets() {
    assert_eq!(
        action_labels("a<b>c"),
        vec!["0:a", "1:<", "3:>", "←", "2:b", "→", "4:c"]
    );
}

#[test]
fn can_disable_pair_matching() {
    let tokens = tokenize_with_config("a(b)c", TypingConfig::default());
    let actions = plan_input_actions(tokens, false);
    let labels: Vec<_> = actions.iter().map(action_label).collect();
    assert_eq!(labels, vec!["0:a", "1:(", "2:b", "3:)", "4:c"]);
}

#[test]
fn plans_when_closer_is_fiftieth_char_after_opener() {
    let inner = "a".repeat(49);
    assert_eq!(
        action_labels(&format!("({inner})")),
        vec![
            "0:(".to_string(),
            "2:)".to_string(),
            "←".to_string(),
            format!("1:{inner}"),
            "→".to_string(),
        ]
    );
}

#[test]
fn keeps_pair_linear_when_closer_is_after_fifty_chars() {
    let inner = "a".repeat(50);
    assert_eq!(
        action_labels(&format!("({inner})")),
        vec!["0:(".to_string(), format!("1:{inner}"), "2:)".to_string()]
    );
}

#[test]
fn continues_planning_after_unmatched_opener() {
    assert_eq!(
        action_labels("([b]"),
        vec!["0:(", "1:[", "3:]", "←", "2:b", "→"]
    );
}

#[test]
fn plans_nested_pairs() {
    assert_eq!(
        action_labels("a(b[c]d)e"),
        vec![
            "0:a", "1:(", "7:)", "←", "2:b", "3:[", "5:]", "←", "4:c", "→", "6:d", "→", "8:e",
        ]
    );
}

#[test]
fn leaves_unmatched_opener_linear() {
    assert_eq!(action_labels("a(b"), vec!["0:a", "1:(", "2:b"]);
}

#[test]
fn leaves_unmatched_closer_linear() {
    assert_eq!(action_labels("a)b"), vec!["0:a", "1:)", "2:b"]);
}

#[test]
fn treats_unrelated_orphan_closer_as_normal_inside_pair() {
    assert_eq!(
        action_labels("a(b]c)d"),
        vec!["0:a", "1:(", "5:)", "←", "2:b", "3:]", "4:c", "→", "6:d"]
    );
}

#[test]
fn continues_planning_after_crossed_mismatch() {
    assert_eq!(
        action_labels("a([)]b"),
        vec!["0:a", "1:(", "2:[", "4:]", "←", "3:)", "→", "5:b"]
    );
}

#[test]
fn leaves_empty_pair_without_cursor_moves() {
    assert_eq!(action_labels("()"), vec!["0:(", "1:)"]);
}

#[test]
fn plans_full_width_pairs() {
    assert_eq!(
        action_labels("中文（测试）"),
        vec!["0:中文", "1:（", "3:）", "←", "2:测试", "→"]
    );
}

#[test]
fn preserves_cjk_token_inside_pair() {
    let tokens = tokenize_with_config("(自动输入器)", TypingConfig::default());
    let actions = plan_input_actions(tokens, true);
    let Some(InputAction::Token { token, token_index }) = actions.get(3) else {
        panic!("expected inner CJK token action");
    };

    assert_eq!(*token_index, 1);
    assert_eq!(token.text, "自动输入器");
    assert_eq!(token.kind, TokenKind::CjkWord);
}
