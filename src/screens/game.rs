use ansi_term::{Colour, Style};
use std::{self, io::stdout, str::FromStr, time::Instant};

use crate::utils;

enum GameCharacters {
    Wall,
    Blank,
}

fn get_style(string: &str) -> Style {
    // Gets the Style for the Characters
    match string {
        "@" => Style::new().hidden().on(Colour::RGB(232, 207, 0)), // Player
        "*" => Style::new().hidden().on(Colour::RGB(22, 69, 142)), // Wall
        "#" => Style::new().hidden().on(Colour::RGB(243, 47, 55)), // Crate
        "?" => Colour::Green.normal(),                             // Storage Loc
        "." => Colour::Black.hidden(),                             // Blank
        _ => Colour::Green.on(Colour::RGB(33, 135, 33)),           // correct pos
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn from_map<'a>(&self, map: &'a [Vec<GameCharacters>]) -> &'a GameCharacters {
        &map[self.y][self.x]
    }
}

struct Level {
    pub index: i8,
    pub map: Vec<Vec<GameCharacters>>,
    pub crates: Vec<Point>,
    pub storage_points: Vec<Point>,
    pub player: Point,
}

impl Level {
    fn crate_index_from_position(&self, point: &Point) -> Option<usize> {
        for index in 0..self.crates.len() {
            if self.crates[index].x == point.x && self.crates[index].y == point.y {
                return Some(index);
            }
        }

        return None;
    }

    pub fn make_move(&mut self, point_inc: Point) {
        let new_point = Point {
            x: self.player.x + &point_inc.x,
            y: self.player.y + &point_inc.y,
        };
        let mut should_move_player = false;

        // check if not hitting wall
        if let GameCharacters::Blank = new_point.from_map(self.map.as_slice()) {
            match self.crate_index_from_position(&new_point) {
                Some(i) => {
                    let crate_point = Point {
                        x: &new_point.x + point_inc.x,
                        y: &new_point.y + point_inc.y,
                    };

                    if let GameCharacters::Blank = crate_point.from_map(self.map.as_slice()) {
                        should_move_player = true;

                        match self.crate_index_from_position(&new_point) {
                            None => {
                                self.crates[i].x = crate_point.x;
                                self.crates[i].y = crate_point.y;

                                should_move_player = true;
                            }
                            _ => should_move_player = false,
                        }
                    }
                }
                None => should_move_player = true,
                _ => {}
            }
        }

        if should_move_player {
            self.player.x -= point_inc.x;
            self.player.y -= point_inc.y;
        }
    }
}

// fn

struct Manager {
    pub current_level: Level,
    pub start_time: Instant,
    pub last_sec: u64,
    pub changed: bool,
}

impl Manager {}
