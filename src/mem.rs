use std::{collections::VecDeque, convert::TryFrom};

// to the user, the pointer appears to be able to become negative
// however, it is just a normal usize pointer with an offset stored separately
pub(crate) struct Memory {
    cells: VecDeque<u8>,
    pointer: usize,
    offset: usize,
}

impl Memory {
    pub fn new() -> Self {
        let mut cells = VecDeque::with_capacity(1);
        cells.push_back(0);
        Self {
            cells,
            pointer: 0,
            offset: 0,
        }
    }

    pub fn add(&mut self, amount: i8) {
        let c = &mut self.cells[self.pointer];
        *c = c.wrapping_add(amount as u8);
    }

    pub fn get_cell(&self) -> u8 {
        self.cells[self.pointer]
    }

    pub fn set_cell(&mut self, c: u8) {
        self.cells[self.pointer] = c;
    }

    pub fn move_pointer(&mut self, amount: isize) {
        if amount >= 0 {
            // moving right!
            self.pointer += amount as usize;
            // expand VecDeque as needed
            if self.pointer >= self.cells.len() {
                self.cells.resize(self.pointer + 1, 0);
            }
        } else {
            // moving left!
            let amount = (-amount) as usize;
            if self.pointer >= amount {
                // safe to subtract
                self.pointer -= amount;
            } else {
                // not safe to subtract, we'll need to do some trickery here
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

    // technically, you can move usize::MAX cells away from the center. that's bigger than
    // isize::MAX, so we have to use i128 (although nobody is crazy enough for this probably)
    pub fn append_debug(&self, output: &mut Vec<u8>) {
        output.append(
            &mut format!(
                "[{},{}]",
                i128::try_from(self.pointer).unwrap() - i128::try_from(self.offset).unwrap(),
                self.get_cell(),
            )
            .into_bytes(),
        );
    }
}
