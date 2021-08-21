#![allow(dead_code)]
use crate::game::VoxGame;
use crate::logger::setup_logger;
use log::warn;
use std::time::Instant;

mod game;
mod logger;
mod personal_world;

fn main() {
    setup_logger().unwrap();
    let timer = Instant::now();

    let mut game = VoxGame::new();
    game.run();
    warn!("game ran for {} seconds", timer.elapsed().as_secs_f64());
}
