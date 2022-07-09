use turn_based_games::model::MoveOutcome::{PlayerWon, SwitchPlayer, Tie};
use turn_based_games::model::{GameState, MiniMaxGameState, MoveOutcome, Player};
use turn_based_games::tournament;

#[derive(PartialEq, Clone, Copy)]
enum FieldState {
    EMPTY,
    CROSS,
    CIRCLE,
}

#[derive(Clone, Debug)]
struct Move {
    index: usize,
}

struct MoveIterator {
    current_index: usize,
    current_move: Move,
}

impl MoveIterator {
    fn new() -> MoveIterator {
        MoveIterator {
            current_index: 0,
            current_move: Move { index: 9 },
        }
    }
}

impl turn_based_games::model::MoveIterator for MoveIterator {
    type Move = Move;
    type GameState = TicTacToeGameState;

    fn next(&mut self, gs: &Self::GameState) -> Option<&Self::Move> {
        for index in self.current_index..9 {
            if gs.field[index] == FieldState::EMPTY {
                self.current_index = index + 1;
                self.current_move = Move { index };
                return Some(&self.current_move);
            }
        }
        self.current_index = 9;
        return None;
    }
}

struct TicTacToeGameState {
    active_player: Player,
    field: [FieldState; 9],
}

impl TicTacToeGameState {
    fn new() -> TicTacToeGameState {
        TicTacToeGameState {
            active_player: Player::Player1,
            field: [FieldState::EMPTY; 9],
        }
    }

    fn field_by_coords(self: &Self, x: usize, y: usize) -> &FieldState {
        &self.field[y * 3 + x]
    }
}

impl GameState for TicTacToeGameState {
    type Move = Move;
    type MoveIterator = MoveIterator;

    fn active_player(self: &Self) -> Player {
        self.active_player.clone()
    }

    fn move_iterator(self: &Self) -> Self::MoveIterator {
        MoveIterator::new()
    }

    fn apply_move(self: &mut Self, m: &Self::Move) -> MoveOutcome {
        // Switch player
        let active_player = self.active_player;
        self.active_player = active_player.other();

        let my_symbol = match active_player {
            Player::Player1 => FieldState::CROSS,
            Player::Player2 => FieldState::CIRCLE,
        };
        self.field[m.index] = my_symbol;
        let my_symbol = &self.field[m.index];
        // Winner?
        for x in 0..3 {
            if self.field_by_coords(x, 0) == my_symbol
                && self.field_by_coords(x, 1) == my_symbol
                && self.field_by_coords(x, 2) == my_symbol
            {
                return PlayerWon(active_player);
            }
        }
        for y in 0..3 {
            if self.field_by_coords(0, y) == my_symbol
                && self.field_by_coords(1, y) == my_symbol
                && self.field_by_coords(2, y) == my_symbol
            {
                return PlayerWon(active_player);
            }
        }
        if self.field_by_coords(0, 0) == my_symbol
            && self.field_by_coords(1, 1) == my_symbol
            && self.field_by_coords(2, 2) == my_symbol
        {
            return PlayerWon(active_player);
        }
        if self.field_by_coords(2, 0) == my_symbol
            && self.field_by_coords(1, 1) == my_symbol
            && self.field_by_coords(0, 2) == my_symbol
        {
            return PlayerWon(active_player);
        }

        if !self.field.contains(&FieldState::EMPTY) {
            return Tie;
        }
        return SwitchPlayer(self.active_player.clone());
    }

    fn reverse_move(self: &mut Self, m: &Self::Move) {
        self.active_player = self.active_player.other();
        self.field[m.index] = FieldState::EMPTY;
    }
}

impl MiniMaxGameState for TicTacToeGameState {
    fn evaluate(self: &Self, _: &Player) -> i64 {
        return 0;
    }
}

fn main() {
    fn find_move(m: &Move, gs: &TicTacToeGameState) -> Option<Move> {
        use turn_based_games::model::MoveIterator;
        let mut move_iterator = gs.move_iterator();
        while let Some(cm) = move_iterator.next(gs) {
            if cm.index == m.index {
                return Some(cm.clone());
            }
        }
        None
    }

    println!("Running A game of 2 tic tac toe player, each looking long into the future");
    let mut gs1 = TicTacToeGameState::new();
    let mut gs2 = TicTacToeGameState::new();
    let winner = tournament::ki_battle(&mut gs1, &mut gs2, 10, 10, find_move, find_move);
    println!("The winner is: {:?}", winner);

    for ki2_depth in 1..10 {
        println!("Running A game of 2 tic tac toe player, Player1 looking long into the future (10), Player2 looking {} in the future", ki2_depth);
        let mut gs1 = TicTacToeGameState::new();
        let mut gs2 = TicTacToeGameState::new();
        let winner = tournament::ki_battle(&mut gs1, &mut gs2, 10, ki2_depth, find_move, find_move);
        println!("The winner is: {:?}", winner);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use turn_based_games::negamax::negamax;

    #[test]
    fn empty_board_should_npt_find_win_move() {
        // Setup
        let mut tictactoe = TicTacToeGameState::new();

        // Act
        let (s, best_move) = negamax(&mut tictactoe, 10);

        // Test
        assert_eq!(s, 0);
    }

    #[test]
    fn find_none_loosing_move() {
        // Running on board, where cross took the top left corner. The center is the only non-loosing move!
        // Setup
        let mut tictactoe = TicTacToeGameState::new();
        tictactoe.apply_move(&Move { index: 0 });

        // Act
        let (s, best_move) = negamax(&mut tictactoe, 10);

        // Test
        assert_eq!(s, 0);
        assert_eq!(best_move, Some(4));
    }

    #[test]
    fn find_winning_move() {
        // Running on board, where cross took the top left corner and circle the top right corner. There are now multiple winning moves!
        let mut tictactoe = TicTacToeGameState::new();
        tictactoe.apply_move(&Move { index: 2 });

        // Act
        let (s, best_move) = negamax(&mut tictactoe, 10);

        // Test
        assert_eq!(s, i64::MAX - 1);
    }
}
