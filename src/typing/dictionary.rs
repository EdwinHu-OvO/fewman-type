use super::common_words::builtin_words;
use super::dictionary_typo;
use super::frequency::frequency_scale_per_mille;
use super::trie::WordTrie;
use super::word_files::dictionary_files;
#[cfg(not(test))]
use super::yaml_words::parse_yaml_word_entries;
use std::collections::HashMap;
#[cfg(not(test))]
use std::fs;
use std::sync::OnceLock;

static WORD_DICTIONARY: OnceLock<WordDictionary> = OnceLock::new();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WordEntry {
    pub(crate) frequency: Option<u64>,
}

struct WordDictionary {
    words: HashMap<String, WordEntry>,
    trie: WordTrie,
    typo_prefixes: OnceLock<HashMap<String, Vec<String>>>,
    max_len: usize,
    min_frequency: u64,
    max_frequency: u64,
}

pub fn dictionary_sources() -> Vec<String> {
    dictionary_files()
        .into_iter()
        .filter_map(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .collect()
}

pub(crate) fn longest_match(chars: &[char], index: usize) -> Option<usize> {
    let dictionary = WORD_DICTIONARY.get_or_init(load_word_dictionary);
    let max_len = (chars.len() - index).min(dictionary.max_len);
    dictionary.trie.longest_match(chars, index, max_len)
}

pub(crate) fn word_frequency_scale_per_mille(word: &str) -> u64 {
    let dictionary = WORD_DICTIONARY.get_or_init(load_word_dictionary);
    let Some(frequency) = dictionary.words.get(word).and_then(|entry| entry.frequency) else {
        return 1000;
    };

    frequency_scale_per_mille(
        frequency,
        dictionary.min_frequency,
        dictionary.max_frequency,
    )
}

fn load_word_dictionary() -> WordDictionary {
    let words = load_words();
    let max_len = words
        .keys()
        .map(|word| word.chars().count())
        .max()
        .unwrap_or(5)
        .clamp(2, 12);
    let (min_frequency, max_frequency) =
        frequency_range(words.values().filter_map(|e| e.frequency));
    let trie = WordTrie::from_words(words.keys(), max_len);

    WordDictionary {
        words,
        trie,
        typo_prefixes: OnceLock::new(),
        max_len,
        min_frequency,
        max_frequency,
    }
}

fn frequency_range(mut frequencies: impl Iterator<Item = u64>) -> (u64, u64) {
    let Some(first) = frequencies.next() else {
        return (1, 1);
    };
    frequencies.fold((first, first), |(min, max), frequency| {
        (min.min(frequency), max.max(frequency))
    })
}

fn common_words() -> HashMap<String, WordEntry> {
    builtin_words()
        .map(|(word, frequency)| (word.to_string(), WordEntry { frequency }))
        .collect()
}

#[cfg(test)]
fn load_words() -> HashMap<String, WordEntry> {
    common_words()
}

#[cfg(not(test))]
fn load_words() -> HashMap<String, WordEntry> {
    let mut words = common_words();
    for path in dictionary_files() {
        if let Ok(content) = fs::read_to_string(path) {
            merge_entries(&mut words, parse_yaml_word_entries(&content));
        }
    }
    words
}

pub(crate) fn typo_candidate_for_word(word: &str, salt: usize) -> Option<String> {
    let dictionary = WORD_DICTIONARY.get_or_init(load_word_dictionary);
    let typo_prefixes = dictionary
        .typo_prefixes
        .get_or_init(|| dictionary_typo::build_typo_prefixes(dictionary.words.keys()));
    dictionary_typo::typo_candidate_for_word(word, salt, typo_prefixes)
}

#[cfg(not(test))]
fn merge_entries(words: &mut HashMap<String, WordEntry>, entries: Vec<(String, WordEntry)>) {
    for (word, entry) in entries {
        words
            .entry(word)
            .and_modify(|existing| {
                existing.frequency = match (existing.frequency, entry.frequency) {
                    (Some(current), Some(next)) => Some(current.max(next)),
                    (None, frequency) | (frequency, None) => frequency,
                };
            })
            .or_insert(entry);
    }
}
