use turn_based_games::model::{GameState, MiniMaxGameState, MoveOutcome, Player};
use turn_based_games::tools::grid_board;
use turn_based_games::tools::grid_board::BoardPosition;
use crate::r#move::{Move, MoveIterator};

pub struct ConnectFourGameState {
    active_player: Player,
    board: grid_board::Board<ConnectFourField>,
    col_heights: [usize;7],
}

pub enum ConnectFourField {
    Empty,
    Piece(Player)
}

#[derive(PartialEq, Debug, Clone)]
pub enum OpportunityType {
    L0,
    L1,
    L2,
    L3,
    L4
}

static L1_SCORE: i64 = 1;
static L2_SCORE: i64 = 1 << 8;
static L3_SCORE: i64 = 1 << 16;
static L4_SCORE: i64 = 1 << 24;

impl ConnectFourGameState {
    pub fn new() -> ConnectFourGameState {
        ConnectFourGameState {
            active_player: Player::Player1,
            board: grid_board::Board::new(6, 7, || ConnectFourField::Empty),
            col_heights: [0;7],
        }
    }

    pub fn print_as_ascii(&self) {
        let mut output = [[' ';15];7];
        for i in 0..6 {
            output[i][0] = '|';
            output[i][14] = '|';
        }
        output[6][0] = '+';
        output[6][14] = '+';
        for i in 1..14 {
            output[6][i] = '-';
        }

        for x in 0..7 {
            for y in 0..6 {
                output[5-y][2*x+1] = match self.board.get(
                    &BoardPosition::new(x as isize, y as isize)
                ) {
                    None => {
                        ' '
                    }
                    Some(ConnectFourField::Empty) => {
                        ' '
                    }
                    Some(ConnectFourField::Piece(Player::Player1)) => {
                        'X'
                    }
                    Some(ConnectFourField::Piece(Player::Player2)) => {
                        'O'
                    }
                }
            }
        }

        for l in 0..7 {
            let line: String = output[l].iter().collect();
            println!("{}", line);
        }
    }

    fn is_tie(&self) -> bool {
        // No win, is it a tie? No, if any row as free fields.
        for col_height in &self.col_heights {
            if *col_height < 6 {
                return false;
            }
        }
        return true;
    }

    pub fn valid_move(self: &Self, m: &Move) -> bool {
        m.col < 7 && self.col_heights[m.col as usize] < 6
    }

    fn best_opportunity_for_field(&self, pos: BoardPosition, player: Player) -> OpportunityType {
        let mut best_opportunity = OpportunityType::L0;
        for dir in [
            (0, 1),
            (1, 0),
            (1, 1),
            (1, -1)] {
            // Find the start position, iterate as much (max 3) as we can opposite to
            // Start positions
            let mut start_offset = 0;
            for (index, field) in self.board.field_iterator(
                pos,
                -dir.0, -dir.1)
                .enumerate().skip(1).take(3) {
                match field {
                    ConnectFourField::Empty => {}
                    ConnectFourField::Piece(p) => {
                        if *p == player.other() {
                            break;
                        }
                    }
                }
                start_offset = -(index as isize);
            }
            // Ok, start position is the offset
            let mut start_pos = pos.clone();
            start_pos.step(start_offset * dir.0,start_offset * dir.1);
            let mut player_fields = 0;
            for window_step in self.board.sliding_window_iterator(start_pos, dir.0, dir.1, 4)
                .take((7 - start_offset - 3) as usize) {
                if let Some(removed_field) = window_step.removed_field {
                    match removed_field {
                        ConnectFourField::Empty => {}
                        ConnectFourField::Piece(p) => {
                            if *p == player.other() {
                                panic!("This should not happen");
                            } else {
                                player_fields -= 1;
                            }
                        }
                    }
                }
                match window_step.new_field {
                    ConnectFourField::Empty => {}
                    ConnectFourField::Piece(p) => {
                        if *p == player.other() {
                            break;
                        } else {
                            player_fields += 1;
                        }
                    }
                }
                if player_fields >= 3 {
                    return OpportunityType::L3;
                }
                if player_fields >= 2 {
                    best_opportunity = OpportunityType::L2;
                } else {
                    if player_fields >= 1 && best_opportunity == OpportunityType::L0 {
                        best_opportunity = OpportunityType::L1;
                    }
                }
            }
        }
        best_opportunity
    }

    fn points_for_opportunity(&self, oppertunity: &OpportunityType) -> i64 {
        match oppertunity {
            OpportunityType::L0 => 0,
            OpportunityType::L1 => L1_SCORE,
            OpportunityType::L2 => L2_SCORE,
            OpportunityType::L3 => L3_SCORE,
            OpportunityType::L4 => L4_SCORE,
        }
    }

    fn count_fields_for_player_in_directory(
        &self,
        start :&BoardPosition,
        col_dir: isize,
        row_dir: isize,
        player: Player,
    ) -> usize {
        let mut res = 0;
        for field in self.board.field_iterator(start.clone(), row_dir, col_dir).take(4) {
            match field {
                ConnectFourField::Empty => {
                    break;
                }
                ConnectFourField::Piece(p) => {
                    if *p == player {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

impl GameState for ConnectFourGameState {
    type Move=Move;
    type MoveIterator=MoveIterator;

    fn active_player(self: &Self) -> Player {
        self.active_player.clone()
    }

    fn move_iterator(self: &Self) -> Self::MoveIterator {
        let mut ms = Vec::new();
        for col in 0..7 {
            if self.valid_move(&Move{col, opp_type: OpportunityType::L0}) {
                ms.push(Move{
                    col,
                    opp_type: self.best_opportunity_for_field(BoardPosition::new(col as isize, self.col_heights[col as usize] as isize), self.active_player),
                });
                // Sort the moves so that the most promising are used first
                ms.sort_by(|a,b| {
                    self.points_for_opportunity(&b.opp_type).cmp(&self.points_for_opportunity(&a.opp_type))
                });
            }
        }
        MoveIterator {
            current_moves: ms,
            index: 0,
        }
    }

    fn apply_move(self: &mut Self, m: &Self::Move) -> MoveOutcome {
        let insert_pos = BoardPosition::new(m.col as isize, self.col_heights[m.col as usize] as isize);

        self.board.set(&insert_pos, ConnectFourField::Piece(self.active_player));
        self.col_heights[m.col as usize] += 1;
        self.active_player = self.active_player.other();

        // Look for win
        if m.opp_type == OpportunityType::L3 || m.opp_type == OpportunityType::L4 {
            return MoveOutcome::PlayerWon(self.active_player.other());
        }

        // No win, is it a tie?
        if self.is_tie() {
            MoveOutcome::Tie
        } else {
            MoveOutcome::SwitchPlayer(self.active_player)
        }
    }

    fn reverse_move(self: &mut Self, m: &Self::Move) {
        self.active_player = self.active_player.other();
        self.col_heights[m.col as usize] -= 1;
        let remove_pos = BoardPosition::new(m.col as isize, self.col_heights[m.col as usize] as isize);

        self.board.set(&remove_pos, ConnectFourField::Empty);
    }
}

impl MiniMaxGameState for ConnectFourGameState {
    fn evaluate(self: &Self, player: &Player) -> i64 {
        let mut p1_points = 0;
        let mut p2_points = 0;
        for col in 0..self.board.columns() {
            let mut last_p1_oppertunity = OpportunityType::L0;
            let mut last_p2_oppertunity = OpportunityType::L0;
            for i in self.col_heights[col]..self.board.rows() {
                let p1_oppertunity = self.best_opportunity_for_field(BoardPosition::new(col as isize, i as isize), Player::Player1);
                let p2_oppertunity = self.best_opportunity_for_field(BoardPosition::new(col as isize, i as isize), Player::Player2);

                if p1_oppertunity == OpportunityType::L3 && last_p1_oppertunity == OpportunityType::L3 {
                    p1_points += self.points_for_opportunity(&OpportunityType::L4);
                    break; // No need to look up this way, this will stop here
                }
                if p2_oppertunity == OpportunityType::L3 && last_p2_oppertunity == OpportunityType::L3 {
                    p2_points += self.points_for_opportunity(&OpportunityType::L4);
                    break; // No need to look up this way, this will stop here
                }

                // Add the opportunity points!
                if last_p2_oppertunity != OpportunityType::L3 { // Skip if opponent wold have won!
                    p1_points += self.points_for_opportunity(&p1_oppertunity);
                }
                if last_p1_oppertunity != OpportunityType::L3 { // Skip if opponent wold have won!
                    p2_points += self.points_for_opportunity(&p2_oppertunity);
                }

                last_p1_oppertunity = p1_oppertunity;
                last_p2_oppertunity = p2_oppertunity;
            }
        }
        if *player == Player::Player1 {
            p1_points - p2_points
        } else {
            p2_points - p1_points
        }
    }
}

#[cfg(test)]
mod test {
    use more_asserts::*;
    use turn_based_games::model::MoveIterator;
    use super::*;

    fn find_move_by_col(gs: &ConnectFourGameState, col: usize) -> Option<Move> {
        let mut move_iterator = gs.move_iterator();
        while let Some(m) = move_iterator.next(&gs) {
            if m.col == col {
                return Some(m.clone());
            }
        };
        None
    }

    #[test]
    fn single_piece_points() {
        // Setup
        let mut gs = ConnectFourGameState::new();

        // Empty Field
        assert_eq!(gs.evaluate(&Player::Player1), 0);

        // One single piece
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        // Should be exactly 15 L1 Scores
        assert_eq!(gs.evaluate(&Player::Player1), 15 * L1_SCORE);
    }

    #[test]
    fn two_pieces_points() {
        // Setup
        let mut gs = ConnectFourGameState::new();

        // One single piece
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());

        // Now it should look like this:
        //
        //       O
        // _ _ X O _ _ _
        // The score should be bigger than  2 L2_Scores, but less than 3 L2 Socres.
        assert_ge!(gs.evaluate(&Player::Player1), L2_SCORE * 2);
        assert_lt!(gs.evaluate(&Player::Player1), L2_SCORE * 3);
    }

    #[test]
    fn three_pieces_vertical_points() {
        // Setup
        let mut gs = ConnectFourGameState::new();

        // 3 pieces
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 4).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());

        // Now it should look like this:
        //       O
        //       O
        // _ _ X O X _ _

        // The points should be
        // 1 L3
        // 1 L2
        // 19 L1
        // -6 L1 (of opponent)
        assert_eq!(gs.evaluate(&Player::Player1), L3_SCORE  + L2_SCORE + 13 * L1_SCORE);
    }

    #[test]
    fn three_pieces_horizontal_points() {
        // Setup
        let mut gs = ConnectFourGameState::new();

        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 0).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 6).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 4).unwrap());

        // Now it should look like this:
        //
        //
        // X _ O O O _ X
        // 2 Of Length 3
        // 19 Of Length 1
        // -11 of Length 1 (opponent)
        assert_eq!(gs.evaluate(&Player::Player1), 2 * L3_SCORE + 8 * L1_SCORE);
    }

    #[test]
    fn three_pieces_upwards_line_points() {
        // Setup
        let mut gs = ConnectFourGameState::new();

        // One single piece
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 4).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        // Now it should look like this:
        //     X
        //     O X
        // _ _ O O X _ _
        // (we now look from the perspective of X=Player2)
        // 1 with lenght 3
        // 1 of length 2
        // -4 of length 2 (from opponent)
        // 20 of length 1
        // -6 of lenght 1 (from opponent)
        assert_eq!(gs.evaluate(&Player::Player2), L3_SCORE -3 * L2_SCORE + 14 *L1_SCORE); }

    #[test]
    fn three_pieces_downwards_line_points() {
        // Setup
        let mut gs = ConnectFourGameState::new();

        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 4).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 4).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 4).unwrap());
        // Now it should look like this:
        //         X
        //       X O
        // _ _ X O O _ _
        // (we now look from the perspective of X=Player2)
        // 1 with length 3
        // 1 of length 2
        // -4 of length 2 (from opponent)
        // 19 of length 1
        // -6 of length 1 (from opponent)
        assert_eq!(gs.evaluate(&Player::Player2), L3_SCORE -3 * L2_SCORE + 14 *L1_SCORE);
    }

    #[test]
    fn two_operatunities_of_three_pieces_above_each_other() {
        // If there are 3 opportunities of L3 above each other, they create a L4 opportunity
        // (its a potential dilemma for the opponent)

        // Setup
        let mut gs = ConnectFourGameState::new();

        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&&find_move_by_col(&gs, 4).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 1).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 5).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 1).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 6).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 1).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 1).unwrap());
        // Now it should look like this:
        //   X
        //   O X
        //   O O
        // _ O O _ X X X
        // (we now look from the perspective of X=Player2)
        // 1 with L4
        // 1 with L3
        // 1 of length 2
        // -7 of length 2 (from opponent)
        // 20 of length 1
        // -8 of lenght 1 (from opponent
        assert_eq!(gs.evaluate(&Player::Player2), L4_SCORE + L3_SCORE - 6 * L2_SCORE + 12 *L1_SCORE);
    }

    #[test]
    fn no_opportunity_above_opportunity_of_opponent() {
        // If the opponent has an opportunity directly below our opportunity, ours does not count
        // Setup
        let mut gs = ConnectFourGameState::new();

        gs.apply_move(&find_move_by_col(&gs, 0).unwrap());
        gs.apply_move(&&find_move_by_col(&gs, 0).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 1).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 1).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 2).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        gs.apply_move(&find_move_by_col(&gs, 3).unwrap());
        // Now it should look like this:
        //
        //
        // X X X
        // O O O _ _ _ _
        // (we now look from the perspective of X=Player2)
        // 0 with L3
        // -1 with L3 (opponent!)
        // 2 of length 2
        // -2 of length 2 (from opponent)
        // 19 of length 1
        // -3 of lenght 1 (from opponent
        assert_eq!(gs.evaluate(&Player::Player2), -L3_SCORE + 16 * L1_SCORE);
    }
}
