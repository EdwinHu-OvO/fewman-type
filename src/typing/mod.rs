mod char_class;
mod common_words;
mod config;
mod dictionary;
mod dictionary_typo;
mod engine;
mod frequency;
mod input_plan;
mod timing;
mod token;
mod tokenizer;
mod trie;
mod typo;
mod word_files;
mod yaml_words;

pub use config::{TypingConfig, TypingPreset};
pub use dictionary::dictionary_sources;
pub use engine::type_text;

#[cfg(test)]
mod input_plan_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod typo_tests;
