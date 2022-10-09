use crossterm::{cursor, event::KeyCode, execute, terminal};
use std::io::stdout;

mod finished;
mod game;
mod instructions;
mod menu;
mod transition;

// TODO: WHEN TRANSITIONING
// {
//     self.state = GameState::Transition;
//     self.transition_manager.setup(2500);
// }

// Shows the Game state and is
// important for key pressing
pub enum GameState {
    Instructions,
    Transition,
    GameOver,
    Running,
    Menu,
}

pub struct Sokoban<'a> {
    pub state: GameState,
    pub menu_manager: menu::Manager<'a>,
    pub transition_manager: transition::Manager,
}

impl Default for Sokoban<'_> {
    fn default() -> Self {
        Sokoban {
            state: GameState::Menu,
            menu_manager: menu::Manager {
                ..Default::default()
            },
            transition_manager: transition::Manager {
                ..Default::default()
            },
        }
    }
}

impl Sokoban<'_> {
    pub fn handle_keypress(&mut self, keycode: KeyCode) -> bool {
        let mut should_exit = false;

        match self.state {
            GameState::Menu => match keycode {
                KeyCode::Up => self.menu_manager.move_pointer(1),
                KeyCode::Down => self.menu_manager.move_pointer(-1),
                KeyCode::Enter | KeyCode::Right => {
                    match self.menu_manager.pointer {
                        1 => should_exit = true,
                        2 => self.state = GameState::Instructions,
                        3 => self.state = GameState::Running,
                        _ => panic!("Wth this is not Possible"),
                    }
                    self.menu_manager.reset_pointer();
                }
                _ => {}
            },
            GameState::Instructions => match keycode {
                KeyCode::Enter | KeyCode::Char('h') => self.state = GameState::Menu,
                _ => {}
            },
            GameState::GameOver => {
                if keycode == KeyCode::Enter {
                    self.state = GameState::Menu
                }
            }

            GameState::Running => {
                // FIXME
                if keycode == KeyCode::Enter {
                    self.state = GameState::Menu
                }
            }
            // TODO: Implement Later
            // No Event for Transition
            _ => {}
        }

        refresh_screen(self);
        should_exit
    }
}

pub fn refresh_screen(game: &Sokoban<'_>) {
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)
        .expect("refresh_screen() failed");

    match game.state {
        GameState::Menu => game.menu_manager.print(),
        GameState::Instructions => instructions::print(),
        GameState::Transition => game.transition_manager.print(),
        GameState::GameOver => finished::print(),
        GameState::Running => {
            // FIXME
            println!("I was'nt able to finish this on time");
            println!("I will finish this after the competition but im happy");
            println!("I Submitted just to cement th progrss i made which im very happy about!");
            println!("Press enter to exit");
            println!("That i made this much in just 5 days");
        }
        // TODO: Implement the Other States
        _ => {}
    }
}
