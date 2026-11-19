use chess_core::{destination, square, Color, Game, GameStatus, Move, Piece, Square};
use primitive_app::{App, Change};
use primitive_ui::{back_button, header, Canvas, Input, Rect, TextStyle, CAPTION};
use std::{fs, io::Write, path::Path};
use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

const GRID: Rect = Rect::new(56, 285, 1152, 1152);
const CELL: i32 = 144;
const UNDO: Rect = Rect::new(55, 1475, 260, 100);
const EXPORT: Rect = Rect::new(330, 1475, 335, 100);
const NEW: Rect = Rect::new(680, 1475, 528, 100);
const LEVEL: Rect = Rect::new(810, 150, 397, 100);

pub struct View {
    pub game: Game,
    pub data: Option<PathBuf>,
    pub export: Option<PathBuf>,
    selected: Option<Square>,
    promotions: Vec<Move>,
    confirms_reset: bool,
    deadline: Option<Instant>,
    notice: String,
}

impl View {
    pub fn new(game: Game, data: Option<PathBuf>, export: Option<PathBuf>) -> Self {
        let mut view = Self {
            game,
            data,
            export,
            selected: None,
            promotions: Vec::new(),
            confirms_reset: false,
            deadline: None,
            notice: String::new(),
        };
        view.schedule();
        view
    }

    fn schedule(&mut self) {
        self.deadline = (self.game.status() == GameStatus::Ongoing
            && self.game.board.side_to_move() == Color::Black)
            .then(|| Instant::now() + Duration::from_millis(250));
    }

    fn commit(&mut self) {
        self.notice = self
            .save()
            .err()
            .map_or_else(String::new, |error| format!("Could not save. {error}"));
    }

    fn play(&mut self, mv: Move) -> Change {
        if !self.game.play(mv) {
            return Change::None;
        }
        self.selected = None;
        self.promotions.clear();
        self.schedule();
        self.commit();
        Change::Partial
    }

    fn piece(piece: Piece) -> &'static str {
        match piece {
            Piece::Pawn => "P",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
        }
    }

    fn status(&self) -> &str {
        match self.game.status() {
            GameStatus::Drawn => "Draw",
            GameStatus::Won if self.game.board.side_to_move() == Color::Black => {
                "Checkmate. You win."
            }
            GameStatus::Won => "Checkmate. Kindle wins.",
            GameStatus::Ongoing if self.game.board.side_to_move() == Color::Black => {
                "Kindle is thinking…"
            }
            GameStatus::Ongoing if !self.game.board.checkers().is_empty() => {
                "Your king is in check."
            }
            GameStatus::Ongoing => "Your move",
        }
    }

    fn draw_board(&self, canvas: &mut Canvas) {
        let moves = self
            .selected
            .map(|selected| self.game.allowed(selected))
            .unwrap_or_default();
        for row in 0..8 {
            for column in 0..8 {
                let cell = Rect::new(
                    GRID.x + column as i32 * CELL,
                    GRID.y + row as i32 * CELL,
                    CELL,
                    CELL,
                );
                let at = square(column, row);
                let background = if (row + column) % 2 == 0 { 255 } else { 212 };
                canvas.fill(cell, background);
                if self.selected == Some(at) {
                    canvas.panel(cell.inset(4), 10, 6, background);
                } else if self
                    .game
                    .last_move()
                    .is_some_and(|(from, to)| at == from || at == to)
                {
                    canvas.panel(cell.inset(8), 10, 3, background);
                }
                let target = moves
                    .iter()
                    .any(|&mv| destination(&self.game.board, mv) == at);
                if let Some(piece) = self.game.board.piece_on(at) {
                    let black = self.game.board.color_on(at) == Some(Color::Black);
                    let disk = Rect::new(cell.x + 24, cell.y + 22, 96, 96);
                    canvas.panel(
                        disk,
                        48,
                        if target { 8 } else { 3 },
                        if black { 0 } else { 255 },
                    );
                    canvas.text(
                        Rect::new(cell.x + 25, cell.y + 29, 94, 86),
                        Self::piece(piece),
                        TextStyle::new(56)
                            .bold()
                            .center()
                            .ink(if black { 255 } else { 0 }),
                    );
                } else if target {
                    canvas.circle(Rect::new(cell.x + 58, cell.y + 58, 28, 28), 0);
                }
                if column == 0 {
                    canvas.text(
                        Rect::new(cell.x + 3, cell.y, 30, 32),
                        &(8 - row).to_string(),
                        TextStyle::new(20),
                    );
                }
                if row == 7 {
                    let label = ((b'a' + column as u8) as char).to_string();
                    canvas.text(
                        Rect::new(cell.right() - 25, cell.bottom() - 30, 24, 30),
                        &label,
                        TextStyle::new(20),
                    );
                }
            }
        }
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        header(canvas, "Primitive Chess", "Done");
        canvas.text(
            Rect::new(56, 163, 710, 100),
            self.status(),
            TextStyle::new(48).bold(),
        );
        canvas.button(LEVEL, self.game.level.label(), false, false);
        self.draw_board(canvas);
        canvas.button(UNDO, "Undo", false, false);
        canvas.button(EXPORT, "Export PGN", false, false);
        canvas.button(NEW, "New game", true, false);
        canvas.text(Rect::new(60, 1590, 1140, 65), &self.notice, CAPTION);
        if self.confirms_reset {
            canvas.popup(Rect::new(190, 575, 884, 390));
            canvas.text(
                Rect::new(240, 630, 784, 90),
                "Start a new game?",
                TextStyle::new(50).bold(),
            );
            canvas.text(
                Rect::new(240, 729, 784, 62),
                "The current game will be replaced.",
                CAPTION,
            );
            canvas.button(Rect::new(270, 816, 330, 100), "Cancel", false, false);
            canvas.button(Rect::new(664, 816, 330, 100), "New game", true, false);
        }
        if !self.promotions.is_empty() {
            canvas.popup(Rect::new(200, 575, 864, 460));
            canvas.text(
                Rect::new(245, 630, 774, 100),
                "Choose promotion",
                TextStyle::new(50).bold(),
            );
            for (i, piece) in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight]
                .into_iter()
                .enumerate()
            {
                canvas.button(
                    Rect::new(245 + 195 * i as i32, 770, 180, 100),
                    Self::piece(piece),
                    i == 0,
                    false,
                );
            }
            canvas.button(Rect::new(430, 900, 404, 100), "Cancel", false, false);
        }
    }

    fn input(&mut self, input: Input) -> Change {
        let Input::Tap(x, y) = input else {
            return Change::None;
        };
        if self.confirms_reset {
            if Rect::new(664, 816, 330, 100).contains(x, y) {
                let level = self.game.level;
                self.game = Game::restore(level.label());
                self.deadline = None;
                self.selected = None;
                self.commit();
            }
            self.confirms_reset = false;
            return Change::Full;
        }
        if !self.promotions.is_empty() {
            for (i, piece) in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight]
                .into_iter()
                .enumerate()
            {
                if Rect::new(245 + 195 * i as i32, 770, 180, 100).contains(x, y) {
                    if let Some(mv) = self
                        .promotions
                        .iter()
                        .find(|mv| mv.promotion == Some(piece))
                        .copied()
                    {
                        self.play(mv);
                        return Change::Full;
                    }
                }
            }
            self.promotions.clear();
            return Change::Full;
        }
        if back_button(1264).contains(x, y) {
            return Change::Exit;
        }
        if LEVEL.contains(x, y) {
            self.game.level = self.game.level.next();
            self.commit();
            return Change::Partial;
        }
        if UNDO.contains(x, y) && self.game.undo_turn() {
            self.selected = None;
            self.deadline = None;
            self.commit();
            return Change::Partial;
        }
        if EXPORT.contains(x, y) {
            self.notice = match self.export.as_deref() {
                Some(directory) => match primitive_app::with_storage(|| {
                    export_pgn(directory, self.game.pgn().as_bytes())
                }) {
                    Ok(name) => format!("Saved {name} to Documents."),
                    Err(error) => format!("Could not export the game. {error}"),
                },
                None => "Choose a document folder first.".into(),
            };
            return Change::Partial;
        }
        if NEW.contains(x, y) {
            self.confirms_reset = true;
            return Change::Full;
        }
        if !GRID.contains(x, y)
            || self.game.status() != GameStatus::Ongoing
            || self.game.board.side_to_move() != Color::White
        {
            return Change::None;
        }
        let at = square(
            ((x as i32 - GRID.x) / CELL) as usize,
            ((y as i32 - GRID.y) / CELL) as usize,
        );
        if let Some(selected) = self.selected {
            let moves: Vec<_> = self
                .game
                .allowed(selected)
                .into_iter()
                .filter(|&mv| destination(&self.game.board, mv) == at)
                .collect();
            if moves.len() > 1 {
                self.promotions = moves;
                return Change::Full;
            }
            if let Some(&mv) = moves.first() {
                return self.play(mv);
            }
        }
        self.selected = Some(at).filter(|&sq| {
            self.game.board.color_on(sq) == Some(Color::White) && self.selected != Some(sq)
        });
        Change::Partial
    }

    fn deadline(&self) -> Option<Instant> {
        self.deadline.filter(|_| !self.confirms_reset)
    }

    fn tick(&mut self, _now: Instant) -> Change {
        self.deadline = None;
        if self.game.opponent_move() {
            self.commit();
            Change::Partial
        } else {
            Change::None
        }
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.data {
            primitive_app::save(path, self.game.encoded().as_bytes())?;
        }
        Ok(())
    }
}

fn export_pgn(directory: &Path, contents: &[u8]) -> io::Result<String> {
    for number in 1..=9999 {
        let name = format!("Primitive Chess {number:04}.pgn");
        let path = directory.join(&name);
        let mut out = match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(out) => out,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        };
        if let Err(error) = out.write_all(contents).and_then(|()| out.sync_all()) {
            drop(out);
            let _ = fs::remove_file(path);
            return Err(error);
        }
        return Ok(name);
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "Move some PGN files out of Documents.",
    ))
}
