use chess_core::Game;
use chess_render::View;
use std::{fs, path::PathBuf};

fn main() -> Result<(), String> {
    let dir = PathBuf::from("/mnt/us/chess");
    let data = dir.join("game.txt");
    let export = PathBuf::from("/mnt/us/documents");
    let game = Game::restore(&fs::read_to_string(&data).unwrap_or_default());
    primitive_app::run(View::new(game, Some(data), Some(export)), &dir)
}
