#![allow(dead_code)]
use crate::game::VoxGame;
use crate::logger::setup_logger;

mod game;
mod logger;
mod personal_world;
mod ui;

fn main() {
    setup_logger().unwrap();
    let mut game = VoxGame::new();
    game.run();
}
