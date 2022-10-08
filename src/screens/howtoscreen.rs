use ansi_term::{ANSIString, ANSIStrings, Colour, Style};
use crossterm::{
    cursor, execute,
    style::Print,
};
use std::io::stdout;

fn strings_array(px: u16) -> [ANSIString<'static>; 15] {
    let default_style = Style::default();
    let info_style = Style::new().fg(Colour::Blue).bold();
    let simple_info = Style::new().fg(Colour::Blue);
    let arrow_style = Style::new().fg(Colour::Yellow);

    let padding_x = vec![" "; px.try_into().unwrap()].join("");
    
    let strings_array: [ANSIString; 15] = [
        default_style.paint(padding_x.clone() + "▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄\n"),
        default_style.paint(padding_x.clone() + "█░██░█▀▄▄▀█░███░███▄░▄█▀▄▄▀███▀▄▄▀█░██░▄▄▀█░██░█\n"),
        default_style.paint(padding_x.clone() + "█░▄▄░█░██░█▄▀░▀▄████░██░██░███░▀▀░█░██░▀▀░█░▀▀░█\n"),
        default_style.paint(padding_x.clone() + "█▄██▄██▄▄███▄█▄█████▄███▄▄████░████▄▄█▄██▄█▀▀▀▄█\n"),
        default_style.paint(padding_x.clone() + "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀\n"),
        default_style.paint("\n"),
        info_style.paint(   padding_x.clone() + "      Use the arrows to move the character\n"),
        info_style.paint(   padding_x.clone() + "      and the r key to restart the level.\n"),
        arrow_style.paint(  padding_x.clone() + "             ↑\n"),
        arrow_style.paint(  padding_x.clone() + "           ←   →               r\n"),
        arrow_style.paint(  padding_x.clone() + "             ↓\n"), 
        info_style.paint(   padding_x.clone() + "           Arrows           Restart\n"),
        default_style.paint("\n"),
        default_style.paint("\n"),
        simple_info.paint(  padding_x.clone() + "          Press ENTER or H to return"),
    ];
    
    return strings_array;
}

pub const msg_dim: (usize, usize) = (48, 15); // made hardcoded 

pub fn print(px: u16, py: u16) {
    execute!(stdout(), cursor::MoveTo(0, py)).unwrap();
    execute!(stdout(), Print(ANSIStrings(&strings_array(px)))).expect("print home screen failed");
}
