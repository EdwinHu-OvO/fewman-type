use super::page::InputPage;
use super::render::{accent, muted, print_text};
use super::state::Page;
use crate::typing::{TypingConfig, TypingPreset};
use cursive::Printer;
use cursive::event::{Event, EventResult, Key};
const CONFIG_ROWS: [(&str, &str); 7] = [
    ("预设模式", "←/→ 切换预设"),
    ("中文拆词", "←/→ 切换中文词组节奏"),
    ("符号匹配", "←/→ 切换成对符号智能输入"),
    ("输入间隔", "←/→ 调整基础间隔"),
    ("词内间隔", "←/→ 切换词组内部停顿"),
    ("错字模拟", "←/→ 切换模拟打错再修正"),
    ("错字率", "←/→ 调整触发概率"),
];

impl InputPage {
    pub(super) fn draw_config_page(&self, printer: &Printer, y: usize, height: usize) {
        let intro = [
            " 配置",
            " 使用 ↑/↓ 选择，←/→ 调整设置，F3 / Esc 返回输入页。",
            "",
        ];
        for (offset, line) in intro.iter().enumerate().take(height) {
            draw_config_intro(printer, y + offset, offset, line);
        }

        let values = self.config_values();
        for (index, (label, hint)) in CONFIG_ROWS.iter().enumerate() {
            let offset = index + intro.len();
            if offset >= height {
                break;
            }
            self.draw_config_item(printer, y + offset, index, label, &values[index], hint);
        }
    }

    pub(super) fn handle_config_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::F3) | Event::Key(Key::Esc) => {
                self.page = Page::Input;
                EventResult::consumed()
            }
            Event::Key(Key::Up) => {
                self.selected_config = self.selected_config.saturating_sub(1);
                EventResult::consumed()
            }
            Event::Key(Key::Down) => {
                self.selected_config = (self.selected_config + 1).min(CONFIG_ROWS.len() - 1);
                EventResult::consumed()
            }
            Event::Key(Key::Left) => self.adjust_selected_value(-5),
            Event::Key(Key::Right) => self.adjust_selected_value(5),
            _ => EventResult::Ignored,
        }
    }

    fn config_values(&self) -> [String; 7] {
        [
            format!("[{}]", self.config.preset.label()),
            format!("[{}]", switch_label(self.config.cjk_segmentation)),
            format!("[{}]", switch_label(self.config.pair_matching)),
            format!("{} ms", self.config.base_interval_ms),
            format!("[{}]", switch_label(!self.config.skip_word_inner_delay)),
            format!("[{}]", switch_label(self.config.typo_simulation)),
            format!("{}%", self.config.typo_rate_percent),
        ]
    }

    fn draw_config_item(
        &self,
        printer: &Printer,
        row: usize,
        index: usize,
        label: &str,
        value: &str,
        hint: &str,
    ) {
        let selected = self.selected_config == index;
        let marker = if selected { ">" } else { " " };
        let text = format!(" {marker} {label:<10} {value:<8} {hint}");
        if selected {
            printer.with_color(accent(), |printer| print_text(printer, row, &text));
        } else {
            print_text(printer, row, &text);
        }
    }

    fn adjust_selected_value(&mut self, delta: i64) -> EventResult {
        match self.selected_config {
            0 => self.apply_preset(shift_preset(self.config.preset, delta)),
            1 => self.config.cjk_segmentation = !self.config.cjk_segmentation,
            2 => self.config.pair_matching = !self.config.pair_matching,
            3 => {
                let value = (self.config.base_interval_ms as i64 + delta).clamp(10, 1000);
                self.config.base_interval_ms = value as u64;
            }
            4 => self.config.skip_word_inner_delay = !self.config.skip_word_inner_delay,
            5 => self.config.typo_simulation = !self.config.typo_simulation,
            6 => {
                let value = (self.config.typo_rate_percent as i64 + delta).clamp(0, 100);
                self.config.typo_rate_percent = value as u8;
            }
            _ => {}
        }

        if self.selected_config != 0 {
            self.config.mark_custom();
        }
        self.mirror_config();

        EventResult::consumed()
    }

    fn apply_preset(&mut self, preset: TypingPreset) {
        match preset {
            TypingPreset::Fast | TypingPreset::Human => {
                self.config = TypingConfig::with_preset(preset);
            }
            TypingPreset::Custom => self.config.mark_custom(),
        }
        self.mirror_config();
    }
}

fn shift_preset(preset: TypingPreset, delta: i64) -> TypingPreset {
    if delta < 0 {
        preset.previous()
    } else {
        preset.next()
    }
}

fn switch_label(enabled: bool) -> &'static str {
    if enabled { "开启" } else { "关闭" }
}

fn draw_config_intro(printer: &Printer, row: usize, offset: usize, label: &str) {
    let color = if offset == 0 { accent() } else { muted() };
    printer.with_color(color, |printer| print_text(printer, row, label));
}
