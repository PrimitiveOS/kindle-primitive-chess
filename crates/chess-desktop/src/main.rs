use chess_core::Game;
use chess_render::View;
use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut app = View::new(Game::default(), None, Some(output.clone()));
    assert_eq!(app.input(Input::Tap(700.0, 1220.0)), Change::Partial);
    write_png(
        &output.join("chess-moves.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    assert_eq!(app.input(Input::Tap(700.0, 930.0)), Change::Partial);
    assert_eq!(app.tick(Instant::now()), Change::Partial);
    assert_eq!(app.input(Input::Tap(400.0, 1510.0)), Change::Partial);
    assert!(fs::read_dir(&output)?.flatten().any(|entry| entry
        .path()
        .extension()
        .is_some_and(|extension| extension == "pgn")));
    write_png(
        &output.join("chess-dark.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, true),
    )?;
    Ok(())
}
