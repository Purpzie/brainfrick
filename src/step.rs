#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    Add(i8),
    Move(isize),
    Loop(LoopKind, usize),
    Output,
    Input,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopKind {
    Begin,
    End,
}

impl LoopKind {
    pub fn should_jump(self, mem: &crate::mem::Memory) -> bool {
        let c = mem.get_cell() == 0;
        match self {
            Self::Begin => c,
            Self::End => !c,
        }
    }
}
