use super::char_class::{is_ascii_word, is_cjk, stable_jitter, take_while};
use super::config::TypingConfig;
use super::dictionary::longest_match;
use super::token::{InputToken, TokenKind};

pub fn tokenize_with_config(text: &str, config: TypingConfig) -> Vec<InputToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        index = read_next_token(&chars, index, config, &mut tokens);
    }

    tokens
}

fn read_next_token(
    chars: &[char],
    index: usize,
    config: TypingConfig,
    tokens: &mut Vec<InputToken>,
) -> usize {
    let ch = chars[index];
    if ch == '\r' {
        tokens.push(InputToken::new("\n", TokenKind::Newline));
        index + 1 + usize::from(chars.get(index + 1) == Some(&'\n'))
    } else if ch == '\n' {
        tokens.push(InputToken::new("\n", TokenKind::Newline));
        index + 1
    } else if is_cjk(ch) {
        let end = take_while(chars, index, is_cjk);
        segment_cjk(&chars[index..end], config.cjk_segmentation, tokens);
        end
    } else if is_ascii_word(ch) {
        collect_run(chars, index, is_ascii_word, TokenKind::Word, tokens)
    } else if ch.is_whitespace() {
        collect_run(
            chars,
            index,
            |ch| ch.is_whitespace() && ch != '\n' && ch != '\r',
            TokenKind::Whitespace,
            tokens,
        )
    } else {
        tokens.push(InputToken::new(ch.to_string(), TokenKind::Punctuation));
        index + 1
    }
}

fn collect_run(
    chars: &[char],
    index: usize,
    predicate: impl Fn(char) -> bool,
    kind: TokenKind,
    tokens: &mut Vec<InputToken>,
) -> usize {
    let end = take_while(chars, index, predicate);
    let text: String = chars[index..end].iter().collect();
    tokens.push(InputToken::new(text, kind));
    end
}

fn segment_cjk(chars: &[char], enabled: bool, tokens: &mut Vec<InputToken>) {
    let mut index = 0;
    while index < chars.len() {
        let length = if enabled {
            longest_match(chars, index).unwrap_or_else(|| fallback_cjk_len(chars, index))
        } else {
            1
        };
        let text: String = chars[index..index + length].iter().collect();
        tokens.push(InputToken::new(text, TokenKind::CjkWord));
        index += length;
    }
}

fn fallback_cjk_len(chars: &[char], index: usize) -> usize {
    let remaining = chars.len() - index;
    if remaining <= 2 {
        remaining
    } else if remaining == 3 {
        2
    } else {
        2 + usize::from(stable_jitter(index, chars[index]) % 2 == 0)
    }
}
