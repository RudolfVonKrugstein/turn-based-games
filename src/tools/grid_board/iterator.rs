use crate::tools::grid_board::Board;
use crate::tools::grid_board::position::BoardPosition;

pub struct FieldIterator<'a, FieldContent> {
    pos: BoardPosition,
    col_dir: isize,
    row_dir: isize,
    field: &'a Board<FieldContent>
}

impl<'a, FieldContent> FieldIterator<'a, FieldContent> {
    pub fn new(board: &'a Board<FieldContent>, start_pos: BoardPosition, row_dir: isize, col_dir: isize)
        -> Self {
        FieldIterator {
            pos: start_pos,
            col_dir,
            row_dir,
            field: board,
        }
    }
}

impl<'a, FieldContent> Iterator for FieldIterator<'a, FieldContent> {
    type Item = &'a FieldContent;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.field.get(&self.pos);
        self.pos.step(self.row_dir, self.col_dir);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use more_asserts::*;

    #[test]
    fn iterator_on_empty_field() {
        // Setup
        let field = Board::new(10,10,|| 0);

        // Act
        let mut fields = Vec::new();
        for f in field.field_iterator(BoardPosition::new(0,1), 1, 0) {
            fields.push(f);
        }

        // Test
        fields.iter().for_each(
            |&item|
                assert_eq!(*item, 0)
        );
    }

    #[test]
    fn iterator_on_field_with_numbers() {
        // Setup
        let mut field = Board::new(10,10,|| 0);
        for i in 0..10 {
            field.set(&BoardPosition::new(i, i), i);
        }

        // Act
        let mut fields = Vec::new();
        for f in field.field_iterator(BoardPosition::new(9,9), -1, -1) {
            fields.push(f);
        }

        // Test
        fields.iter().enumerate().for_each(
            |(index, &item)|
                assert_eq!(*item, 9-index as isize)
        );
    }
}
