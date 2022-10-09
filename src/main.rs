use ansi_term;
use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyModifiers},
    execute, style, terminal,
};
use rand::Rng;
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

pub mod screens;
pub mod utils;

fn main() -> io::Result<()> {
    // * Setup Terminal
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide).unwrap();

    terminal::enable_raw_mode().unwrap();
    ansi_term::enable_ansi_support().unwrap();

    execute!(stdout(), terminal::SetTitle("Sokoban - RC")).unwrap();

    // * The Actual Game
    let mut game = screens::Sokoban {
        ..Default::default()
    };
    screens::refresh_screen(&game);

    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                // if esc or ctrl + c pressed quit
                let ctrl_c_pressed = event.code == KeyCode::Char('c')
                    && event.modifiers == KeyModifiers::CONTROL;
                let esc_pressed = event.code == KeyCode::Esc;

                if ctrl_c_pressed || esc_pressed {
                    break;
                }

                let should_quit = game.handle_keypress(event.code);

                if should_quit {
                    break;
                }
            }

            // handle transition
            if matches!(game.state, screens::GameState::Transition) {
                screens::refresh_screen(&game);

                if game.transition_manager.check() {
                    screens::refresh_screen(&game);
                    sleep(Duration::from_millis(500)); // momentary pause
                    game.state = screens::GameState::Menu;
                } else {
                    sleep(Duration::from_millis(
                        rand::thread_rng().gen_range(10..=300),
                    ));
                } // momentary pause

                screens::refresh_screen(&game);
            }
        }
    }

    // * Resetting the Terminal
    // TODO: add Reset on Drop
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        style::ResetColor,
        cursor::Show
    )
    .unwrap();
    Ok(())
}
