use crossterm::style::{Color, Stylize};
use std::collections::VecDeque;

use crate::engine::Display;

#[derive(Clone, Copy)]
pub enum LogType {
    Log,
    Warn,
    Error,
    SUccess,
}

#[derive(Clone)]
pub struct Log {
    pub message: String,
    pub log_type: LogType,
}

pub struct Debug {
    pub enabled: bool,
    display_logs: bool,
    logs: VecDeque<Log>,
    scroll_back: usize,
}

impl Debug {
    pub fn new() -> Self {
        Self {
            enabled: false,
            display_logs: false,
            logs: VecDeque::new(),
            scroll_back: 0,
        }
    }

    pub fn log<S>(&mut self, message: S, log_type: LogType)
    where
        S: Into<String>,
    {
        self.logs.push_back(Log {
            message: message.into(),
            log_type,
        });

        if self.logs.len() > 100 {
            self.logs.pop_front();
        }
    }

    pub fn toggle_debug(&mut self, debug_state: bool) {
        self.enabled = debug_state;
    }

    fn wrap_line(log: &Log, width: usize) -> Vec<Log> {
        let mut result: Vec<Log> = Vec::new();
        let mut current = String::new();

        for c in log.message.chars() {
            current.push(c);

            if current.chars().count() >= width {
                result.push(Log {
                    message: current,
                    log_type: log.log_type,
                });
                current = String::new();
            }
        }

        if !current.is_empty() {
            result.push(Log {
                message: current,
                log_type: log.log_type,
            });
        }

        result
    }

    fn log_color(log_type: LogType) -> Color {
        match log_type {
            LogType::Log => Color::White,
            LogType::Warn => Color::Yellow,
            LogType::Error => Color::Red,
            LogType::SUccess => Color::Green,
        }
    }

    pub fn render_logs_overlay(&mut self, display: &mut Display) {
        display.set_area_bg(0, 0, display.width, display.height, Color::Black);
        display.set_area_fg(0, 0, display.width, display.height, Color::White);
        display.set_area_char(0, 0, display.width, display.height, ' ');

        let mut visual_lines = Vec::new();

        for log in &self.logs {
            let wrapped = Debug::wrap_line(&log, display.width);
            for line in wrapped {
                visual_lines.push(line);
            }
        }

        let height = display.height;
        let total = visual_lines.len();

        let end = total.saturating_sub(self.scroll_back);
        let start = end.saturating_sub(height);

        let visible = &visual_lines[start..end];

        for (y, line) in visible.iter().enumerate() {
            display.insert_raw_str_fg(&line.message, 0, y, Debug::log_color(line.log_type));
        }
    }

    pub fn draw_debug(&mut self, lines: &[&str], display: &mut Display) {
        display.set_area_bg(0, display.height - 1, display.width, 1, Color::DarkGrey);

        display.set_area_fg(0, display.height - 1, display.width, 1, Color::DarkGrey);

        let mut position: (usize, usize) = (0, display.height - 1);

        for s in lines.iter() {
            position =
                display.insert_raw_str_fg(&format!("{s} |"), position.0, position.1, Color::White);

            position.0 += 1;
        }
    }
}
