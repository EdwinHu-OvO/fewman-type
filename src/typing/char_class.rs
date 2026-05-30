pub(crate) fn is_ascii_word(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '\''
}

pub(crate) fn is_cjk(ch: char) -> bool {
    matches!(
        ch,
        '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{20000}'..='\u{2A6DF}'
            | '\u{2A700}'..='\u{2B73F}'
            | '\u{2B740}'..='\u{2B81F}'
            | '\u{2B820}'..='\u{2CEAF}'
    )
}

pub(crate) fn take_while(chars: &[char], start: usize, predicate: impl Fn(char) -> bool) -> usize {
    let mut end = start;
    while end < chars.len() && predicate(chars[end]) {
        end += 1;
    }
    end
}

pub(crate) fn stable_jitter(index: usize, ch: char) -> u64 {
    stable_jitter_num(index).wrapping_add(ch as u64)
}

pub(crate) fn stable_jitter_num(value: usize) -> u64 {
    (value as u64)
        .wrapping_mul(1_103_515_245)
        .wrapping_add(12_345)
}

pub(crate) fn stable_jitter_text(text: &str, salt: usize) -> u64 {
    text.chars().fold(stable_jitter_num(salt), |acc, ch| {
        acc.wrapping_mul(31).wrapping_add(ch as u64)
    })
}
