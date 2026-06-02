use super::page::InputPage;
use super::render::{LOGO_LINES, accent, muted, print_text};
use super::state::Page;
use cursive::Printer;

impl InputPage {
    pub(super) fn draw_header(&self, printer: &Printer) {
        let width = printer.size.x;
        if width < 4 || printer.size.y == 0 {
            return;
        }

        let right = width - 1;
        let bottom = super::render::HEADER_HEIGHT.saturating_sub(1);
        let divider_x = if width >= 96 { 44 } else { width / 2 };
        draw_frame(printer, right, bottom, divider_x);
        self.draw_header_left(printer, bottom);
        self.draw_header_right(printer, right, bottom, divider_x);
    }

    fn draw_header_left(&self, printer: &Printer, bottom: usize) {
        for (i, line) in LOGO_LINES.iter().enumerate() {
            let y = 2 + i;
            if y >= bottom {
                break;
            }
            printer.with_color(accent(), |printer| print_text(printer, y, line));
        }

        print_text(printer, 6, "  Ready to type like paste is a keyboard.");
        printer.with_color(muted(), |printer| {
            print_text(
                printer,
                8,
                "  粘贴文本到下方，确认后移动光标并按 Ctrl+V 开始。",
            );
            print_text(printer, 9, "  F3 配置，Esc 退出，Ctrl+Enter / F2 提交。");
        });
    }

    fn draw_header_right(&self, printer: &Printer, right: usize, bottom: usize, divider_x: usize) {
        if divider_x + 3 >= right {
            return;
        }

        let x = divider_x + 3;
        for (i, line) in self.config_preview_lines().iter().enumerate() {
            let y = 2 + i;
            if y >= bottom {
                break;
            }
            let color = if i == 0 { accent() } else { muted() };
            printer.with_color(color, |printer| printer.print((x, y), line));
        }
    }

    fn config_preview_lines(&self) -> [String; 9] {
        let mode = match self.page {
            Page::Input => "输入页",
            Page::Config => "配置页",
        };
        let split = if self.config.cjk_segmentation {
            "开启"
        } else {
            "关闭"
        };
        let inner = if self.config.skip_word_inner_delay {
            "关闭"
        } else {
            "开启"
        };
        [
            "配置预览".to_string(),
            format!("当前页    {mode}"),
            format!("输入间隔  {} ms", self.config.base_interval_ms),
            format!("中文拆词  {split}"),
            format!("词内间隔  {inner}"),
            self.dictionary_summary_line(),
            self.dictionary_detail_line(0),
            self.dictionary_detail_line(1),
            "触发键    Ctrl+V".to_string(),
        ]
    }
}

fn draw_frame(printer: &Printer, right: usize, bottom: usize, divider_x: usize) {
    printer.with_color(accent(), |printer| {
        printer.print((0, 0), "╭");
        printer.print_hline((1, 0), printer.size.x.saturating_sub(2), "─");
        printer.print((right, 0), "╮");
        printer.print((2, 0), " FewmanType ");

        for y in 1..bottom {
            printer.print((0, y), "│");
            printer.print((right, y), "│");
        }
        if divider_x > 8 && divider_x + 2 < right {
            printer.print_vline((divider_x, 1), bottom.saturating_sub(1), "│");
        }

        printer.print((0, bottom), "╰");
        printer.print_hline((1, bottom), printer.size.x.saturating_sub(2), "─");
        printer.print((right, bottom), "╯");
    });
}
