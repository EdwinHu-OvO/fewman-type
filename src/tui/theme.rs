use cursive::theme::{BorderStyle, Color, PaletteColor, PaletteStyle, Style, Theme};

pub(super) fn transparent_theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = false;
    theme.borders = BorderStyle::None;

    for color in PaletteColor::all() {
        theme.palette[color] = Color::TerminalDefault;
    }

    for style in PaletteStyle::all() {
        theme.palette[style] = Style::terminal_default();
    }

    theme
}
