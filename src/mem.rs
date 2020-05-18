use std::collections::VecDeque;

// to the user, the pointer appears to be able to become negative
// however, it is just a normal usize pointer with an offset stored separately
pub(crate) struct Memory {
    cells: VecDeque<u8>,
    pointer: usize,
    offset: usize,
}

impl Memory {
    #[inline]
    pub fn new() -> Self {
        let mut cells = VecDeque::with_capacity(1);
        cells.push_back(0);

        Self {
            cells,
            pointer: 0,
            offset: 0,
        }
    }

    #[inline]
    pub fn add(&mut self, amount: i8) {
        let c = &mut self.cells[self.pointer];
        *c = c.wrapping_add(amount as u8);
    }

    #[inline]
    pub fn cell(&self) -> u8 {
        self.cells[self.pointer]
    }

    #[inline]
    pub fn set_cell(&mut self, c: u8) {
        self.cells[self.pointer] = c;
    }

    #[inline]
    pub fn move_pointer(&mut self, amount: isize) {
        if amount >= 0 {
            self.pointer += amount as usize;
            if self.pointer >= self.cells.len() {
                self.cells.resize(self.pointer + 1, 0);
            }
        } else {
            let amount = (-amount) as usize;
            if amount <= self.pointer {
                self.pointer -= amount;
            } else {
                let offset = amount - self.pointer;
                self.pointer = 0;
                self.offset += offset;
                self.cells.reserve(offset);
                for _ in 0..offset {
                    self.cells.push_front(0);
                }
            }
        }
    }

    // technically, you can move usize::MAX cells away from the center
    // that is bigger than isize::MAX, so return a usize with a bool indicating its sign
    #[inline]
    pub fn pointer(&self) -> (usize, bool) {
        if self.pointer >= self.offset {
            (self.pointer - self.offset, true)
        } else {
            (self.offset - self.pointer, false)
        }
    }
}
