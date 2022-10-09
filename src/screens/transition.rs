use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{cursor, execute, style::Print};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

pub const MSG_DIMENSIONS: (usize, usize) = (29, 13);

pub struct Manager {
    pub start_time: Instant,
    pub wait_time: Duration,
}

impl Default for Manager {
    fn default() -> Self {
        Manager {
            start_time: Instant::now(),
            wait_time: Duration::new(0, 0),
        }
    }
}

impl Manager {
    pub fn check(&mut self) -> bool {
        return self.start_time.elapsed() > self.wait_time;
    }

    pub fn setup(&mut self, wait_millis: u64) {
        self.start_time = Instant::now();
        self.wait_time = Duration::from_millis(wait_millis);
    }

    pub fn print(&self, px: u16, py: u16) {
        execute!(stdout(), cursor::MoveTo(0, py)).unwrap();

        let default_style = Style::default();
        let title_style = Style::new().fg(Colour::Blue);
        let bar_style = Style::new().fg(Colour::Green);
        let arrow_style = Style::new().fg(Colour::Yellow);

        let padding_x = vec![" "; px.try_into().unwrap()].join("");

        let amount: i32 = {
            let mut result = ((self.start_time.elapsed().as_millis() as f64
                / self.wait_time.as_millis() as f64)
                * 25.0)
                .round() as i32;

            if result > 25 {
                result = 25;
            }

            result
        };

        let percentage = vec!["="; amount.try_into().unwrap()].join("");
        let remaining = vec![" "; (25 - amount).try_into().unwrap()].join("");

        let strings_array: [ANSIString; MSG_DIMENSIONS.1] = [
            default_style.paint(String::from("\n")),
            title_style.paint(padding_x.clone() + &String::from("       █▄░█ █▀▀ ▀▄▀ ▀█▀ \n")),
            title_style.paint(padding_x.clone() + &String::from("       █░▀█ ██▄ █░█ ░█░ \n")),
            default_style.paint(String::from("\n")),
            title_style.paint(padding_x.clone() + &String::from("     █░░ █▀▀ █░█ █▀▀ █░░\n")),
            title_style.paint(padding_x.clone() + &String::from("     █▄▄ ██▄ ▀▄▀ ██▄ █▄▄\n")),
            default_style.paint(String::from("\n")),
            default_style.paint(String::from("\n")),
            arrow_style.paint(padding_x.clone() + &String::from("< ")), // THe initial
            bar_style.paint(percentage),                                // the completed amount
            default_style.paint(remaining),                             // the remaining space
            arrow_style.paint(String::from(" > \n")),                   // the closing character
            default_style.paint(String::from("\n")),
        ];

        execute!(stdout(), Print(ANSIStrings(&strings_array))).expect("print home screen failed");
    }
}
