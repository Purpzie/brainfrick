#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    Add(i8),
    Move(isize),
    Loop(Loop, usize),
    Output,
    Input,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Loop {
    Begin,
    End,
}

impl Loop {
    #[inline(always)]
    pub fn should_jump(self, mem: &crate::mem::Memory) -> bool {
        let c = mem.cell() == 0;
        match self {
            Self::Begin => c,
            Self::End => !c,
        }
    }
}
