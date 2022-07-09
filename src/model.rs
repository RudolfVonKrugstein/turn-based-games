/** The current state of a game.
 *
 * It stores the complete (abstract) state of the game, for example the location of all
 * the pieces in the game. The main implementation exposes the function to generate,
 * apply and reverse moves. If a move is reversed the game state _should_ be the same as before:
 ```
  use turn_based_games::model::GameState;#[doc(hidden)]
  use turn_based_games::model::MoveIterator;#[doc(hidden)]
  fn apply_reverse_moves<GS: GameState>(gs: &mut GS) {
      let mut move_iterator = gs.move_iterator();
      while let Some(m) = move_iterator.next(gs) {
          gs.apply_move(m);
          gs.reverse_move(m);
      }
  }
  ```
*/
pub trait GameState {
    type Move: Clone;
    type MoveIterator: MoveIterator<Move = Self::Move, GameState = Self>;

    fn active_player(self: &Self) -> Player;
    fn move_iterator(self: &Self) -> Self::MoveIterator;
    fn apply_move(self: &mut Self, m: &Self::Move) -> MoveOutcome;
    fn reverse_move(self: &mut Self, m: &Self::Move);
}

/** The MoveIterator allows to iterate over all possible moves for a GameState. It holds
 * the current iterating state.
 *
 * We do not use a normal iterator, because the GameState is modified while we are iterating
 * over the moves (that is needed for the minimax/negamax algorithm).
 *
 * If you want you can generate all moves in advance and store an index
 * like this:
 *
 ```
use std::slice::Iter;

#[derive(Clone)]
struct MyMove {}

struct MyMoveIterator {
    moves: Vec<MyMove>,
    index: usize,
}

impl MoveIterator for MyMoveIterator {
    type Move=MyMove;
    type GameState=MyGameState;
    fn next(&mut self, gs :&MyGameState) -> Option<&MyMove> {
        self.index += 1;
        self.moves.get(self.index - 1)
    }
}

struct MyGameState {};#[doc(hidden)]
use turn_based_games::model::{GameState, MoveIterator, MoveOutcome, Player};#[doc(hidden)]

impl GameState for MyGameState {
    type Move = MyMove;
    type MoveIterator = MyMoveIterator;
    fn move_iterator(&self) -> MyMoveIterator {
        let moves = Vec::new(); // Generate the moves in a vector
        MyMoveIterator {
            moves,
            index: 0,
        }
    }
    fn active_player(self: &Self) -> Player {
        todo!()
    }
    fn apply_move(self: &mut Self, m: &Self::Move) -> MoveOutcome {
        todo!()
    }
    fn reverse_move(self: &mut Self, m: &Self::Move) {
        todo!()
    }
}
 ```
 *
 * But the Idea of the MoveIterator is, that not all moves are generated in advance. Becuase minimax with
 * alpha-beta pruning may cur some leaves and the moves never have to be generated.
 */

pub trait MoveIterator {
    type Move: Clone;
    type GameState;
    fn next(&mut self, gs: &Self::GameState) -> Option<&Self::Move>;
}

/** Allows a GameState to be used for a MiniMax based AI.
 * MiniMax bases on a evaluation function, which tells MiniMax how good a state is for
 * a particular player.
 * The quality of the evaluation function is essential for the strength of the resulting
 * AI.
*/
pub trait MiniMaxGameState: GameState {
    fn evaluate(self: &Self, player: &Player) -> i64;
}

/** All the players in the game. Since this is for turn based 2 player games,
 *  there are exactly 2 players!
 */
#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn other(self: &Self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

/** All the results a move can have!*/
pub enum MoveOutcome {
    /// A player has won
    PlayerWon(Player),
    /// Game ended in a Tie
    Tie,
    /// Its the other players turn now
    SwitchPlayer(Player),
    /// The same player should make the next move,
    ContinuePlayer(Player),
}
