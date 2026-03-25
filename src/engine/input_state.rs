use std::collections::HashSet;

use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

pub struct InputState {
    // frame-based
    pub keys_pressed: Vec<KeyEvent>,

    // state-based
    pub keys_down: HashSet<KeyCode>,

    pub mouse_position: (u16, u16),
    pub mouse_buttons_pressed: Vec<MouseButton>,
    pub mouse_moved: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_pressed: Vec::new(),
            keys_down: HashSet::new(),
            mouse_position: (0, 0),
            mouse_buttons_pressed: Vec::new(),
            mouse_moved: false,
        }
    }

    pub fn is_key_down(&self, code: KeyCode) -> bool {
        self.keys_down.contains(&code)
    }

    pub fn is_key_pressed(&self, code: KeyCode) -> bool {
        self.keys_pressed.iter().any(|k| k.code == code)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn update_mouse(&mut self, event: MouseEvent) {
        self.mouse_position = (event.column, event.row);
        self.mouse_moved = false;

        match event.kind {
            MouseEventKind::Down(button) => {
                if !self.mouse_buttons_pressed.contains(&button) {
                    self.mouse_buttons_pressed.push(button);
                }
            }
            MouseEventKind::Up(button) => {
                self.mouse_buttons_pressed.retain(|b| *b != button);
            }
            MouseEventKind::Moved => {
                self.mouse_moved = true;
            }
            MouseEventKind::Drag(button) => {
                self.mouse_moved = true;
                if !self.mouse_buttons_pressed.contains(&button) {
                    self.mouse_buttons_pressed.push(button);
                }
            }
            _ => {}
        }
    }

    pub fn clear_frame_state(&mut self) {
        self.keys_pressed.clear();
        self.mouse_moved = false;
    }

    pub fn is_mouse_over(&self, x: u16, y: u16, w: u16, h: u16) -> bool {
        let (mx, my) = self.mouse_position;
        mx >= x && mx < x + w && my >= y && my < y + h
    }
}
