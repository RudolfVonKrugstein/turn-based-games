use crate::tools::grid_board::iterator::FieldIterator;
use crate::tools::grid_board::sliding_window_iterator::SlidingWindowIterator;

pub mod position;
pub use position::BoardPosition;
pub mod iterator;
pub mod sliding_window_iterator;

// A Board for any game, putting its pieces on a regular grid.
pub struct Board<FieldContent> {
    fields: Vec<FieldContent>,
    cols: usize,
    rows: usize
}

impl<FieldContent> Board<FieldContent> {
    pub fn new(rows: usize, cols: usize, default_content: fn() ->FieldContent)
        -> Board<FieldContent> {
        Board {
            cols,
            rows,
            fields: std::iter::repeat_with(default_content).take(rows*cols).collect(),
        }
    }

    pub fn columns(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    // Set the content of a field
    pub fn set(&mut self, pos: &BoardPosition, value: FieldContent) {
        if let Some(index) = pos.linear_index(self.rows, self.cols) {
            self.fields[index] = value;
        } else {
            panic!("Invalid insert position");
        }
    }

    // Get the content of the specific field
    pub fn get(&self, pos: &BoardPosition) -> Option<&FieldContent> {
        pos.linear_index(self.rows, self.cols).map(|i| &self.fields[i])
    }

    pub fn field_iterator(&self, pos: BoardPosition, row_dir: isize, col_dir: isize)
        -> FieldIterator<FieldContent> {
        FieldIterator::new(
            self,
            pos,
         row_dir,
            col_dir,
        )
    }

    pub fn sliding_window_iterator(&self, pos: BoardPosition, row_dir: isize, col_dir: isize, window_size: usize)
                      -> SlidingWindowIterator<FieldContent> {
        SlidingWindowIterator::new(
            self,
            pos,
            row_dir,
            col_dir,
            window_size
        )
    }
}
