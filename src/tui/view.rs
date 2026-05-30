use super::page::InputPage;
use super::render::{
    FOOTER_LINE, HEADER_HEIGHT, input_y, max_textarea_height, print_hline, print_text,
};
use super::state::Page;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::view::{CannotFocus, View};
use cursive::{Printer, Rect, Vec2};

impl View for InputPage {
    fn draw(&self, printer: &Printer) {
        self.draw_header(printer);

        let top_line_y = HEADER_HEIGHT;
        let input_y = input_y();
        let content_height = self.content_height();
        let bottom_line_y = input_y + content_height;
        let footer_y = bottom_line_y + 1;

        print_hline(printer, top_line_y);
        self.draw_body(printer, input_y);
        print_hline(printer, bottom_line_y);
        print_text(printer, footer_y, FOOTER_LINE);
    }

    fn layout(&mut self, size: Vec2) {
        let max_height = max_textarea_height(size);
        self.body_height = max_height;
        let requested = self.textarea.required_size(Vec2::new(size.x, max_height));
        self.textarea_height = requested.y.clamp(1, max_height);
        self.textarea
            .layout(Vec2::new(size.x, self.textarea_height));
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event == Event::Key(Key::F3) {
            self.toggle_page();
            return EventResult::consumed();
        }

        match self.page {
            Page::Input => {
                let result = self.textarea.on_event(event);
                self.mirror_content();
                result
            }
            Page::Config => self.handle_config_event(event),
        }
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
        match self.page {
            Page::Input => self.textarea.take_focus(source),
            Page::Config => Ok(EventResult::consumed()),
        }
    }

    fn important_area(&self, _: Vec2) -> Rect {
        Rect::from_size((0, input_y()), (1, self.textarea_height))
    }
}

impl InputPage {
    fn content_height(&self) -> usize {
        match self.page {
            Page::Input => self.textarea_height,
            Page::Config => self.body_height,
        }
    }

    fn draw_body(&self, printer: &Printer, y: usize) {
        match self.page {
            Page::Input => self.draw_textarea(printer, y),
            Page::Config => self.draw_config_page(printer, y, self.body_height),
        }
    }

    fn draw_textarea(&self, printer: &Printer, y: usize) {
        if y >= printer.size.y {
            return;
        }
        let height = self.textarea_height.min(printer.size.y - y);
        if height == 0 {
            return;
        }
        let printer = printer
            .offset((0, y))
            .cropped(Vec2::new(printer.size.x, height))
            .focused(printer.focused);
        self.textarea.draw(&printer);
    }

    fn toggle_page(&mut self) {
        self.page = match self.page {
            Page::Input => Page::Config,
            Page::Config => Page::Input,
        };
    }
}
