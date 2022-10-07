use ansi_term;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Print, Stylize},
    terminal,
};
use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
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

fn main() -> io::Result<()> {
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;
    terminal::enable_raw_mode()?;
    let _ = ansi_term::enable_ansi_support();

    // execute!(stdout(), terminal::SetSize(200, 200))?;
    execute!(stdout(), terminal::SetTitle("Sokoban - RC"))?;
    let (columns, rows) = terminal::size().unwrap();

    let text = format!("hello {columns} {rows}");
    execute!(stdout(), Print(text)).expect("Problem");

    // execute!(stdout(), terminal::LeaveAlternateScreen)?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}
