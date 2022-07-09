// A position on the regular board, so basically its coordinates.

#[derive(Clone, Copy)]
pub struct BoardPosition {
    x: isize,
    y: isize
}

impl BoardPosition {
    pub fn new(x: isize, y: isize) ->BoardPosition {
        BoardPosition {
            x, y
        }
    }

    pub fn step(&mut self, dx: isize, dy: isize) {
        self.x += dx;
        self.y += dy;
    }

    pub fn linear_index(&self, rows: usize, cols: usize) -> Option<usize> {
        if self.x < 0 || self.y < 0 || self.x >= cols as isize || self.y >= rows as isize {
            None
        } else {
            Some(self.x as usize + self.y as usize * cols)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        // Setup
        let mut p = BoardPosition::new(1,1);

        // Act
        p.step(-1, -1);

        // Test
        assert_eq!(p.linear_index(10, 10), Some(0));
    }

    fn linear_index() {
        // Setup
        let p = BoardPosition::new(5,5);

        // Act
        let index = p.linear_index(10,10);

        // Test
        assert_eq!(index, Some(55));
    }

    fn invalid_linear_index() {
        assert_eq!(BoardPosition::new(-1,0).linear_index(10,10), None);
        assert_eq!(BoardPosition::new(0,-1).linear_index(10,10), None);
        assert_eq!(BoardPosition::new(10,0).linear_index(10,10), None);
        assert_eq!(BoardPosition::new(0,10).linear_index(10,10), None);
    }
}
