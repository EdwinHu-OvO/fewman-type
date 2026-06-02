mod char_class;
mod common_words;
mod config;
mod dictionary;
mod dictionary_typo;
mod engine;
mod frequency;
mod timing;
mod token;
mod tokenizer;
mod trie;
mod typo;
mod word_files;
mod yaml_words;

pub use config::TypingConfig;
pub use dictionary::dictionary_sources;
pub use engine::type_text;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod typo_tests;
