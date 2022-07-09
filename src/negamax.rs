use crate::model::MoveOutcome;
use crate::model::{self, MoveIterator};

const MAX_SCORE: i64 = i64::MAX - 2;
/** Public interface if the negamax function, sets some defaults. See [negamax_impl]*/
pub fn negamax<GS>(gs: &mut GS, depth: i8) -> (i64, Option<GS::Move>)
where
    GS: model::MiniMaxGameState,
{
    negamax_impl(gs, depth, -MAX_SCORE - 1, MAX_SCORE + 1)
}

/** Implementation of the negamax algorithm (https://www.chessprogramming.org/Negamax)
 * based on the turn based model introduces in the model module.
*/
fn negamax_impl<GS>(
    gs: &mut GS,
    mut depth: i8,
    mut alpha: i64,
    beta: i64,
) -> (i64, Option<GS::Move>)
where
    GS: model::MiniMaxGameState,
{
    let active_player = gs.active_player();

    if depth == 0 {
        return (gs.evaluate(&active_player), None);
    }
    let mut best_score: i64 = -MAX_SCORE - 1;
    let mut best_move: Option<GS::Move> = None;

    let mut move_iterator = gs.move_iterator();
    while let Some(m) = move_iterator.next(gs) {
        let score = match gs.apply_move(&m) {
            MoveOutcome::PlayerWon(p) => {
                depth = 0;
                if p == active_player {
                    MAX_SCORE
                } else {
                    -MAX_SCORE
                }
            }
            MoveOutcome::Tie => 0,
            MoveOutcome::SwitchPlayer(_p) => -negamax_impl(gs, depth - 1, -beta, -alpha).0,
            MoveOutcome::ContinuePlayer(_p) => negamax_impl(gs, depth - 1, alpha, beta).0,
        };
        // Undo the move
        gs.reverse_move(m);

        // Adjust best score
        if score > best_score {
            best_score = score;
            best_move = Some(m.clone());
        }

        // If the depth was reduced to 0, stop!
        if depth == 0 {
            return (best_score, best_move);
        }

        // Adjust alpha
        if best_score > alpha {
            alpha = best_score;
        }

        // Alpha-beta pruning
        if alpha >= beta {
            return (alpha, best_move);
        }
    }
    return (best_score, best_move);
}

#[cfg(test)]
mod tests {
    use crate::model::MoveOutcome::{PlayerWon, SwitchPlayer};
    use crate::model::{self, MoveIterator};
    use crate::model::{MiniMaxGameState, MoveOutcome, Player};
    use crate::negamax::{negamax, MAX_SCORE};

    // Mock for a game model
    struct CenterScoreWinMovePathGame {
        win_moves: Vec<i32>,
        active_player: Player,
        current_moves: Vec<i32>,
        p1_won: bool,
        p2_won: bool,
        visitor: Option<fn(&Vec<i32>) -> ()>,
    }

    impl CenterScoreWinMovePathGame {
        fn new(
            win_moves: Vec<i32>,
            visitor: Option<fn(&Vec<i32>) -> ()>,
        ) -> CenterScoreWinMovePathGame {
            CenterScoreWinMovePathGame {
                win_moves,
                active_player: Player::Player1,
                current_moves: vec![0; 0],
                p1_won: false,
                p2_won: false,
                visitor,
            }
        }
    }

    struct CenterScoreMoveIterator {
        current_move: i32,
    }
    impl MoveIterator for CenterScoreMoveIterator {
        type Move = i32;
        type GameState = CenterScoreWinMovePathGame;

        fn next(&mut self, _gs: &Self::GameState) -> Option<&Self::Move> {
            self.current_move += 1;
            if self.current_move >= 20 {
                None
            } else {
                Some(&self.current_move)
            }
        }
    }

    impl model::GameState for CenterScoreWinMovePathGame {
        type Move = i32;
        type MoveIterator = CenterScoreMoveIterator;

        fn active_player(self: &Self) -> Player {
            self.active_player.clone()
        }

        fn move_iterator(self: &Self) -> Self::MoveIterator {
            CenterScoreMoveIterator { current_move: -1 }
        }

        fn apply_move(self: &mut Self, m: &Self::Move) -> MoveOutcome {
            self.current_moves.push(m.clone());
            if let Some(v) = self.visitor {
                v(&self.current_moves);
            }
            return if self.current_moves == self.win_moves {
                match self.active_player {
                    Player::Player1 => self.p1_won = true,
                    Player::Player2 => self.p2_won = true,
                }
                PlayerWon(self.active_player.clone())
            } else {
                self.active_player = self.active_player.other();
                SwitchPlayer(self.active_player.clone())
            };
        }

        fn reverse_move(self: &mut Self, _m: &Self::Move) {
            self.current_moves.pop();
            self.p1_won = false;
            self.p2_won = false;
            self.active_player = self.active_player.other();
        }
    }

    impl MiniMaxGameState for CenterScoreWinMovePathGame {
        fn evaluate(self: &Self, _player: &Player) -> i64 {
            let p1_score = if self.p1_won {
                MAX_SCORE
            } else {
                if self.p2_won {
                    -MAX_SCORE
                } else {
                    i64::from(
                        self.current_moves
                            .iter()
                            .step_by(2)
                            .fold(0, |acc, e| acc + 10 - (10 - e).abs()),
                    )
                }
            };
            return match self.active_player {
                Player::Player1 => p1_score,
                Player::Player2 => -p1_score,
            };
        }
    }

    #[test]
    /// Game model, where one move immediately wins. This move must be returned.
    fn choose_winning_move() {
        // Setup
        let mut gs = CenterScoreWinMovePathGame::new(vec![5], None);

        // Act
        let (_s, m) = negamax(&mut gs, 1);

        // Test
        assert_eq!(m, Some(5));
    }

    #[test]
    /// Game model, where at a depth of 2 a move wins.
    fn dont_choose_opponent_winning_move() {
        // Setup
        let mut gs = CenterScoreWinMovePathGame::new(vec![10, 1], None);

        // Act
        let (s, m) = negamax(&mut gs, 5);

        // Test
        println!("Score {}, move: {:?}", s, m);
        assert_eq!(m, Some(9));
    }

    #[test]
    fn choose_best_move_no_wins() {
        // Setup
        let mut gs = CenterScoreWinMovePathGame::new(vec![], None);

        // Act
        let (_s, m) = negamax(&mut gs, 5);

        // Test
        assert_eq!(m, Some(10));
    }

    #[test]
    fn test_alpha_beta_pruning() {
        // Setup
        let mut gs = CenterScoreWinMovePathGame::new(
            vec![],
            Some(|moves| {
                // If all moves are above the best move (10), the following moves may not be
                // Above it!
                for (index, m) in moves.iter().enumerate() {
                    if *m < 11 {
                        return;
                    }
                    if index > 0 && *m > 11 {
                        panic!("Moves should not be reached: {:?}", moves)
                    }
                }
            }),
        );

        // Act
        let (_, m) = negamax(&mut gs, 5);

        // Test
        assert_eq!(m, Some(10));
    }
}
