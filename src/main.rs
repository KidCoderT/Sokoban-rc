use ansi_term::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
    terminal,
};
use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant}, str::FromStr,
};

struct Point {
    x: i8, // column
    y: i8, // row
}

struct Game {
    player: Point,
    walls: Vec<Vec<Point>>, // column, row ~ x, y
    storage_locations: Vec<Point>,
    crates: Vec<Point>,
}

// Shows the Game state and is
// important for key pressing
enum GameState {
    HomeScreen,
    HowToPlayScreen,
    IsPlaying,
    LevelFinished,
    GameOver,
}

fn padding(window_size: u16, text_size: u16) -> u16 {
    // Gets the position from where
    // if printed a text it will be shown
    // in the center
    (window_size - text_size) / 2
}

fn get_style(string: &str) -> Style {
    // Gets the Style for the Characters
    match string {
        "@" => Color::White.on(Color::RGB(232,207,0)), // Player
        "*" => Color::White.on(Color::RGB(22,69,142)), // Wall
        "#" => Color::White.on(Color::RGB(243,47,55)), // Crate
        "?" => Color::Green.on(Color::Black), // Storage Loc
        "." => Color::Black.on(Color::Black), // Blank
        _ => Color::Green.on(Color::RGB(33,135,33)) // correct pos
    }
}
 
fn main() -> io::Result<()> {
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    terminal::enable_raw_mode()?;
    let _ = enable_ansi_support();

    // execute!(stdout(), terminal::SetSize(200, 200))?;
    execute!(stdout(), terminal::SetTitle("Sokoban - RC"))?;
    let (columns, rows) = terminal::size().unwrap();

    let text = format!("columns: {columns}, rows: {rows}");
    execute!(stdout(), Print(text))?;

    // Printing all the colors
    execute!(stdout(), Print(get_style("@").paint("\n    Player    ")))?;
    execute!(stdout(), Print(get_style("*").paint("\n    Wall    ")))?;
    execute!(stdout(), Print(get_style("#").paint("\n    Crate    ")))?;
    execute!(stdout(), Print(get_style(".").paint("\n    Blank    ")))?;
    execute!(stdout(), Print(get_style("?").paint("\n    Storage Loc ???")))?;
    execute!(stdout(), Print(get_style("-").paint("\n    Correct Loc")))?;

    // execute!(stdout(), terminal::LeaveAlternateScreen)?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}
