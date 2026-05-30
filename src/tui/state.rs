use crate::typing::TypingConfig;

#[derive(Clone, Debug)]
pub struct InputSession {
    pub text: String,
    pub config: TypingConfig,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Page {
    Input,
    Config,
}

#[derive(Clone, Debug, Default)]
pub(super) struct InputState {
    pub text: String,
    pub config: TypingConfig,
    pub cancelled: bool,
}
