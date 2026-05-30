use super::page::InputPage;
use super::render::{accent, muted, print_text};
use super::state::Page;
use cursive::Printer;
use cursive::event::{Event, EventResult, Key};

impl InputPage {
    pub(super) fn draw_config_page(&self, printer: &Printer, y: usize, height: usize) {
        let lines = [
            " 配置页",
            " 使用 ↑/↓ 选择配置项，←/→ 调整数值，Space 切换开关，F3 返回输入页。",
            "",
            "中文拆词模式",
            "输入间隔",
            "实验：关闭词组内间隔",
        ];
        let values = self.config_values();

        for (offset, label) in lines.iter().enumerate().take(height) {
            let row = y + offset;
            if offset < 3 {
                draw_config_intro(printer, row, offset, label);
                continue;
            }
            self.draw_config_item(printer, row, offset - 3, label, &values[offset]);
        }
    }

    pub(super) fn handle_config_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::F3) => {
                self.page = Page::Input;
                EventResult::consumed()
            }
            Event::Key(Key::Up) => {
                self.selected_config = self.selected_config.saturating_sub(1);
                EventResult::consumed()
            }
            Event::Key(Key::Down) => {
                self.selected_config = (self.selected_config + 1).min(2);
                EventResult::consumed()
            }
            Event::Key(Key::Left) => self.adjust_interval(-5),
            Event::Key(Key::Right) => self.adjust_interval(5),
            Event::Char(' ') => self.toggle_selected_config(),
            _ => EventResult::Ignored,
        }
    }

    fn config_values(&self) -> [String; 6] {
        let split = if self.config.cjk_segmentation {
            "开启"
        } else {
            "关闭"
        };
        let inner = if self.config.skip_word_inner_delay {
            "开启"
        } else {
            "关闭"
        };
        [
            String::new(),
            String::new(),
            String::new(),
            format!("[{split}]"),
            format!("{} ms", self.config.base_interval_ms),
            format!("[{inner}]"),
        ]
    }

    fn draw_config_item(
        &self,
        printer: &Printer,
        row: usize,
        index: usize,
        label: &str,
        value: &str,
    ) {
        let selected = self.selected_config == index;
        let marker = if selected { ">" } else { " " };
        let text = format!(" {marker} {label:<12} {value}");
        if selected {
            printer.with_color(accent(), |printer| print_text(printer, row, &text));
        } else {
            print_text(printer, row, &text);
        }
    }

    fn adjust_interval(&mut self, delta: i64) -> EventResult {
        if self.selected_config == 1 {
            let value = (self.config.base_interval_ms as i64 + delta).clamp(10, 1000);
            self.config.base_interval_ms = value as u64;
            self.mirror_config();
        }
        EventResult::consumed()
    }

    fn toggle_selected_config(&mut self) -> EventResult {
        match self.selected_config {
            0 => self.config.cjk_segmentation = !self.config.cjk_segmentation,
            2 => self.config.skip_word_inner_delay = !self.config.skip_word_inner_delay,
            _ => return EventResult::consumed(),
        }
        self.mirror_config();
        EventResult::consumed()
    }
}

fn draw_config_intro(printer: &Printer, row: usize, offset: usize, label: &str) {
    let color = if offset == 0 { accent() } else { muted() };
    printer.with_color(color, |printer| print_text(printer, row, label));
}
