use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{
    cursor, execute,
    style::Print,
};
use std::io::stdout;

pub struct Manager<'a> {
    pub pointer: i8, // 1, 2, 3
    pub text: Vec<&'a str>,
    pub color: Vec<Style>,
}

impl Default for Manager<'_> {
    fn default() -> Self {
        let title = Style::new().fg(Colour::RGB(243, 47, 55));

        let text = vec![
            "",
            " ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄   ▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄ ▄▄    ▄ ",
            "█       █       █   █ █ █       █  ▄    █      █  █  █ █",
            "█  ▄▄▄▄▄█   ▄   █   █▄█ █   ▄   █ █▄█   █  ▄   █   █▄█ █",
            "█ █▄▄▄▄▄█  █ █  █      ▄█  █ █  █       █ █▄█  █       █",
            "█▄▄▄▄▄  █  █▄█  █     █▄█  █▄█  █  ▄   ██      █  ▄    █",
            " ▄▄▄▄▄█ █       █    ▄  █       █ █▄█   █  ▄   █ █ █   █",
            "█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄▄▄█ █▄█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄█ █▄▄█▄█  █▄▄█",
            "",
            "",
            "                     ", // to be filled
            "                     ", // to be filled
            "                     ", // to be filled
        ];

        let color = vec![
            Style::default(),
            title,
            title,
            title,
            title,
            title,
            title,
            title,
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default(),
        ];

        Manager {
            pointer: 3,
            text,
            color,
        }
    }
}

impl Manager<'_> {
    pub fn move_pointer(&mut self, amount: i8) {
        self.pointer += amount;

        if self.pointer < 1 {
            self.pointer = 3;
        } else if self.pointer > 3 {
            self.pointer = 1
        }
    }

    pub fn reset_pointer(&mut self) {
        self.pointer = 3;
    }

    pub fn print(&self, px: u16, py: u16) {
        for i in 0..self.text.len() {
            let index = i.clone() as u16;
            let reverse_index = (self.text.len() - i) as u16;

            execute!(stdout(), cursor::MoveTo(px, py + index),).unwrap();

            let ansi_string = self.color[i].paint(self.text[i].to_string());
            let mut final_string: Vec<ANSIString> = Vec::new();
            final_string.push(ansi_string);
            
            if reverse_index < 4 {
                let mut text = String::new();

                let mut highlight = Style::new().fg(Colour::RGB(33, 135, 33));
                let is_highlighted = reverse_index == self.pointer.try_into().unwrap();
                if reverse_index == self.pointer.try_into().unwrap() {
                    highlight = Style::new().fg(Colour::White).on(Colour::Red);
                }

                match reverse_index {
                    3 => text = format!("{} New Game", if is_highlighted {">"} else {" "}),
                    2 => text = format!("{} How to play?", if is_highlighted {">"} else {" "}),
                    1 => text = format!("{} Exit", if is_highlighted {">"} else {" "}),
                    _ => {}
                }

                final_string.push(highlight.paint(text));
            } else {
                final_string.push(Style::default().paint(""));
            }

            let strings_array: &[ANSIString<'static>] = &[final_string[0].clone(), final_string[1].clone()];
            execute!(stdout(), Print(ANSIStrings(strings_array))).expect("print home screen failed");
        }
    }
}
