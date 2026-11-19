use cozy_chess::{
    util::{display_san_move, display_uci_move, parse_uci_move},
    File, Rank,
};
pub use cozy_chess::{Board, Color, GameStatus, Move, Piece, Square};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Level {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Level {
    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Easy => Self::Medium,
            Self::Medium => Self::Hard,
            Self::Hard => Self::Easy,
        }
    }

    fn depth(self) -> usize {
        match self {
            Self::Easy => 1,
            Self::Medium => 2,
            Self::Hard => 3,
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub board: Board,
    pub level: Level,
    history: Vec<(Board, Move)>,
}

impl Game {
    pub fn restore(raw: &str) -> Self {
        let mut lines = raw.lines();
        let level = match lines.next() {
            Some("Easy") => Level::Easy,
            Some("Hard") => Level::Hard,
            _ => Level::Medium,
        };
        let mut game = Self {
            level,
            ..Self::default()
        };
        for line in lines.take(1024) {
            let Ok(mv) = parse_uci_move(&game.board, line) else {
                break;
            };
            if !game.play(mv) {
                break;
            }
        }
        game
    }

    pub fn encoded(&self) -> String {
        let mut state = self.level.label().to_owned();
        for (board, mv) in &self.history {
            state.push('\n');
            state.push_str(&display_uci_move(board, *mv).to_string());
        }
        state.push('\n');
        state
    }

    pub fn status(&self) -> GameStatus {
        let status = self.board.status();
        if status != GameStatus::Ongoing {
            return status;
        }
        if self
            .history
            .iter()
            .filter(|(old, _)| old.same_position(&self.board))
            .take(2)
            .count()
            == 2
            || insufficient_material(&self.board)
        {
            return GameStatus::Drawn;
        }
        GameStatus::Ongoing
    }

    pub fn play(&mut self, mv: Move) -> bool {
        if self.status() != GameStatus::Ongoing {
            return false;
        }
        let previous = self.board.clone();
        if self.board.try_play(mv).is_err() {
            return false;
        }
        self.history.push((previous, mv));
        true
    }

    pub fn undo_turn(&mut self) -> bool {
        let mut changed = false;
        while let Some((old, _)) = self.history.pop() {
            self.board = old;
            changed = true;
            if self.board.side_to_move() == Color::White {
                break;
            }
        }
        changed
    }

    pub fn allowed(&self, square: Square) -> Vec<Move> {
        legal(&self.board)
            .into_iter()
            .filter(|mv| mv.from == square)
            .collect()
    }

    pub fn opponent_move(&mut self) -> bool {
        if self.status() != GameStatus::Ongoing || self.board.side_to_move() != Color::Black {
            return false;
        }
        best_move(&self.board, self.level.depth()).is_some_and(|mv| self.play(mv))
    }

    pub fn last_move(&self) -> Option<(Square, Square)> {
        self.history
            .last()
            .map(|(board, mv)| (mv.from, destination(board, *mv)))
    }

    pub fn pgn(&self) -> String {
        let outcome = match self.status() {
            GameStatus::Drawn => "1/2-1/2",
            GameStatus::Won if self.board.side_to_move() == Color::White => "0-1",
            GameStatus::Won => "1-0",
            GameStatus::Ongoing => "*",
        };
        let mut body = String::new();
        for (i, (board, mv)) in self.history.iter().enumerate() {
            if i % 2 == 0 {
                body.push_str(&format!("{}. ", i / 2 + 1));
            }
            body.push_str(&display_san_move(board, *mv).to_string());
            body.push(' ');
        }
        format!("[Event \"Primitive Chess\"]\n[Site \"Kindle\"]\n[Date \"????.??.??\"]\n[Round \"1\"]\n[White \"Player\"]\n[Black \"Kindle\"]\n[Result \"{outcome}\"]\n\n{body}{outcome}\n")
    }
}

pub fn destination(board: &Board, mv: Move) -> Square {
    if board.piece_on(mv.from) == Some(Piece::King)
        && board.color_on(mv.to) == Some(board.side_to_move())
    {
        return Square::new(
            if mv.to.file() > mv.from.file() {
                File::G
            } else {
                File::C
            },
            mv.from.rank(),
        );
    }
    mv.to
}

pub fn square(column: usize, row: usize) -> Square {
    Square::new(File::ALL[column], Rank::ALL[7 - row])
}

fn legal(board: &Board) -> Vec<Move> {
    let mut moves = Vec::with_capacity(32);
    board.generate_moves(|chunk| {
        moves.extend(chunk);
        false
    });
    moves
}

fn insufficient_material(board: &Board) -> bool {
    let pawns_or_heavy =
        board.pieces(Piece::Pawn) | board.pieces(Piece::Rook) | board.pieces(Piece::Queen);
    if !pawns_or_heavy.is_empty() {
        return false;
    }
    let bishops = board.pieces(Piece::Bishop);
    let knights = board.pieces(Piece::Knight);
    if bishops.len() + knights.len() <= 1 {
        return true;
    }
    knights.is_empty() && {
        const DARK: u64 = 0xaa55_aa55_aa55_aa55;
        let squares = bishops.0;
        squares & DARK == 0 || squares & !DARK == 0
    }
}

fn value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 335,
        Piece::Rook => 510,
        Piece::Queen => 940,
        Piece::King => 0,
    }
}

fn score(board: &Board) -> i32 {
    let side = board.side_to_move();
    let center = [Square::D4, Square::E4, Square::D5, Square::E5];
    let control = |side| {
        Piece::ALL
            .into_iter()
            .map(|piece| board.colored_pieces(side, piece).len() as i32 * value(piece))
            .sum::<i32>()
            + center
                .iter()
                .filter(|&&sq| board.color_on(sq) == Some(side))
                .count() as i32
                * 15
    };
    control(side) - control(!side)
}

fn ordered(board: &Board) -> Vec<Move> {
    let mut moves = legal(board);
    moves.sort_by_cached_key(|mv| {
        let capture = board.piece_on(mv.to).map(value).unwrap_or_default();
        let promotion = mv.promotion.map(value).unwrap_or_default();
        -(capture + promotion)
    });
    moves
}

fn search(board: &Board, depth: usize, mut alpha: i32, beta: i32) -> i32 {
    match board.status() {
        GameStatus::Drawn => return 0,
        GameStatus::Won => return -20_000 - depth as i32,
        GameStatus::Ongoing if depth == 0 => return score(board),
        GameStatus::Ongoing => {}
    }
    for mv in ordered(board) {
        let mut next = board.clone();
        next.play(mv);
        let current = -search(&next, depth - 1, -beta, -alpha);
        if current >= beta {
            return current;
        }
        alpha = alpha.max(current);
    }
    alpha
}

fn best_move(board: &Board, depth: usize) -> Option<Move> {
    let mut alpha = -30_000;
    let mut result = None;
    for mv in ordered(board) {
        let mut next = board.clone();
        next.play(mv);
        let value = -search(&next, depth - 1, -30_000, -alpha);
        if value > alpha {
            alpha = value;
            result = Some(mv);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_move_opponent_and_resume_preserve_the_real_position() {
        let mut game = Game::default();
        assert_eq!(legal(&game.board).len(), 20);
        assert!(game.play(parse_uci_move(&game.board, "e2e4").unwrap()));
        assert!(game.opponent_move());
        assert_eq!(game.board.side_to_move(), Color::White);
        assert_eq!(Game::restore(&game.encoded()).board, game.board);
        assert!(game.pgn().contains("1. e4"));
        assert!(game.undo_turn());
        assert_eq!(game.board, Board::default());
    }

    #[test]
    fn detects_threefold_repetition() {
        let game = Game::restore("Easy\ng1f3\ng8f6\nf3g1\nf6g8\ng1f3\ng8f6\nf3g1\nf6g8\n");
        assert_eq!(game.status(), GameStatus::Drawn);
    }
}
