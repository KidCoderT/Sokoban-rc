use crossterm::terminal;
use std::cmp;

pub fn padding(window_size: u16, text_size: u16) -> u16 {
    // Gets the position from where
    // if printed a text it will be shown
    // in the center
    if text_size > window_size {
        panic!("Window way too Small!!")
    }
    (window_size - text_size) / 2
}

pub fn long_text_len(strings: &Vec<&str>) -> u16 {
    let mut max_str_len = 0;

    for string in strings {
        max_str_len = cmp::max(string.chars().count().try_into().unwrap(), max_str_len);
    }

    max_str_len
}

pub fn terminal_size() -> (u16, u16) {
    terminal::size().unwrap()
}
