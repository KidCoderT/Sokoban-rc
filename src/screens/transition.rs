use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{cursor, execute, style::Print};
use rand::{thread_rng, Rng};
use std::io::stdout;

pub const MSG_DIMENSIONS: (usize, usize) = (29, 13);

pub struct Manager {
    pub ticks: i8,
}

impl Manager {
    pub fn tick(&mut self) -> bool {
        let mut rng = thread_rng();
        let increment = rng.gen_range(0..=30);
        self.ticks = self.ticks + increment;

        if self.ticks >= 100 {
            self.ticks = 0;
            return true;
        }

        return false;
    }

    pub fn print(&self, px: u16, py: u16) {
        execute!(stdout(), cursor::MoveTo(0, py)).unwrap();

        let default_style = Style::default();
        let title_style = Style::new().fg(Colour::Blue);
        let bar_style = Style::new().fg(Colour::Green);
        let arrow_style = Style::new().fg(Colour::Yellow);

        let padding_x = vec![" "; px.try_into().unwrap()].join("");

        let amount = (&self.ticks / 100) * 25;
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
