use std::time::Duration;
use crate::model::{MiniMaxGameState, MoveOutcome, Player};
use crate::negamax::negamax;

#[derive(Debug)]
pub struct Statistics {
    pub total_negamax_duration: Duration,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            total_negamax_duration: Duration::from_secs(0),
        }
    }
}

pub fn ki_battle<GS1, GS2, F2To1, F1To2>(
    gs1: &mut GS1,
    gs2: &mut GS2,
    ki1_depth: i8,
    ki2_depth: i8,
    find_gs1_move: F2To1,
    find_gs2_move: F1To2,
) -> (Option<Player>, Statistics, Statistics)
where
    GS1: MiniMaxGameState,
    GS2: MiniMaxGameState,
    F1To2: Fn(&GS1::Move, &GS2) -> Option<GS2::Move>,
    F2To1: Fn(&GS2::Move, &GS1) -> Option<GS1::Move>,
{
    let mut active_player = Player::Player1;
    let mut p1_statistics = Statistics::new();
    let mut p2_statistics = Statistics::new();
    loop {
        let outcome = match active_player {
            Player::Player1 => {
                let start = std::time::Instant::now();
                let (_s, m) = negamax(gs1, ki1_depth);
                p1_statistics.total_negamax_duration += start.elapsed();
                if let Some(the_move) = m {
                    gs2.apply_move(&find_gs2_move(&the_move, &gs2).unwrap());
                    gs1.apply_move(&the_move)
                } else {
                    panic!("No move found!");
                }
            }
            Player::Player2 => {
                let start = std::time::Instant::now();
                let (_s, m) = negamax(gs2, ki2_depth);
                p2_statistics.total_negamax_duration += start.elapsed();
                if let Some(the_move) = m {
                    gs1.apply_move(&find_gs1_move(&the_move, &gs1).unwrap());
                    gs2.apply_move(&the_move)
                } else {
                    panic!("No move found!");
                }
            }
        };
        match outcome {
            MoveOutcome::PlayerWon(p) => {
                return (Some(p), p1_statistics, p2_statistics);
            }
            MoveOutcome::Tie => {
                return (None, p1_statistics, p2_statistics);
            }
            MoveOutcome::SwitchPlayer(_) => {
                active_player = active_player.other();
            }
            MoveOutcome::ContinuePlayer(_) => {}
        }
    }
}
