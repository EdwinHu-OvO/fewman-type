#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypingConfig {
    pub cjk_segmentation: bool,
    pub base_interval_ms: u64,
    pub skip_word_inner_delay: bool,
    pub typo_simulation: bool,
    pub typo_rate_percent: u8,
}

impl Default for TypingConfig {
    fn default() -> Self {
        Self {
            cjk_segmentation: true,
            base_interval_ms: 50,
            skip_word_inner_delay: false,
            typo_simulation: false,
            typo_rate_percent: 15,
        }
    }
}
