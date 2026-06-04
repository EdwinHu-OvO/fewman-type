use super::state::{InputState, Page};
use crate::typing::{TypingConfig, dictionary_sources};
use cursive::views::TextArea;
use std::sync::{Arc, Mutex};

pub(super) struct InputPage {
    pub(super) textarea: TextArea,
    pub(super) textarea_height: usize,
    pub(super) body_height: usize,
    pub(super) page: Page,
    pub(super) selected_config: usize,
    pub(super) state: Arc<Mutex<InputState>>,
    pub(super) config: TypingConfig,
    pub(super) dictionaries: Vec<String>,
}

impl InputPage {
    pub(super) fn new(state: Arc<Mutex<InputState>>) -> Self {
        Self {
            textarea: TextArea::new(),
            textarea_height: 1,
            body_height: 1,
            page: Page::Input,
            selected_config: 0,
            state,
            config: TypingConfig::default(),
            dictionaries: dictionary_sources(),
        }
    }

    pub(super) fn mirror_content(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.text = self.textarea.get_content().to_string();
        }
    }

    pub(super) fn mirror_config(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.config = self.config;
        }
    }

    pub(super) fn dictionary_summary_line(&self) -> String {
        if self.dictionaries.is_empty() {
            "词库      内置小词表".to_string()
        } else {
            format!("词库      {} 份外挂", self.dictionaries.len())
        }
    }
}
