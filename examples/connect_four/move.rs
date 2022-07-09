use crate::game_state::{ConnectFourGameState, OpportunityType};

#[derive(Clone)]
pub struct Move {
    pub opp_type: OpportunityType,
    pub col: usize
}

#[derive(Clone)]
pub struct MoveIterator {
    pub current_moves: Vec<Move>,
    pub index: usize,
}

impl turn_based_games::model::MoveIterator for MoveIterator {
    type Move = Move;
    type GameState = ConnectFourGameState;

    fn next(&mut self, gs: &Self::GameState) -> Option<&Self::Move> {
        let result = self.current_moves.get(self.index);
        self.index += 1;
        result
    }
}
