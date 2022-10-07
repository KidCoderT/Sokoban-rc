use ansi_term;
use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Print},
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

// ! can use SetColors when printing game
// ! can use SetBackgroundColor Sets a default bg color for all the printed text

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

fn get_style(string: &str) -> ansi_term::Style {
    // Gets the Style for the Characters
    match string {
        "@" => ansi_term::Color::White.on(ansi_term::Color::RGB(232, 207, 0)), // Player
        "*" => ansi_term::Color::White.on(ansi_term::Color::RGB(22, 69, 142)), // Wall
        "#" => ansi_term::Color::White.on(ansi_term::Color::RGB(243, 47, 55)), // Crate
        "?" => ansi_term::Color::Green.on(ansi_term::Color::Black),            // Storage Loc
        "." => ansi_term::Color::Black.on(ansi_term::Color::Black),            // Blank
        _ => ansi_term::Color::Green.on(ansi_term::Color::RGB(33, 135, 33)),   // correct pos
    }
}

fn main() -> io::Result<()> {
    // * Setup Terminal
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    terminal::enable_raw_mode()?;
    let _ = ansi_term::enable_ansi_support();

    execute!(stdout(), terminal::SetTitle("Sokoban - RC"))?;
    // execute!(stdout(), terminal::SetSize(200, 200))?;

    // * The Actual Game
    let (mut columns, mut rows) = terminal::size().unwrap();

    let text = format!("columns: {columns}, rows: {rows}");
    execute!(stdout(), Print(text))?;

    // Printing all the colors
    execute!(stdout(), Print(get_style("@").paint("\n    Player    ")))?;
    execute!(stdout(), Print(get_style("*").paint("\n    Wall    ")))?;
    execute!(stdout(), Print(get_style("#").paint("\n    Crate    ")))?;
    execute!(stdout(), Print(get_style(".").paint("\n    Blank    ")))?;
    execute!(
        stdout(),
        Print(get_style("?").paint("\n    Storage Loc ???"))
    )?;
    execute!(stdout(), Print(get_style("-").paint("\n    Correct Loc\n")))?;

    loop {
        // `poll()` waits for an `Event` for a given time period
        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(event) => {
                    // match the important events
                    match event.code {
                        KeyCode::Left => println!("Left key pressed"),
                        KeyCode::Right => println!("Right key pressed"),
                        KeyCode::Up => println!("Up key pressed"),
                        KeyCode::Down => println!("Down key pressed"),
                        KeyCode::Enter => println!("Enter key pressed"),
                        KeyCode::Char('r') => println!("R key pressed"),
                        _ => {}
                    }

                    // if esc or ctrl + c pressed quit
                    let ctrl_c_pressed = event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL;
                    let esc_pressed = event.code == KeyCode::Esc;

                    if ctrl_c_pressed || esc_pressed {
                        break
                    }
                },
                Event::Resize(width, height) => {
                    // reset the dimensions and redraw
                    (columns, rows) = terminal::size().unwrap();
                },
                _ => {}
            }
        }
    }

    // * Resetting the Terminal
    execute!(stdout(), style::ResetColor, cursor::Show)?;
    Ok(())
}
