use ansi_term;
use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Print},
    terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

pub mod screens;

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

fn main() -> io::Result<()> {
    // * Setup Terminal
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    terminal::enable_raw_mode()?;
    let _ = ansi_term::enable_ansi_support();

    execute!(stdout(), terminal::SetTitle("Sokoban - RC"))?;

    // * The Actual Game
    screens::refresh_screen();

    loop {
        if poll(Duration::from_millis(200))? {
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
                        _ => screens::refresh_screen(),
                    }

                    // if esc or ctrl + c pressed quit
                    let ctrl_c_pressed = event.code == KeyCode::Char('c')
                        && event.modifiers == KeyModifiers::CONTROL;
                    let esc_pressed = event.code == KeyCode::Esc;

                    if ctrl_c_pressed || esc_pressed {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    // * Resetting the Terminal
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        style::ResetColor,
        cursor::Show
    )?;
    Ok(())
}
