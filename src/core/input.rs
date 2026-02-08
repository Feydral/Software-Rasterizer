use minifb::Key;
use std::collections::HashSet;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Default)]
struct Input {
    keys_pressed: HashSet<Key>,
    keys_down: HashSet<Key>,
    keys_up: HashSet<Key>,
    prev_pressed: HashSet<Key>,
}

static INPUT: Lazy<Mutex<Input>> = Lazy::new(|| { Mutex::new(Input::default()) });

pub fn update(window: &minifb::Window) {
    let mut input = INPUT.lock().unwrap();

    input.prev_pressed = input.keys_pressed.clone();
    input.keys_pressed.clear();

    for key in ALL_KEYS {
        if window.is_key_down(*key) {
            input.keys_pressed.insert(*key);
        }
    }

    input.keys_down = &input.keys_pressed - &input.prev_pressed;
    input.keys_up   = &input.prev_pressed - &input.keys_pressed;
}

#[allow(dead_code)]
pub fn is_pressed(key: Key) -> bool {
    INPUT.lock().unwrap().keys_pressed.contains(&key)
}

#[allow(dead_code)]
pub fn is_down(key: Key) -> bool {
    INPUT.lock().unwrap().keys_down.contains(&key)
}

#[allow(dead_code)]
pub fn is_up(key: Key) -> bool {
    INPUT.lock().unwrap().keys_up.contains(&key)
}

static ALL_KEYS: &[Key] = &[
    Key::A, Key::B, Key::C, Key::D, Key::E, Key::F, Key::G, Key::H, Key::I, Key::J, Key::K, Key::L,
    Key::M, Key::N, Key::O, Key::P, Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X,
    Key::Y, Key::Z,
    Key::Key0, Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::Key8,
    Key::Key9,
    Key::Escape, Key::Space, Key::Enter, Key::Backspace, Key::Tab, Key::Left, Key::Right,
    Key::Up, Key::Down, Key::LeftShift, Key::RightShift, Key::LeftCtrl, Key::RightCtrl,
    Key::LeftAlt, Key::RightAlt,
];