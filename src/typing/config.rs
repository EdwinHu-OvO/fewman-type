#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypingPreset {
    Fast,
    Human,
    Custom,
}

impl TypingPreset {
    pub fn label(self) -> &'static str {
        match self {
            Self::Fast => "急速",
            Self::Human => "一键拟人",
            Self::Custom => "自定义",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Fast => Self::Human,
            Self::Human => Self::Custom,
            Self::Custom => Self::Fast,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Fast => Self::Custom,
            Self::Human => Self::Fast,
            Self::Custom => Self::Human,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypingConfig {
    pub preset: TypingPreset,
    pub cjk_segmentation: bool,
    pub pair_matching: bool,
    pub base_interval_ms: u64,
    pub skip_word_inner_delay: bool,
    pub typo_simulation: bool,
    pub typo_rate_percent: u8,
}

impl TypingConfig {
    pub fn with_preset(preset: TypingPreset) -> Self {
        match preset {
            TypingPreset::Fast => Self {
                preset,
                cjk_segmentation: false,
                pair_matching: false,
                base_interval_ms: 50,
                skip_word_inner_delay: false,
                typo_simulation: false,
                typo_rate_percent: 0,
            },
            TypingPreset::Human => Self {
                preset,
                cjk_segmentation: true,
                pair_matching: true,
                base_interval_ms: 350,
                skip_word_inner_delay: true,
                typo_simulation: true,
                typo_rate_percent: 15,
            },
            TypingPreset::Custom => {
                let mut config = Self::with_preset(TypingPreset::Human);
                config.preset = TypingPreset::Custom;
                config
            }
        }
    }

    pub fn mark_custom(&mut self) {
        self.preset = TypingPreset::Custom;
    }
}

impl Default for TypingConfig {
    fn default() -> Self {
        Self::with_preset(TypingPreset::Human)
    }
}
