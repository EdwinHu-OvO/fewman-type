#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Word,
    CjkWord,
    Whitespace,
    Punctuation,
    Newline,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputToken {
    pub text: String,
    pub kind: TokenKind,
}

impl InputToken {
    pub(crate) fn new(text: impl Into<String>, kind: TokenKind) -> Self {
        Self {
            text: text.into(),
            kind,
        }
    }
}
