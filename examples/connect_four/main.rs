use turn_based_games::model::{GameState, MoveIterator};
use turn_based_games::tournament::ki_battle;
use crate::game_state::ConnectFourGameState;
use crate::r#move::Move;

mod r#move;
mod game_state;

fn main() {
    // Tournament of 2 connect 4 players
    let mut gs1 = ConnectFourGameState::new();
    let mut gs2 = ConnectFourGameState::new();

    fn find_move(m: &Move, gs :&ConnectFourGameState) -> Option<Move> {
        let mut i = gs.move_iterator();
        while let Some(new_m) = i.next(gs) {
            if m.col == new_m.col {
                return Some(new_m.clone());
            }
        }
        None
    }

    gs1.print_as_ascii();

    let winner = ki_battle(&mut gs1, &mut gs2, 6, 6, find_move, find_move);
    println!("The winner is: {:?}", winner);
}
