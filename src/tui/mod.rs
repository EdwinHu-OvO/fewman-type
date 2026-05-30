mod config_page;
mod header;
mod page;
mod render;
mod state;
mod theme;
mod view;

use cursive::event::{Event, Key};
use cursive::view::Resizable;
use std::sync::{Arc, Mutex};

pub use state::InputSession;

use page::InputPage;
use state::InputState;

pub fn get_input() -> Option<InputSession> {
    let mut siv = cursive::default();
    siv.set_theme(theme::transparent_theme());

    let state = Arc::new(Mutex::new(InputState::default()));
    siv.add_fullscreen_layer(InputPage::new(Arc::clone(&state)).full_screen());
    bind_global_callbacks(&mut siv, Arc::clone(&state));

    siv.run();
    finish_session(state)
}

fn bind_global_callbacks(siv: &mut cursive::Cursive, state: Arc<Mutex<InputState>>) {
    siv.add_global_callback(Key::Esc, move |s| {
        if let Ok(mut state) = state.lock() {
            state.text.clear();
            state.cancelled = true;
        }
        s.quit();
    });

    siv.add_global_callback(Event::Ctrl(Key::Enter), |s| s.quit());
    siv.add_global_callback(Key::F2, |s| s.quit());
}

fn finish_session(state: Arc<Mutex<InputState>>) -> Option<InputSession> {
    let result = state.lock().map(|state| state.clone()).unwrap_or_default();
    if result.cancelled || result.text.trim().is_empty() {
        None
    } else {
        Some(InputSession {
            text: result.text,
            config: result.config,
        })
    }
}
