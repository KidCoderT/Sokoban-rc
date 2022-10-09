use crossterm::{cursor, event::KeyCode, execute, terminal};
use std::{cmp, io::stdout};

mod instructions;
mod menu;
mod transition;

fn padding(window_size: u16, text_size: u16) -> u16 {
    // Gets the position from where
    // if printed a text it will be shown
    // in the center
    if text_size > window_size {
        panic!("Window way toooo Small!!")
    }
    (window_size - text_size) / 2
}

// TODO: Use the func when implementing Game
// fn get_style(string: &str) -> Style {
//     // Gets the Style for the Characters
//     match string {
//         "@" => Colour::White.on(Colour::RGB(232, 207, 0)), // Player
//         "*" => Colour::White.on(Colour::RGB(22, 69, 142)), // Wall
//         "#" => Colour::White.on(Colour::RGB(243, 47, 55)), // Crate
//         "?" => Colour::Green.on(Colour::Black),            // Storage Loc
//         "." => Colour::Black.on(Colour::Black),            // Blank
//         _ => Colour::Green.on(Colour::RGB(33, 135, 33)),   // correct pos
//     }
// }

fn long_text_len(strings: &Vec<&str>) -> u16 {
    let mut max_str_len = 0;

    for string in strings {
        max_str_len = cmp::max(string.chars().count().try_into().unwrap(), max_str_len);
    }

    return max_str_len;
}

// Shows the Game state and is
// important for key pressing
pub enum GameState {
    Menu,
    Instructions,
    Running,
    Transition,
    GameOver,
}

pub struct Sokoban<'a> {
    pub state: GameState,
    pub home_screen: menu::Manager<'a>,
    pub transition_manager: transition::Manager,
}

impl Default for Sokoban<'_> {
    fn default() -> Self {
        Sokoban {
            state: GameState::Menu,
            home_screen: menu::Manager {
                ..Default::default()
            },
            transition_manager: transition::Manager { ticks: 0 },
        }
    }
}

impl Sokoban<'_> {
    pub fn handle_keypress(&mut self, keycode: KeyCode) -> bool {
        let mut should_exit = false;

        match self.state {
            GameState::Menu => match keycode {
                KeyCode::Up => self.home_screen.move_pointer(1),
                KeyCode::Down => self.home_screen.move_pointer(-1),
                KeyCode::Enter | KeyCode::Right => {
                    match self.home_screen.pointer {
                        1 => should_exit = true,
                        2 => self.state = GameState::Instructions,
                        3 => {
                            self.transition_manager.ticks = 0;
                            println!("{}", self.transition_manager.ticks);
                            self.state = GameState::Transition
                        } // FIXME: GameState::Running,
                        _ => panic!("Wth this is not Possible"),
                    }
                    self.home_screen.reset_pointer();
                }
                _ => {}
            },
            GameState::Instructions => match keycode {
                KeyCode::Enter | KeyCode::Char('h') => self.state = GameState::Menu,
                _ => {}
            },
            // TODO: Implement Later
            // No Event for Transition
            _ => {}
        }

        refresh_screen(self);
        return should_exit;
    }
}

pub fn refresh_screen(game: &Sokoban<'_>) {
    let (columns, rows) = terminal::size().unwrap();

    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)
        .expect("refresh_screen() failed");

    match game.state {
        GameState::Menu => {
            let text_length = long_text_len(&game.home_screen.text);
            let text_height = game.home_screen.text.len().try_into().unwrap();

            game.home_screen
                .print(padding(columns, text_length), padding(rows, text_height));
        }
        GameState::Instructions => {
            let (text_length, text_height) = instructions::MSG_DIMENSIONS;

            instructions::print(
                padding(columns, text_length.try_into().unwrap()),
                padding(rows, text_height.try_into().unwrap()),
            );
        }
        GameState::Transition => {
            let (text_length, text_height) = transition::MSG_DIMENSIONS;

            game.transition_manager.print(
                padding(columns, text_length.try_into().unwrap()),
                padding(rows, text_height.try_into().unwrap()),
            );
        }
        // TODO: Implement the Other States
        _ => {}
    }
}
