use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{
    cursor, execute,
    style::{self, Print},
    terminal,
};
use std::{cmp, io::stdout};

fn padding(window_size: u16, text_size: u16) -> u16 {
    // Gets the position from where
    // if printed a text it will be shown
    // in the center
    (window_size - text_size) / 2
}

fn get_style(string: &str) -> Style {
    // Gets the Style for the Characters
    match string {
        "@" => Colour::White.on(Colour::RGB(232, 207, 0)), // Player
        "*" => Colour::White.on(Colour::RGB(22, 69, 142)), // Wall
        "#" => Colour::White.on(Colour::RGB(243, 47, 55)), // Crate
        "?" => Colour::Green.on(Colour::Black),            // Storage Loc
        "." => Colour::Black.on(Colour::Black),            // Blank
        _ => Colour::Green.on(Colour::RGB(33, 135, 33)),   // correct pos
    }
}

fn long_text_len(strings: &Vec<&str>) -> u16 {
    let mut max_str_len = 0;

    for string in strings {
        max_str_len = cmp::max(string.len().try_into().unwrap(), max_str_len);
    }

    return max_str_len;
}

// Shows the Game state and is
// important for key pressing
// pub enum GameState {
//     HomeScreen,
//     HowToPlayScreen,
//     IsPlaying,
//     LevelFinished,
//     GameOver,
// }

pub struct HomeScreenData<'a> {
    pointer: i8, // 1, 2, 3
    text: Vec<&'a str>,
    color: Vec<Style>,
}

impl Default for HomeScreenData<'_> {
    fn default() -> Self {
        let title = Style::new().fg(Colour::RGB(243, 47, 55));
        let option = Style::new().fg(Colour::RGB(33, 135, 33)).bold();

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
            "                                                        ",
            "                     > New Game      ",
            "                     > How to play   ",
            "                     > Exit          ",
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
            option,
            option,
            option,
        ];

        // let mut strings: Vec<ANSIString<'static>> = Vec::new();

        // for i in 0..colors.len() {
        //     strings.push(colors[i].paint(text[i]));
        // }

        HomeScreenData {
            pointer: 3,
            text,
            color,
        }
    }
}

impl HomeScreenData<'_> {
    fn move_pointer(&mut self, amount: i8) {
        self.pointer += amount;

        if self.pointer < 1 {
            self.pointer = 3;
        } else if self.pointer > 3 {
            self.pointer = 1
        }
    }

    fn reset_pointer(&mut self) {
        self.pointer = 3;
    }

    fn print(&self, px: u16, py: u16) {
        for i in 0..self.text.len() {
            let index = i.clone() as u16;

            execute!(stdout(), cursor::MoveTo(px, py + index),).unwrap();

            let text = self.text[i];
            let highlight = Style::new().fg(Colour::White).on(Colour::Red);
            let mut string = self.color[i].paint(text);

            match (self.text.len() - i, self.pointer) {
                (1, 1) | (2, 2) | (3, 3) => {
                    string = highlight.paint(text);
                }
                _ => {}
            }

            execute!(stdout(), Print(string)).expect("print home screen failed");
        }
    }
}

pub fn refresh_screen() {
    let (columns, rows) = terminal::size().unwrap();
    execute!(stdout(), terminal::EnterAlternateScreen,).expect("refresh_screen() failed");
    let homescreen = HomeScreenData {
        ..Default::default()
    };
    homescreen.print(
        padding(columns, long_text_len(&homescreen.text)),
        padding(rows, homescreen.text.len().try_into().unwrap()),
    );
}
