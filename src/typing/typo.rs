use super::dictionary::typo_candidate_for_word;
use super::token::{InputToken, TokenKind};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TypoPlan {
    pub(crate) wrong_text: String,
    pub(crate) backspaces: usize,
    pub(crate) retype_text: String,
}

pub(crate) fn plan_for_token(token: &InputToken, salt: usize) -> Option<TypoPlan> {
    if token.kind != TokenKind::CjkWord {
        return None;
    }

    let correct_chars: Vec<char> = token.text.chars().collect();
    let n = correct_chars.len();
    if n <= 1 {
        return None;
    }

    let candidate = typo_candidate_for_word(&token.text, salt)?;
    let wrong_chars: Vec<char> = candidate.chars().collect();

    let keep_chars = correct_chars
        .iter()
        .zip(wrong_chars.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let backspaces = wrong_chars.len().saturating_sub(keep_chars);
    if backspaces == 0 {
        return None;
    }

    let retype_text: String = correct_chars[keep_chars..].iter().collect();
    if retype_text.is_empty() {
        return None;
    }

    Some(TypoPlan {
        wrong_text: candidate,
        backspaces,
        retype_text,
    })
}
