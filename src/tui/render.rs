use cursive::theme::{Color, ColorStyle};
use cursive::{Printer, Vec2};

pub(super) const HEADER_HEIGHT: usize = 12;
pub(super) const INPUT_FOOTER_LINE: &str = " F3 配置  |  Ctrl+Enter / F2 确认  |  Esc 退出 ";
pub(super) const CONFIG_FOOTER_LINE: &str =
    " ↑↓ 选择  |  ←→ 调整设置  |  F3 / Esc 返回  |  Ctrl+Enter / F2 确认 ";
pub(super) const FIXED_HEIGHT: usize = HEADER_HEIGHT + 3;

pub(super) const LOGO_LINES: [&str; 8] = [
    r"  ______                               _______               ",
    r" |  ____|                             |__   __|              ",
    r" | |__ _____      ___ __ ___   __ _ _ __ | |_   _ _ __   ___ ",
    r" |  __/ _ \ \ /\ / / '_ ` _ \ / _` | '_ \| | | | | '_ \ / _ \",
    r" | | |  __/\ V  V /| | | | | | (_| | | | | | |_| | |_) |  __/",
    r" |_|  \___| \_/\_/ |_| |_| |_|\__,_|_| |_|_|\__, | .__/ \___|",
    r"                                             __/ | |         ",
    r"                                            |___/|_|         ",
];

pub(super) fn input_y() -> usize {
    HEADER_HEIGHT + 1
}

pub(super) fn max_textarea_height(size: Vec2) -> usize {
    size.y.saturating_sub(FIXED_HEIGHT).max(1)
}

pub(super) fn print_hline(printer: &Printer, y: usize) {
    if y < printer.size.y {
        printer.print_hline((0, y), printer.size.x, "─");
    }
}

pub(super) fn print_text(printer: &Printer, y: usize, text: &str) {
    if y < printer.size.y {
        printer.print((1, y), text);
    }
}

pub(super) fn accent() -> ColorStyle {
    ColorStyle::front(Color::Rgb(210, 126, 87))
}

pub(super) fn muted() -> ColorStyle {
    ColorStyle::front(Color::Rgb(150, 150, 150))
}
