use crate::utils;
use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{cursor, execute, style::Print};
use std::io::stdout;

const MSG_DIMENSIONS: (usize, usize) = (48, 17); // made hardcoded

fn strings_array(px: u16) -> [ANSIString<'static>; MSG_DIMENSIONS.1] {
    let default_style = Style::default();
    let title_style = Style::new().fg(Colour::Red);
    let info_style = Style::new().fg(Colour::Blue).bold();
    let simple_info = Style::new().fg(Colour::Blue);
    let arrow_style = Style::new().fg(Colour::Yellow);

    let padding_x = vec![" "; px.try_into().unwrap()].join("");

    let strings_array: [ANSIString; MSG_DIMENSIONS.1] = [
        title_style.paint(padding_x.clone() + "▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄\n"),
        title_style.paint(padding_x.clone() + "█░██░█▀▄▄▀█░███░███▄░▄█▀▄▄▀███▀▄▄▀█░██░▄▄▀█░██░█\n"),
        title_style.paint(padding_x.clone() + "█░▄▄░█░██░█▄▀░▀▄████░██░██░███░▀▀░█░██░▀▀░█░▀▀░█\n"),
        title_style.paint(padding_x.clone() + "█▄██▄██▄▄███▄█▄█████▄███▄▄████░████▄▄█▄██▄█▀▀▀▄█\n"),
        title_style.paint(padding_x.clone() + "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀\n"),
        default_style.paint("\n"),
        info_style.paint(padding_x.clone() + "      Use the arrows to move the character\n"),
        info_style.paint(padding_x.clone() + "      and the r key to restart the level.\n"),
        default_style.paint("\n"),
        arrow_style.paint(padding_x.clone() + "             ↑\n"),
        arrow_style.paint(padding_x.clone() + "           ←   →               r\n"),
        arrow_style.paint(padding_x.clone() + "             ↓\n"),
        default_style.paint("\n"),
        info_style.paint(padding_x.clone() + "           Arrows           Restart\n"),
        default_style.paint("\n"),
        default_style.paint("\n"),
        simple_info.paint(padding_x.clone() + "          Press ENTER or H to return"),
    ];

    strings_array
}

pub fn print() {
    let (columns, rows) = utils::terminal_size();
    let (px, py) = (
        utils::padding(columns, MSG_DIMENSIONS.0 as u16),
        utils::padding(rows, MSG_DIMENSIONS.1 as u16),
    );
    execute!(stdout(), cursor::MoveTo(0, py)).unwrap();
    execute!(stdout(), Print(ANSIStrings(&strings_array(px)))).expect("print home screen failed");
}
