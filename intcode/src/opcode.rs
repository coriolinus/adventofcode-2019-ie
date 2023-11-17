use crate::{
    error::{Error, Result},
    Word,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum::FromRepr)]
#[repr(u8)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl Opcode {
    pub const fn parameter_count(self) -> usize {
        match self {
            Opcode::Input | Opcode::Output => 1,
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => 2,
            Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => 3,
            Opcode::Halt => 0,
        }
    }
}

impl TryFrom<Word> for Opcode {
    type Error = Error;

    fn try_from(value: Word) -> Result<Self> {
        value
            .try_into()
            .ok()
            .and_then(Self::from_repr)
            .ok_or(Error::UnknownOpcode(value))
    }
}
