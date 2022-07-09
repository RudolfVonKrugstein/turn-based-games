use crate::tools::grid_board::Board;
use crate::tools::grid_board::position::BoardPosition;

pub struct SlidingWindowIterator<'a, FieldContent> {
    front_pos: BoardPosition,
    end_pos: BoardPosition,
    current_length: usize,
    full_length: usize,
    col_step: isize,
    row_step: isize,
    board: &'a Board<FieldContent>
}

impl<'a, FieldContent> SlidingWindowIterator<'a, FieldContent> {
    pub fn new(board: &'a Board<FieldContent>, start_pos: BoardPosition, row_step: isize, col_step: isize, length: usize) -> SlidingWindowIterator<'a, FieldContent> {
        SlidingWindowIterator {
            front_pos: start_pos.clone(),
            end_pos: start_pos,
            current_length: 0,
            full_length: length,
            col_step,
            row_step,
            board
        }
    }
}

impl<'a, FieldContent> Iterator for SlidingWindowIterator<'a, FieldContent> {
    type Item=SlidingWindowStep<'a, FieldContent>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.board.get(&self.front_pos) {
            None => None,
            Some(new_field) => {
                // Update length
                let was_already_full = self.current_length == self.full_length;
                if !was_already_full {
                    self.current_length += 1;
                }
                let is_full = self.current_length == self.full_length;

                let result = SlidingWindowStep {
                    new_field,
                    removed_field: if was_already_full {
                        self.board.get(&self.end_pos)
                    } else {
                        None
                    },
                    length: self.current_length,
                    is_full,
                    font_pos: self.front_pos.clone(),
                    end_pos: self.end_pos.clone()
                };

                // Update positions
                self.front_pos.step(self.row_step, self.col_step);
                if was_already_full {
                    self.end_pos.step(self.row_step, self.col_step);
                }

                Some(result)
            }
        }
    }
}

pub struct SlidingWindowStep<'a, FieldContent> {
    // The field sliding into the window
    pub new_field: &'a FieldContent,
    // The field being removed from the window (if any)
    pub removed_field: Option<&'a FieldContent>,
    // The current length of the window
    pub length: usize,
    // Is the window full? (this is equal to length==full_length)
    pub is_full: bool,
    pub  font_pos: BoardPosition,
    pub end_pos: BoardPosition
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn slide_over_field_of_zeros() {
        // Setup
        let board = Board::new(10,10,|| 0);

        // Act
        let mut fields = Vec::new();
        for f in SlidingWindowIterator::new(&board,BoardPosition::new(0,1), 1, 0, 4) {
            fields.push(f);
        }

        // Test
        fields.iter().take(3).for_each (
            |item| {
                assert!(!item.is_full);
                assert_eq!(item.removed_field, None);
                assert_eq!(*item.new_field, 0);
            }
        );
        fields.iter().skip(3).take(1).for_each (
            |item| {
                assert!(item.is_full);
                assert_eq!(item.removed_field, None);
                assert_eq!(*item.new_field, 0);
            }
        );
        fields.iter().skip(4).for_each (
            |item| {
                assert!(item.is_full);
                assert_eq!(item.removed_field, Some(&0));
                assert_eq!(*item.new_field, 0);
            }
        );
    }

    #[test]
    fn slide_over_field_with_numbers() {
        // Setup
        let mut board = Board::new(10,10,|| 0);
        for i in 0..10 {
            board.set(&BoardPosition::new(i, i), i);
        }

        // Act
        let mut fields = Vec::new();
        for f in SlidingWindowIterator::new(&board,BoardPosition::new(9,9), -1, -1, 4) {
            fields.push(f);
        }

        // Test
        fields.iter().enumerate().take(3).for_each (
            |(step, item)| {
                assert!(!item.is_full);
                assert_eq!(item.removed_field, None);
                assert_eq!(*item.new_field, 9-step as isize);
            }
        );
        fields.iter().enumerate().skip(3).take(1).for_each (
            |(step, item)| {
                assert!(item.is_full);
                assert_eq!(item.removed_field, None);
                assert_eq!(*item.new_field, 6);
            }
        );
        fields.iter().enumerate().skip(4).for_each (
            |(step, item)| {
                assert!(item.is_full);
                assert_eq!(item.removed_field, Some(&(9+4-step as isize)));
                assert_eq!(*item.new_field, 9-step as isize);
            }
        );
    }
}
