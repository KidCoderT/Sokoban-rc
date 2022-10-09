use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{cursor, execute, style::Print};
use std::io::stdout;

pub const MSG_DIMENSIONS: (usize, usize) = (24, 22); // made hardcoded

fn strings_array(px: u16) -> [ANSIString<'static>; MSG_DIMENSIONS.1] {
    let default_style = Style::default();
    let title_style = Colour::White.on(Colour::Yellow);
    let title_empty_style = Style::new().hidden().on(Colour::Yellow);
    let trophy_style = Style::new().fg(Colour::Yellow);

    let padding_x = vec![" "; px.try_into().unwrap()].join("");

    let strings_array: [ANSIString; MSG_DIMENSIONS.1] = [
        default_style.paint("\n"),
        default_style.paint(padding_x.clone()),
        title_style.paint("         █░█░█ █ █▄░█"),
        title_empty_style.paint("--------\n"),
        default_style.paint(padding_x.clone()),
        title_style.paint("         ▀▄▀▄▀ █ █░▀█"),
        title_empty_style.paint("--------\n"),
        default_style.paint("\n"),
        trophy_style.paint(padding_x.clone() + "         ___________\n"),
        trophy_style.paint(padding_x.clone() + "        '._==_==_=_.'\n"),
        trophy_style.paint(padding_x.clone() + "        .-\\:      /-.\n"),
        trophy_style.paint(padding_x.clone() + "       | (|:.     |) |\n"),
        trophy_style.paint(padding_x.clone() + "        '-|:.     |-'\n"),
        trophy_style.paint(padding_x.clone() + "          \\::.    /\n"),
        trophy_style.paint(padding_x.clone() + "           '::. .'\n"),
        trophy_style.paint(padding_x.clone() + "             ) (\n"),
        trophy_style.paint(padding_x.clone() + "           _.' '._\n"),
        trophy_style.paint(padding_x.clone() + "          `\"\"\"\"\"\"\"`\n"),
        default_style.paint("\n"),
        default_style.paint(padding_x.clone()),
        title_style.paint("  Press ENTER to go to Back!  "),
        default_style.paint("\n"),
    ];

    return strings_array;
}

pub fn print(px: u16, py: u16) {
    execute!(stdout(), cursor::MoveTo(0, py)).unwrap();
    execute!(stdout(), Print(ANSIStrings(&strings_array(px)))).expect("print home screen failed");
}
