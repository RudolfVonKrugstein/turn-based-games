use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use turn_based_games::model;
use turn_based_games::model::{GameState, MiniMaxGameState, MoveIterator, MoveOutcome, Player};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
enum ChessPiece {
    WhitePawn1 = 0,
    WhitePawn2 = 1,
    WhitePawn3 = 2,
    WhitePawn4 = 3,
    WhitePawn5 = 4,
    WhitePawn6 = 5,
    WhitePawn7 = 6,
    WhitePawn8 = 7,
    WhiteRook1 = 8,
    WhiteRook2 = 9,
    WhiteKnight1 = 10,
    WhiteKnight2 = 11,
    WhiteBishop1 = 12,
    WhiteBishop2 = 13,
    WhiteQueen = 14,
    WhiteKing = 15,
    BlackPawn1 = 16,
    BlackPawn2 = 17,
    BlackPawn3 = 18,
    BlackPawn4 = 19,
    BlackPawn5 = 20,
    BlackPawn6 = 21,
    BlackPawn7 = 22,
    BlackPawn8 = 23,
    BlackRook1 = 24,
    BlackRook2 = 25,
    BlackKnight1 = 26,
    BlackKnight2 = 27,
    BlackBishop1 = 28,
    BlackBishop2 = 29,
    BlackQueen = 30,
    BlackKing = 31,
    NoPiece = 32,
}

impl ChessPiece {
    fn is_valid(&self) -> bool {
        *self != ChessPiece::NoPiece
    }
    fn is_white(&self) -> bool {
        *self >= ChessPiece::WhitePawn1 && *self <= ChessPiece::WhiteKing
    }
    fn is_black(&self) -> bool {
        *self >= ChessPiece::BlackPawn1 && *self <= ChessPiece::BlackKing
    }
    fn is_opposite_color(&self, other: &ChessPiece) -> bool {
        self.is_valid() && other.is_valid() && self.is_white() != other.is_white()
    }
    fn piece_type(&self) -> ChessPieceType {
        if *self >= ChessPiece::WhitePawn1 && *self <= ChessPiece::WhitePawn8
            || *self >= ChessPiece::BlackPawn1 && *self <= ChessPiece::BlackPawn8
        {
            return ChessPieceType::Pawn;
        }
        if *self >= ChessPiece::WhiteRook1 && *self <= ChessPiece::WhiteRook2
            || *self >= ChessPiece::BlackRook1 && *self <= ChessPiece::BlackRook2
        {
            return ChessPieceType::Rook;
        }
        if *self >= ChessPiece::WhiteKnight1 && *self <= ChessPiece::WhiteKnight2
            || *self >= ChessPiece::BlackKnight1 && *self <= ChessPiece::BlackKnight2
        {
            return ChessPieceType::Knight;
        }
        if *self >= ChessPiece::WhiteBishop1 && *self <= ChessPiece::WhiteBishop2
            || *self >= ChessPiece::BlackBishop1 && *self <= ChessPiece::BlackBishop2
        {
            return ChessPieceType::Bishop;
        }
        if *self == ChessPiece::WhiteQueen || *self == ChessPiece::BlackQueen {
            return ChessPieceType::Queen;
        }
        if *self == ChessPiece::WhiteKing || *self == ChessPiece::BlackKing {
            return ChessPieceType::King;
        }
        ChessPieceType::NoType
    }
}

#[derive(PartialEq)]
enum ChessPieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    NoType,
}

type BoardPosition = usize;

fn on_board(pos: BoardPosition) -> bool {
    pos < 64
}

#[derive(Clone, Debug, PartialEq)]
enum ChessMoveType {
    Move(ChessPiece, BoardPosition, BoardPosition),
    Beat(ChessPiece, ChessPiece, BoardPosition, BoardPosition),
    SmallCastling,
    BigCastling,
}

#[derive(Clone, Debug, PartialEq)]
struct ChessMove {
    move_type: ChessMoveType,
    castling_state: CastlingState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CastlingState {
    big_castling_allowed: bool,
    small_castling_allowed: bool,
}

impl CastlingState {
    fn new() -> CastlingState {
        CastlingState {
            big_castling_allowed: true,
            small_castling_allowed: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ChessGameState {
    active_player: model::Player,
    field: [ChessPiece; 64],
    positions: [BoardPosition; 32],
    player1_castling_state: CastlingState,
    player2_castling_state: CastlingState,
}

impl ChessGameState {
    pub fn new() -> ChessGameState {
        use ChessPiece::*;

        let field = [
            WhiteRook1,
            WhiteKnight1,
            WhiteBishop1,
            WhiteKing,
            WhiteQueen,
            WhiteBishop2,
            WhiteKnight2,
            WhiteRook2,
            WhitePawn1,
            WhitePawn2,
            WhitePawn3,
            WhitePawn4,
            WhitePawn5,
            WhitePawn6,
            WhitePawn7,
            WhitePawn8,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            NoPiece,
            BlackPawn1,
            BlackPawn2,
            BlackPawn3,
            BlackPawn4,
            BlackPawn5,
            BlackPawn6,
            BlackPawn7,
            BlackPawn8,
            BlackRook1,
            BlackKnight1,
            BlackBishop1,
            BlackKing,
            BlackQueen,
            BlackBishop2,
            BlackKnight2,
            BlackRook2,
        ];
        ChessGameState::new_with_field(field, Player::Player1)
    }

    fn create_move(&self, t: ChessMoveType) -> ChessMove {
        ChessMove {
            move_type: t,
            castling_state: if self.active_player == Player::Player1 {
                self.player1_castling_state.clone()
            } else {
                self.player2_castling_state.clone()
            },
        }
    }

    fn create_move_move(&self, s: BoardPosition, e: BoardPosition) -> ChessMove {
        self.create_move(ChessMoveType::Move(self.field[s], s, e))
    }

    fn create_beat_move(&self, s: BoardPosition, e: BoardPosition) -> ChessMove {
        self.create_move(ChessMoveType::Beat(self.field[s], self.field[e], s, e))
    }

    fn new_with_field(field: [ChessPiece; 64], active_player: Player) -> ChessGameState {
        let mut positions = [64; 32];
        for (pos, piece) in field.iter().enumerate() {
            if *piece == ChessPiece::NoPiece {
                continue;
            }
            positions[*piece as usize] = pos;
        }
        ChessGameState {
            active_player,
            field,
            positions,
            player1_castling_state: CastlingState::new(),
            player2_castling_state: CastlingState::new(),
        }
    }

    fn relative_field_pos(
        self: &Self,
        pos: &BoardPosition,
        dx: isize,
        dy: isize,
    ) -> Option<BoardPosition> {
        let target_pos: isize = (*pos as isize) + dx + 8 * dy;
        // First check all out of bound cases!
        if target_pos >= 64 || target_pos < 0 {
            return None;
        }
        if (*pos as isize) % 8 + dx >= 8 || (*pos as isize) % 8 + dx < 0 {
            return None;
        }
        return Some(target_pos as usize);
    }

    fn generate_moves_in_directory(
        &self,
        pos: BoardPosition,
        directory: (isize, isize),
        moves: &mut Vec<ChessMove>,
    ) {
        for dist in 1..9 {
            if let Some(target_pos) =
                self.relative_field_pos(&pos, directory.0 * dist, directory.1 * dist)
            {
                if self.field[target_pos].is_opposite_color(&self.field[pos]) {
                    moves.push(self.create_beat_move(pos, target_pos));
                }
                if self.field[target_pos] == ChessPiece::NoPiece {
                    moves.push(self.create_move_move(pos, target_pos));
                } else {
                    break;
                }
            }
        }
    }

    fn generate_pawn_moves(&self, pos: BoardPosition, moves: &mut Vec<ChessMove>) {
        // Find the move directory based on the color
        let dy = if self.field[pos].is_white() { 1 } else { -1 };
        // Check if we can move one field
        if let Some(target_pos) = self.relative_field_pos(&pos, 0, dy) {
            if self.field[target_pos] == ChessPiece::NoPiece {
                // We can move 1 field, store it!
                moves.push(self.create_move_move(pos, target_pos));
                // Can we also move 2 fields?
                if let Some(target_pos2) = self.relative_field_pos(&pos, 0, 2 * dy) {
                    // And are we allowed to move 2 fields? That is only the case if we have not moved yet
                    if self.field[pos].is_white() && pos / 8 == 1
                        || self.field[pos].is_black() && pos / 8 == 6
                    {
                        if self.field[target_pos2] == ChessPiece::NoPiece {
                            // Yes, store the move!
                            moves.push(self.create_move_move(pos, target_pos2));
                        }
                    }
                }
            }
        }
        // Check if we can beat!
        for dx in [-1, 1] {
            if let Some(target_pos) = self.relative_field_pos(&pos, dx, dy) {
                if self.field[target_pos].is_opposite_color(&self.field[pos]) {
                    // Yes we can beat!
                    moves.push(self.create_beat_move(pos, target_pos));
                }
            }
        }
    }

    fn generate_knight_moves(&self, pos: BoardPosition, moves: &mut Vec<ChessMove>) {
        for (dx, dy) in [
            (1, 2),
            (-1, 2),
            (-1, -2),
            (1, -2),
            (2, 1),
            (2, -1),
            (-2, -1),
            (-2, 1),
        ] {
            if let Some(target_pos) = self.relative_field_pos(&pos, dx, dy) {
                if self.field[target_pos] == ChessPiece::NoPiece {
                    moves.push(self.create_move_move(pos, target_pos));
                }
                if self.field[target_pos].is_opposite_color(&self.field[pos]) {
                    moves.push(self.create_beat_move(pos, target_pos));
                }
            }
        }
    }

    fn generate_rook_moves(&self, pos: BoardPosition, moves: &mut Vec<ChessMove>) {
        self.generate_moves_in_directory(pos, (0, 1), moves);
        self.generate_moves_in_directory(pos, (0, -1), moves);
        self.generate_moves_in_directory(pos, (1, 0), moves);
        self.generate_moves_in_directory(pos, (-1, 0), moves);
    }

    fn generate_bishop_moves(&self, pos: BoardPosition, moves: &mut Vec<ChessMove>) {
        self.generate_moves_in_directory(pos, (1, 1), moves);
        self.generate_moves_in_directory(pos, (1, -1), moves);
        self.generate_moves_in_directory(pos, (-1, 1), moves);
        self.generate_moves_in_directory(pos, (-1, -1), moves);
    }

    fn generate_queen_moves(&self, pos: BoardPosition, moves: &mut Vec<ChessMove>) {
        self.generate_moves_in_directory(pos, (0, 1), moves);
        self.generate_moves_in_directory(pos, (0, -1), moves);
        self.generate_moves_in_directory(pos, (1, 0), moves);
        self.generate_moves_in_directory(pos, (-1, 0), moves);
        self.generate_moves_in_directory(pos, (1, 1), moves);
        self.generate_moves_in_directory(pos, (1, -1), moves);
        self.generate_moves_in_directory(pos, (-1, 1), moves);
        self.generate_moves_in_directory(pos, (-1, -1), moves);
    }

    fn generate_king_moves(&self, pos: BoardPosition, moves: &mut Vec<ChessMove>) {
        for (dx, dy) in [
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ] {
            if let Some(target_pos) = self.relative_field_pos(&pos, dx, dy) {
                if self.field[target_pos] == ChessPiece::NoPiece {
                    moves.push(self.create_move_move(pos, target_pos));
                }
                if self.field[target_pos].is_opposite_color(&self.field[pos]) {
                    moves.push(self.create_beat_move(pos, target_pos));
                }
            }
        }
    }

    fn generate_castling_moves(&self, moves: &mut Vec<ChessMove>) {
        if self.active_player == Player::Player1 {
            if self.player1_castling_state.small_castling_allowed {
                if self.field[0] == ChessPiece::WhiteRook1
                    && self.field[1] == ChessPiece::NoPiece
                    && self.field[2] == ChessPiece::NoPiece
                    && self.field[3] == ChessPiece::WhiteKing
                {
                    moves.push(self.create_move(ChessMoveType::SmallCastling));
                }
            }
            if self.player1_castling_state.big_castling_allowed {
                if self.field[7] == ChessPiece::WhiteRook2
                    && self.field[6] == ChessPiece::NoPiece
                    && self.field[5] == ChessPiece::NoPiece
                    && self.field[4] == ChessPiece::NoPiece
                    && self.field[3] == ChessPiece::WhiteKing
                {
                    moves.push(self.create_move(ChessMoveType::BigCastling));
                }
            }
        } else {
            if self.player2_castling_state.small_castling_allowed {
                if self.field[56] == ChessPiece::BlackRook1
                    && self.field[57] == ChessPiece::NoPiece
                    && self.field[58] == ChessPiece::NoPiece
                    && self.field[59] == ChessPiece::BlackKing
                {
                    moves.push(self.create_move(ChessMoveType::SmallCastling));
                }
            }
            if self.player2_castling_state.big_castling_allowed {
                if self.field[63] == ChessPiece::BlackRook2
                    && self.field[62] == ChessPiece::NoPiece
                    && self.field[61] == ChessPiece::NoPiece
                    && self.field[60] == ChessPiece::NoPiece
                    && self.field[59] == ChessPiece::BlackKing
                {
                    moves.push(self.create_move(ChessMoveType::BigCastling));
                }
            }
        }
    }

    fn update_castling_from_moved_piece(self: &mut Self, piece: &ChessPiece) {
        if *piece == ChessPiece::WhiteRook1 || *piece == ChessPiece::WhiteKing {
            self.player1_castling_state.small_castling_allowed = false;
        };
        if *piece == ChessPiece::WhiteRook2 || *piece == ChessPiece::WhiteKing {
            self.player1_castling_state.big_castling_allowed = false;
        };
        if *piece == ChessPiece::BlackRook1 || *piece == ChessPiece::BlackKing {
            self.player2_castling_state.small_castling_allowed = false;
        };
        if *piece == ChessPiece::BlackRook2 || *piece == ChessPiece::BlackKing {
            self.player2_castling_state.big_castling_allowed = false;
        };
    }
}

impl GameState for ChessGameState {
    type Move = ChessMove;
    type MoveIterator = ChessMoveIterator;

    fn active_player(self: &Self) -> Player {
        self.active_player
    }

    fn move_iterator(self: &Self) -> Self::MoveIterator {
        let mut moves = Vec::new();
        use ChessPiece::*;
        let pieces = if self.active_player == Player::Player1 {
            [
                WhitePawn1,
                WhitePawn2,
                WhitePawn3,
                WhitePawn4,
                WhitePawn5,
                WhitePawn6,
                WhitePawn7,
                WhitePawn8,
                WhiteRook1,
                WhiteRook2,
                WhiteKnight1,
                WhiteKnight2,
                WhiteBishop1,
                WhiteBishop2,
                WhiteQueen,
                WhiteKing,
            ]
        } else {
            [
                BlackPawn1,
                BlackPawn2,
                BlackPawn3,
                BlackPawn4,
                BlackPawn5,
                BlackPawn6,
                BlackPawn7,
                BlackPawn8,
                BlackRook1,
                BlackRook2,
                BlackKnight1,
                BlackKnight2,
                BlackBishop1,
                BlackBishop2,
                BlackQueen,
                BlackKing,
            ]
        };
        for piece in pieces {
            let piece_index = piece as usize;
            if on_board(self.positions[piece_index]) {
                match piece.piece_type() {
                    ChessPieceType::Pawn => {
                        self.generate_pawn_moves(self.positions[piece_index], &mut moves);
                    }
                    ChessPieceType::Rook => {
                        self.generate_rook_moves(self.positions[piece_index], &mut moves);
                    }
                    ChessPieceType::Knight => {
                        self.generate_knight_moves(self.positions[piece_index], &mut moves);
                    }
                    ChessPieceType::Bishop => {
                        self.generate_bishop_moves(self.positions[piece_index], &mut moves);
                    }
                    ChessPieceType::Queen => {
                        self.generate_queen_moves(self.positions[piece_index], &mut moves);
                    }
                    ChessPieceType::King => {
                        self.generate_king_moves(self.positions[piece_index], &mut moves);
                    }
                    ChessPieceType::NoType => {
                        panic!("Unreachable code");
                    }
                };
            }
        }
        self.generate_castling_moves(&mut moves);
        ChessMoveIterator { moves, index: 0 }
    }

    fn apply_move(self: &mut Self, m: &Self::Move) -> MoveOutcome {
        self.active_player = self.active_player.other();
        match m.move_type {
            ChessMoveType::Move(piece, start, target) => {
                self.field[target] = piece;
                self.field[start] = ChessPiece::NoPiece;
                self.positions[piece as usize] = target;
                // Update castling
                self.update_castling_from_moved_piece(&piece);
                MoveOutcome::SwitchPlayer(self.active_player.clone())
            }
            ChessMoveType::Beat(piece, beat_piece, start, target) => {
                self.field[target] = piece;
                self.field[start] = ChessPiece::NoPiece;
                self.positions[piece as usize] = target;
                self.positions[beat_piece as usize] = 65;
                // Update castling
                self.update_castling_from_moved_piece(&piece);
                if beat_piece.piece_type() == ChessPieceType::King {
                    MoveOutcome::PlayerWon(self.active_player.other())
                } else {
                    MoveOutcome::SwitchPlayer(self.active_player.clone())
                }
            }
            ChessMoveType::SmallCastling => {
                match self.active_player.other() {
                    Player::Player1 => {
                        self.field[0] = ChessPiece::WhiteKing;
                        self.field[3] = ChessPiece::WhiteRook1;
                    }
                    Player::Player2 => {
                        self.field[56] = ChessPiece::BlackKing;
                        self.field[59] = ChessPiece::BlackRook1;
                    }
                };
                MoveOutcome::SwitchPlayer(self.active_player.clone())
            }
            ChessMoveType::BigCastling => {
                match self.active_player.other() {
                    Player::Player1 => {
                        self.field[7] = ChessPiece::WhiteKing;
                        self.field[3] = ChessPiece::WhiteRook2;
                    }
                    Player::Player2 => {
                        self.field[63] = ChessPiece::BlackKing;
                        self.field[59] = ChessPiece::BlackRook2;
                    }
                };
                MoveOutcome::SwitchPlayer(self.active_player.clone())
            }
        }
    }

    fn reverse_move(self: &mut Self, m: &Self::Move) {
        self.active_player = self.active_player.other();
        // Restore castling
        match self.active_player {
            Player::Player1 => self.player1_castling_state = m.castling_state,
            Player::Player2 => self.player2_castling_state = m.castling_state,
        };

        // Undo move itself
        match m.move_type {
            ChessMoveType::Move(piece, start, target) => {
                self.field[start] = piece;
                self.field[target] = ChessPiece::NoPiece;
                self.positions[piece as usize] = start;
            }
            ChessMoveType::Beat(piece, beat_piece, start, target) => {
                self.field[start] = piece;
                self.field[target] = beat_piece;
                self.positions[piece as usize] = start;
                self.positions[beat_piece as usize] = target;
            }
            ChessMoveType::SmallCastling => {
                match self.active_player {
                    Player::Player1 => {
                        self.field[3] = ChessPiece::WhiteKing;
                        self.field[0] = ChessPiece::WhiteRook1;
                    }
                    Player::Player2 => {
                        self.field[59] = ChessPiece::BlackKing;
                        self.field[56] = ChessPiece::BlackRook1;
                    }
                };
            }
            ChessMoveType::BigCastling => {
                match self.active_player {
                    Player::Player1 => {
                        self.field[3] = ChessPiece::WhiteKing;
                        self.field[7] = ChessPiece::WhiteRook2;
                    }
                    Player::Player2 => {
                        self.field[59] = ChessPiece::BlackKing;
                        self.field[63] = ChessPiece::BlackRook2;
                    }
                };
            }
        }
    }
}

struct ChessMoveIterator {
    moves: Vec<ChessMove>,
    index: usize,
}

impl MoveIterator for ChessMoveIterator {
    type Move = ChessMove;
    type GameState = ChessGameState;

    fn next(&mut self, _gs: &Self::GameState) -> Option<&Self::Move> {
        self.index += 1;
        self.moves.get(self.index - 1)
    }
}

impl MiniMaxGameState for ChessGameState {
    fn evaluate(self: &Self, player: &Player) -> i64 {
        static PAWN_VALUE: i64 = 100;
        static KNIGHT_VALUE: i64 = 350;
        static BISHOP_VALUE: i64 = 350;
        static ROOK_VALUE: i64 = 525;
        static QUEEN_VALUE: i64 = 1000;

        let mut white_score: i64 = 0;
        let mut black_score: i64 = 0;

        for (index, piece) in ChessPiece::iter().enumerate() {
            if let Some(p) = self.positions.get(index) {
                if on_board(*p) {
                    use ChessPieceType::*;
                    let value = match piece.piece_type() {
                        Pawn => PAWN_VALUE,
                        Knight => KNIGHT_VALUE,
                        Bishop => BISHOP_VALUE,
                        Rook => ROOK_VALUE,
                        Queen => QUEEN_VALUE,
                        King => 0,
                        NoType => panic!("unreachable code!"),
                    };
                    if piece.is_white() {
                        white_score += value;
                    } else {
                        black_score += value;
                    }
                }
            }
        }
        match player {
            Player::Player1 => white_score - black_score,
            Player::Player2 => black_score - white_score,
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use turn_based_games::model::GameState;

    // Helper function
    fn find_move_by_positions(gs: &ChessGameState, start: usize, end: usize) -> Option<ChessMove> {
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            match m.move_type {
                ChessMoveType::Move(_, m_start, m_end) => {
                    if start == m_start && end == m_end {
                        return Some(m.clone());
                    }
                }
                ChessMoveType::Beat(_, _, m_start, m_end) => {
                    if start == m_start && end == m_end {
                        return Some(m.clone());
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn find_small_castling_move(gs: &ChessGameState) -> Option<ChessMove> {
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            match m.move_type {
                ChessMoveType::SmallCastling => {
                    return Some(m.clone());
                }
                _ => {}
            }
        }
        None
    }

    fn find_big_castling_move(gs: &ChessGameState) -> Option<ChessMove> {
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            match m.move_type {
                ChessMoveType::BigCastling => {
                    return Some(m.clone());
                }
                _ => {}
            }
        }
        None
    }

    // Returns a game state for castlings
    fn castling_game_state(start_player: Player) -> ChessGameState {
        use ChessPiece::*;
        let field = [
            WhiteRook1, NoPiece, NoPiece, WhiteKing, NoPiece, NoPiece, NoPiece, WhiteRook2,
            WhitePawn1, WhitePawn2, WhitePawn3, WhitePawn4, WhitePawn5, WhitePawn6, WhitePawn7,
            WhitePawn8, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, BlackPawn1, BlackPawn2,
            BlackPawn3, BlackPawn4, BlackPawn5, BlackPawn6, BlackPawn7, BlackPawn8, BlackRook1,
            NoPiece, NoPiece, BlackKing, NoPiece, NoPiece, NoPiece, BlackRook2,
        ];
        ChessGameState::new_with_field(field, start_player)
    }

    #[test]
    fn start_moves_white() {
        // Setup
        let gs = ChessGameState::new();

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        use ChessPiece::*;
        let expected_result = [
            ChessMoveType::Move(WhitePawn1, 8, 16),
            ChessMoveType::Move(WhitePawn2, 9, 17),
            ChessMoveType::Move(WhitePawn3, 10, 18),
            ChessMoveType::Move(WhitePawn4, 11, 19),
            ChessMoveType::Move(WhitePawn5, 12, 20),
            ChessMoveType::Move(WhitePawn6, 13, 21),
            ChessMoveType::Move(WhitePawn7, 14, 22),
            ChessMoveType::Move(WhitePawn8, 15, 23),
            ChessMoveType::Move(WhitePawn1, 8, 24),
            ChessMoveType::Move(WhitePawn2, 9, 25),
            ChessMoveType::Move(WhitePawn3, 10, 26),
            ChessMoveType::Move(WhitePawn4, 11, 27),
            ChessMoveType::Move(WhitePawn5, 12, 28),
            ChessMoveType::Move(WhitePawn6, 13, 29),
            ChessMoveType::Move(WhitePawn7, 14, 30),
            ChessMoveType::Move(WhitePawn8, 15, 31),
            ChessMoveType::Move(WhiteKnight1, 1, 16),
            ChessMoveType::Move(WhiteKnight1, 1, 18),
            ChessMoveType::Move(WhiteKnight2, 6, 21),
            ChessMoveType::Move(WhiteKnight2, 6, 23),
        ];
        assert_eq!(moves.len(), expected_result.len());
        for expected_move in expected_result {
            assert!(
                moves.contains(&expected_move),
                "{:?} not found",
                expected_move
            )
        }
    }

    #[test]
    fn start_moves_black() {
        // Setup
        let mut gs = ChessGameState::new();
        gs.apply_move(&find_move_by_positions(&gs, 8, 16).unwrap());

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        use ChessPiece::*;
        let expected_result = [
            ChessMoveType::Move(BlackPawn1, 48, 40),
            ChessMoveType::Move(BlackPawn2, 49, 41),
            ChessMoveType::Move(BlackPawn3, 50, 42),
            ChessMoveType::Move(BlackPawn4, 51, 43),
            ChessMoveType::Move(BlackPawn5, 52, 44),
            ChessMoveType::Move(BlackPawn6, 53, 45),
            ChessMoveType::Move(BlackPawn7, 54, 46),
            ChessMoveType::Move(BlackPawn8, 55, 47),
            ChessMoveType::Move(BlackPawn1, 48, 32),
            ChessMoveType::Move(BlackPawn2, 49, 33),
            ChessMoveType::Move(BlackPawn3, 50, 34),
            ChessMoveType::Move(BlackPawn4, 51, 35),
            ChessMoveType::Move(BlackPawn5, 52, 36),
            ChessMoveType::Move(BlackPawn6, 53, 37),
            ChessMoveType::Move(BlackPawn7, 54, 38),
            ChessMoveType::Move(BlackPawn8, 55, 39),
            ChessMoveType::Move(BlackKnight1, 57, 40),
            ChessMoveType::Move(BlackKnight1, 57, 42),
            ChessMoveType::Move(BlackKnight2, 62, 45),
            ChessMoveType::Move(BlackKnight2, 62, 47),
        ];
        assert_eq!(moves.len(), expected_result.len());
        for expected_move in expected_result {
            assert!(
                moves.contains(&expected_move),
                "{:?} not found",
                expected_move
            )
        }
    }

    #[test]
    fn undo_move() {
        // Setup
        let mut gs = ChessGameState::new();

        // Act
        let original_gs = gs.clone();
        let m = find_move_by_positions(&gs, 8, 16).unwrap();
        gs.apply_move(&m);
        gs.reverse_move(&m);

        // Test
        assert_eq!(original_gs, gs);
    }

    #[test]
    fn undo_beat() {
        // Setup
        let mut gs = ChessGameState::new();

        // Act
        // Setup beat position
        gs.apply_move(&find_move_by_positions(&gs, 1, 18).unwrap()); // Move the knight
        gs.apply_move(&find_move_by_positions(&gs, 49, 33).unwrap()); // Move pawn to be beaten by knight

        // Store original positions
        let original_gs = gs.clone();
        // Find beat move
        let beat_move = find_move_by_positions(&gs, 18, 33).unwrap();
        gs.apply_move(&beat_move);
        gs.reverse_move(&beat_move);

        // Test
        assert!(matches!(beat_move.move_type, ChessMoveType::Beat(..)));
        assert_eq!(original_gs, gs);
    }

    #[test]
    fn small_castling_white() {
        // Setup
        let gs = castling_game_state(Player::Player1);

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(moves.contains(&ChessMoveType::SmallCastling));
    }

    #[test]
    fn undo_small_castling_white() {
        // Setup
        let mut gs = castling_game_state(Player::Player1);

        // Act
        let original_gs = gs.clone();
        let castling_move = find_small_castling_move(&gs).unwrap();
        gs.apply_move(&castling_move);
        let gs_with_castling = gs.clone();
        gs.reverse_move(&castling_move);

        // Test
        assert_eq!(original_gs, gs);
        assert_eq!(gs_with_castling.field[0], ChessPiece::WhiteKing);
        assert_eq!(gs_with_castling.field[3], ChessPiece::WhiteRook1);
    }

    #[test]
    fn big_castling_white() {
        // Setup
        let gs = castling_game_state(Player::Player1);

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(moves.contains(&ChessMoveType::BigCastling));
    }

    #[test]
    fn undo_big_castling_white() {
        // Setup
        let mut gs = castling_game_state(Player::Player1);

        // Act
        let original_gs = gs.clone();
        let castling_move = find_big_castling_move(&gs).unwrap();
        gs.apply_move(&castling_move);
        let gs_with_castling = gs.clone();
        gs.reverse_move(&castling_move);

        // Test
        assert_eq!(original_gs, gs);
        assert_eq!(gs_with_castling.field[7], ChessPiece::WhiteKing);
        assert_eq!(gs_with_castling.field[3], ChessPiece::WhiteRook2);
    }

    #[test]
    fn small_castling_black() {
        // Setup
        let gs = castling_game_state(Player::Player2);

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(moves.contains(&ChessMoveType::SmallCastling));
    }

    #[test]
    fn undo_small_castling_black() {
        // Setup
        let mut gs = castling_game_state(Player::Player2);

        // Act
        let original_gs = gs.clone();
        let castling_move = find_small_castling_move(&gs).unwrap();
        gs.apply_move(&castling_move);
        let gs_with_castling = gs.clone();
        gs.reverse_move(&castling_move);

        // Test
        assert_eq!(original_gs, gs);
        assert_eq!(gs_with_castling.field[56], ChessPiece::BlackKing);
        assert_eq!(gs_with_castling.field[59], ChessPiece::BlackRook1);
    }

    #[test]
    fn big_castling_black() {
        // Setup
        let gs = castling_game_state(Player::Player2);

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(moves.contains(&ChessMoveType::BigCastling));
    }

    #[test]
    fn undo_big_castling_black() {
        // Setup
        let mut gs = castling_game_state(Player::Player2);

        // Act
        let original_gs = gs.clone();
        let castling_move = find_big_castling_move(&gs).unwrap();
        gs.apply_move(&castling_move);
        let gs_with_castling = gs.clone();
        gs.reverse_move(&castling_move);

        // Test
        assert_eq!(original_gs, gs);
        assert_eq!(gs_with_castling.field[63], ChessPiece::BlackKing);
        assert_eq!(gs_with_castling.field[59], ChessPiece::BlackRook2);
    }

    #[test]
    fn small_castling_white_disallowed() {
        // Setup
        let mut gs = castling_game_state(Player::Player1);

        // Move the rook and back
        gs.apply_move(&find_move_by_positions(&gs, 0, 1).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 48, 40).unwrap()); // Just a move of black
        gs.apply_move(&find_move_by_positions(&gs, 1, 0).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 49, 41).unwrap()); // Just a move of black

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(!moves.contains(&ChessMoveType::SmallCastling));
    }

    #[test]
    fn big_castling_white_disallowed() {
        // Setup
        let mut gs = castling_game_state(Player::Player1);
        // Move the rook and back
        gs.apply_move(&find_move_by_positions(&gs, 7, 6).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 48, 40).unwrap()); // Just a move of black
        gs.apply_move(&find_move_by_positions(&gs, 6, 7).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 49, 41).unwrap()); // Just a move of black

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(!moves.contains(&ChessMoveType::BigCastling));
    }

    #[test]
    fn small_castling_black_disallowed() {
        // Setup
        let mut gs = castling_game_state(Player::Player2);
        // Move the rook and back
        gs.apply_move(&find_move_by_positions(&gs, 56, 57).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 8, 16).unwrap()); // Just a move of white
        gs.apply_move(&find_move_by_positions(&gs, 57, 56).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 9, 17).unwrap()); // Just a move of white

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(!moves.contains(&ChessMoveType::SmallCastling));
    }

    #[test]
    fn big_castling_black_disallowed() {
        // Setup
        let mut gs = castling_game_state(Player::Player2);

        // Move the rook and back
        gs.apply_move(&find_move_by_positions(&gs, 63, 62).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 8, 16).unwrap()); // Just a move of white
        gs.apply_move(&find_move_by_positions(&gs, 62, 63).unwrap());
        gs.apply_move(&find_move_by_positions(&gs, 9, 17).unwrap()); // Just a move of white

        // Act
        let mut moves = Vec::new();
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            moves.push(m.move_type.clone());
        }

        // Test
        assert!(!moves.contains(&ChessMoveType::BigCastling));
    }

    #[test]
    fn all_pieces_score() {
        // Setup
        // get the default field
        let mut default_field = ChessGameState::new().field;
        // Remove all black pieces
        for i in 32..64 {
            default_field[i] = ChessPiece::NoPiece;
        }
        let gs = ChessGameState::new_with_field(default_field, Player::Player1);

        // Act
        let white_score = gs.evaluate(&Player::Player1);
        let black_score = gs.evaluate(&Player::Player2);

        // Test
        assert_eq!(white_score, 4250);
        assert_eq!(black_score, -4250);
    }
}
