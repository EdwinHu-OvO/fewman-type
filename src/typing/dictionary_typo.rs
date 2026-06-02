use super::char_class::stable_jitter_text;

pub(crate) fn build_typo_prefixes<'a>(
    words: impl Iterator<Item = &'a String>,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut prefixes: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for word in words {
        let chars: Vec<char> = word.chars().collect();
        for len in 1..chars.len() {
            let prefix: String = chars[0..len].iter().collect();
            prefixes.entry(prefix).or_default().push(word.clone());
        }
    }

    // Sort to ensure determinism across different runs and dictionary load orders
    for list in prefixes.values_mut() {
        list.sort();
    }

    prefixes
}

pub(crate) fn typo_candidate_for_word(
    word: &str,
    salt: usize,
    dictionary_prefixes: &std::collections::HashMap<String, Vec<String>>,
) -> Option<String> {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    if n <= 1 {
        return None;
    }

    for len in 1..n {
        let prefix: String = chars[0..len].iter().collect();
        if let Some(candidates) = dictionary_prefixes.get(&prefix) {
            let valid_candidates: Vec<&String> = candidates
                .iter()
                .filter(|c| {
                    let c_chars: Vec<char> = c.chars().collect();
                    let common_len = chars
                        .iter()
                        .zip(c_chars.iter())
                        .take_while(|(a, b)| a == b)
                        .count();
                    common_len == len && c.as_str() != word
                })
                .collect();

            if !valid_candidates.is_empty() {
                let choice = (stable_jitter_text(word, salt) as usize) % valid_candidates.len();
                return Some(valid_candidates[choice].clone());
            }
        }
    }

    None
}
