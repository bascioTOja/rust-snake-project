use crate::engine::{Display, GameObject, InputState};
use crossterm::style::Color;

pub struct ScreenTester {
    time: f32,
}

impl ScreenTester {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    fn rgb_carousel(t: f32) -> Color {
        let t = t.fract(); // ensure 0 <= t < 1
        let h = t * 6.0; // map to 0..6
        let i = h.floor() as u32;
        let f = h - i as f32;
        let q = 1.0 - f;

        let c = match i % 6 {
            0 => (1.0, f, 0.0),
            1 => (q, 1.0, 0.0),
            2 => (0.0, 1.0, f),
            3 => (0.0, q, 1.0),
            4 => (f, 0.0, 1.0),
            5 => (1.0, 0.0, q),
            _ => (1.0, 0.0, 0.0), // fallback
        };

        Color::Rgb {
            r: (c.0 * 255.0) as u8,
            g: (c.1 * 255.0) as u8,
            b: (c.2 * 255.0) as u8,
        }
    }
}

impl GameObject for ScreenTester {
    fn update(&mut self, dt: f32, input_events: &InputState, display: &mut Display) {
        self.time += dt / 10.0;
        for y in 0..display.height {
            for x in 0..display.width {
                let t1: f32 = (x as f32 + (y * 2) as f32) / display.width as f32;
                let t2: f32 = (x as f32 + ((y * 2) + 1) as f32) / display.width as f32;
                let c1 = ScreenTester::rgb_carousel(t1 + self.time);
                let c2 = ScreenTester::rgb_carousel(t2 + self.time);
                display.insert_raw_str_fg_bg("▄", x, y, c2, c1);
                // display.set_px_bg(x, y, color);
            }
        }
    }
}
