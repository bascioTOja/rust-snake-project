use crossterm::{
    style::{Color, Stylize},
    terminal,
};
use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

#[derive(Clone)]
pub struct PixelChar {
    pub character: char,
    pub foreground: Color,
    pub background: Color,
}

pub struct Display {
    pub width: usize,
    pub height: usize,
    stdout: std::io::Stdout,
    grid: Vec<Vec<PixelChar>>,
}

impl Drop for Display {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[allow(dead_code)]
impl Display {
    pub fn new() -> Self {
        let size = terminal::size().unwrap();
        let width = (size.0) as usize;
        let height = (size.1) as usize;

        Self {
            width: width,
            height: height,
            stdout: stdout(),
            grid: vec![
                vec![
                    PixelChar {
                        character: ' ',
                        foreground: Color::Rgb {
                            r: 255,
                            g: 255,
                            b: 255
                        },
                        background: Color::Rgb { r: 0, g: 0, b: 0 },
                    };
                    width
                ];
                height
            ],
        }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;

        Ok(())
    }

    pub fn draw_frame(&mut self) -> std::io::Result<()> {
        execute!(self.stdout, MoveTo(0, 0))?;
        let frame = self.get_frame();
        write!(self.stdout, "{frame}")?;
        execute!(self.stdout, MoveTo(0, 0))?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> std::io::Result<()> {
        disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen, Show, DisableMouseCapture)?;
        Ok(())
    }

    pub fn resize_check(&mut self) {
        let size = terminal::size().unwrap();
        let width = (size.0) as usize;
        let height = (size.1) as usize;

        if width != self.width || height != self.height {
            self.resize();
        }
    }

    fn resize(&mut self) {
        let size = terminal::size().unwrap();
        self.width = (size.0) as usize;
        self.height = (size.1) as usize;

        let template = PixelChar {
            character: ' ',
            foreground: Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
            background: Color::Rgb { r: 00, g: 00, b: 0 },
        };

        self.grid
            .resize(self.height, vec![template.clone(); self.width]);
        for row in self.grid.iter_mut() {
            row.resize(self.width, template.clone());
        }
    }

    pub fn set_px_fg(&mut self, x: usize, y: usize, color: Color) {
        self.grid[y][x].foreground = color;
    }

    pub fn set_px_bg(&mut self, x: usize, y: usize, color: Color) {
        self.grid[y][x].background = color;
    }

    pub fn set_px_char(&mut self, x: usize, y: usize, character: char) {
        self.grid[y][x].character = character;
    }

    pub fn set_area_fg(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        for y in start_y..start_y + height {
            for x in start_x..start_x + width {
                if x >= self.width || y >= self.height {
                    continue;
                }
                self.set_px_fg(x, y, color);
            }
        }
    }

    pub fn set_area_bg(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        for y in start_y..start_y + height {
            for x in start_x..start_x + width {
                if x >= self.width || y >= self.height {
                    continue;
                }
                self.set_px_bg(x, y, color);
            }
        }
    }

    pub fn set_area_char(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        character: char,
    ) {
        for y in start_y..start_y + height {
            for x in start_x..start_x + width {
                if x >= self.width || y >= self.height {
                    continue;
                }
                self.set_px_char(x, y, character);
            }
        }
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;

        (x, y)
    }

    pub fn insert_raw_str(
        &mut self,
        raw_str: &str,
        start_x: usize,
        start_y: usize,
    ) -> (usize, usize) {
        let position = self.xy_to_index(start_x, start_y);

        // if position + raw_str.len() > self.width * self.height {
        //     return (start_x, start_y);
        // }

        let mut last_pos = (start_x, start_y);

        for (i, c) in raw_str.chars().enumerate() {
            let (x, y) = self.index_to_xy(position + i);
            if x >= self.width || y >= self.height {
                continue;
            }
            self.grid[y][x].character = c;
            last_pos = (x, y);
        }

        last_pos
    }

    pub fn insert_raw_str_fg(
        &mut self,
        raw_str: &str,
        start_x: usize,
        start_y: usize,
        fg_color: Color,
    ) -> (usize, usize) {
        let position = self.xy_to_index(start_x, start_y);

        // if position + raw_str.len() > self.width * self.height {
        //     return (start_x, start_y);
        // }

        let mut last_pos = (start_x, start_y);

        for (i, c) in raw_str.chars().enumerate() {
            let (x, y) = self.index_to_xy(position + i);
            if x >= self.width || y >= self.height {
                continue;
            }
            self.grid[y][x].character = c;
            self.grid[y][x].foreground = fg_color;
            last_pos = (x, y);
        }

        last_pos
    }

    pub fn insert_raw_str_bg(
        &mut self,
        raw_str: &str,
        start_x: usize,
        start_y: usize,
        bg_color: Color,
    ) -> (usize, usize) {
        let position = self.xy_to_index(start_x, start_y);

        // if position + raw_str.len() > self.width * self.height {
        //     return (start_x, start_y);
        // }

        let mut last_pos = (start_x, start_y);

        for (i, c) in raw_str.chars().enumerate() {
            let (x, y) = self.index_to_xy(position + i);
            if x >= self.width || y >= self.height {
                continue;
            }
            self.grid[y][x].character = c;
            self.grid[y][x].background = bg_color;
            last_pos = (x, y);
        }

        last_pos
    }

    pub fn insert_raw_str_fg_bg(
        &mut self,
        raw_str: &str,
        start_x: usize,
        start_y: usize,
        fg_color: Color,
        bg_color: Color,
    ) -> (usize, usize) {
        let position = self.xy_to_index(start_x, start_y);

        // if position + raw_str.len() > self.width * self.height {
        //     return (start_x, start_y);
        // }

        let mut last_pos = (start_x, start_y);

        for (i, c) in raw_str.chars().enumerate() {
            let (x, y) = self.index_to_xy(position + i);
            if x >= self.width || y >= self.height {
                continue;
            }
            self.grid[y][x].character = c;
            self.grid[y][x].foreground = fg_color;
            self.grid[y][x].background = bg_color;
            last_pos = (x, y);
        }

        last_pos
    }

    pub fn get_frame(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                // let test = &self.grid[y][x];
                // let temp_str = test
                //     .character
                //     .to_string()
                //     .with(test.foreground)
                //     .on(test.background)
                //     .to_string();
                // output.push_str(&temp_str);

                //better xD
                let cell = &self.grid[y][x];
                std::fmt::Write::write_fmt(
                    &mut output,
                    format_args!(
                        "{}",
                        cell.character.with(cell.foreground).on(cell.background)
                    ),
                )
                .unwrap();
            }
            if y < self.height - 1 {
                output.push('\n');
                output.push('\r');
            }
        }

        output
    }
}
