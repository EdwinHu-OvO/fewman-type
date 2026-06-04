use super::token::{InputToken, TokenKind};

const MAX_PAIR_LOOKAHEAD_CHARS: usize = 50;

const PAIRS: &[(char, char)] = &[
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('<', '>'),
    ('（', '）'),
    ('【', '】'),
    ('《', '》'),
    ('〈', '〉'),
    ('「', '」'),
    ('『', '』'),
    ('“', '”'),
    ('‘', '’'),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum InputAction {
    Token {
        token: InputToken,
        token_index: usize,
    },
    MoveLeft,
    MoveRight,
}

pub(crate) fn plan_input_actions(tokens: Vec<InputToken>) -> Vec<InputAction> {
    let indexed_tokens: Vec<(usize, InputToken)> = tokens.into_iter().enumerate().collect();
    let mut actions = Vec::new();
    plan_range(&indexed_tokens, 0, indexed_tokens.len(), &mut actions);
    actions
}

fn plan_range(
    tokens: &[(usize, InputToken)],
    start: usize,
    end: usize,
    actions: &mut Vec<InputAction>,
) {
    let mut index = start;
    while index < end {
        if is_opener_at(tokens, index) {
            if let Some(close_index) = find_matching_close(tokens, index, end) {
                push_token_action(tokens, index, actions);
                push_token_action(tokens, close_index, actions);
                if close_index > index + 1 {
                    actions.push(InputAction::MoveLeft);
                    plan_range(tokens, index + 1, close_index, actions);
                    actions.push(InputAction::MoveRight);
                }
                index = close_index + 1;
            } else {
                push_token_action(tokens, index, actions);
                index += 1;
            }
        } else {
            push_token_action(tokens, index, actions);
            index += 1;
        }
    }
}

fn find_matching_close(
    tokens: &[(usize, InputToken)],
    open_index: usize,
    end: usize,
) -> Option<usize> {
    let open = punctuation_char(&tokens[open_index].1)?;
    let mut expected_closers = vec![close_for_open(open)?];
    let mut scanned_chars = 0;

    for (index, (_, token)) in tokens.iter().enumerate().take(end).skip(open_index + 1) {
        scanned_chars += token_char_count(token);
        if scanned_chars > MAX_PAIR_LOOKAHEAD_CHARS {
            return None;
        }

        let Some(ch) = punctuation_char(token) else {
            continue;
        };

        if let Some(close) = close_for_open(ch) {
            expected_closers.push(close);
        } else if expected_closers.last() == Some(&ch) {
            expected_closers.pop();
            if expected_closers.is_empty() {
                return Some(index);
            }
        } else if expected_closers.contains(&ch) {
            return None;
        }
    }

    None
}

fn is_opener_at(tokens: &[(usize, InputToken)], index: usize) -> bool {
    punctuation_char(&tokens[index].1)
        .and_then(close_for_open)
        .is_some()
}

fn close_for_open(ch: char) -> Option<char> {
    PAIRS
        .iter()
        .find_map(|(open, close)| (*open == ch).then_some(*close))
}

fn punctuation_char(token: &InputToken) -> Option<char> {
    if token.kind != TokenKind::Punctuation {
        return None;
    }

    let mut chars = token.text.chars();
    let ch = chars.next()?;
    chars.next().is_none().then_some(ch)
}

fn push_token_action(tokens: &[(usize, InputToken)], index: usize, actions: &mut Vec<InputAction>) {
    let (token_index, token) = &tokens[index];
    actions.push(InputAction::Token {
        token: token.clone(),
        token_index: *token_index,
    });
}

fn token_char_count(token: &InputToken) -> usize {
    token.text.chars().count()
}
