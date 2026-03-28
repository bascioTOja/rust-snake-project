mod debug;
mod display;
mod fps_smoother;
mod input_state;

mod screen_tester;

use std::time::{Duration, Instant};

pub use debug::{Debug, LogType};
pub use display::Display;
pub use fps_smoother::FpsSmoother;
pub use input_state::InputState;
pub use screen_tester::ScreenTester;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Stylize},
};

pub trait GameObject {
    fn start(&mut self, display: &mut Display) {}
    fn update(&mut self, dt: f32, input_events: &InputState, display: &mut Display) {}
}

pub struct GameEngine {
    pub debug: Debug,
    objects: Vec<Box<dyn GameObject>>,
    pub display: Display,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            debug: Debug::new(),
            objects: Vec::new(),
            display: Display::new(),
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn GameObject>) {
        self.objects.push(obj);
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut fps_smoother = FpsSmoother::new(0.1);

        let mut last = Instant::now();
        self.display.init()?;
        let mut input = InputState::new();

        self.start();

        'game_loop: loop {
            self.display.resize_check();
            while event::poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key_event) => {
                        if key_event.code == KeyCode::Char('c')
                            && key_event.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break 'game_loop;
                        }

                        input.keys_pressed.push(key_event);

                        match key_event.kind {
                            crossterm::event::KeyEventKind::Press => {
                                input.keys_down.insert(key_event.code);
                            }
                            crossterm::event::KeyEventKind::Release => {
                                input.keys_down.remove(&key_event.code);
                            }
                            _ => {}
                        }
                    }

                    Event::Mouse(mouse_event) => {
                        input.update_mouse(mouse_event);
                    }

                    _ => {}
                }
            }

            let now = Instant::now();
            let dt = (now - last).as_secs_f32();
            last = now;

            self.update(dt, &input);

            let fps = fps_smoother.update(dt);

            if self.debug.enabled {
                self.debug.draw_debug(
                    &[
                        &format!("{:5.0} FPS", fps),
                        &format!(
                            "Screen Size: ({},{})",
                            self.display.width, self.display.height
                        ),
                        &format!(
                            "Mouse Pos: ({},{})",
                            input.mouse_position.0, input.mouse_position.1
                        ),
                    ],
                    &mut self.display,
                );
            }

            self.display.draw_frame()?;
            input.clear_frame_state();
        }
        self.display.cleanup()?;
        Ok(())
    }

    fn start(&mut self) {
        for obj in &mut self.objects {
            obj.start(&mut self.display);
        }
    }

    fn update(&mut self, dt: f32, input_events: &InputState) {
        for obj in &mut self.objects {
            obj.update(dt, input_events, &mut self.display);
        }
    }
}
