pub type Word = i64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum::FromRepr)]
#[repr(u8)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl Opcode {
    const fn parameter_count(self) -> usize {
        match self {
            Opcode::Add | Opcode::Multiply => 3,
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

#[derive(Debug, PartialEq, Eq, Clone, derive_more::From, derive_more::Into)]
pub struct Memory(Vec<Word>);

impl From<&[Word]> for Memory {
    fn from(value: &[Word]) -> Self {
        Self(value.to_owned())
    }
}

impl<const N: usize> From<[Word; N]> for Memory {
    fn from(value: [Word; N]) -> Self {
        Self(value.into())
    }
}

trait MemIdx<Idx> {
    fn get(&self, idx: Idx) -> Result<Word>;
    fn get_mut(&mut self, idx: Idx) -> Result<&mut Word>;
}

impl MemIdx<usize> for Memory {
    #[inline]
    fn get(&self, idx: usize) -> Result<Word> {
        self.0.get(idx).copied().ok_or(Error::Underflow {
            idx,
            len: self.0.len(),
        })
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> Result<&mut Word> {
        let len = self.0.len();
        self.0.get_mut(idx).ok_or(Error::Underflow { idx, len })
    }
}

impl MemIdx<Word> for Memory {
    #[inline]
    fn get(&self, idx: Word) -> Result<Word> {
        let idx: usize = idx.try_into().map_err(|_| Error::IndexFailed(idx))?;
        self.get(idx)
    }

    #[inline]
    fn get_mut(&mut self, idx: Word) -> Result<&mut Word> {
        let idx: usize = idx.try_into().map_err(|_| Error::IndexFailed(idx))?;
        self.get_mut(idx)
    }
}

pub struct Computer {
    memory: Memory,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new(program: impl Into<Memory>) -> Self {
        Self {
            memory: program.into(),
            instruction_pointer: 0,
        }
    }

    /// Get `N` parameters for the current instruction.
    ///
    /// Does not advance the instruction pointer.
    fn parameters<const N: usize>(&self) -> Result<[Word; N]> {
        let low = self.instruction_pointer + 1;
        let high = low + N;
        let slice = self.memory.0.get(low..high).ok_or(Error::Underflow {
            idx: high,
            len: self.memory.0.len(),
        })?;
        Ok(slice
            .try_into()
            .expect("`fn operands()` produces an appropriately sized slice"))
    }

    /// Execute the opcode at the current instruction pointer.
    ///
    /// If the instruction succeeded, increment the instruction pointer appropriately.
    /// Otherwise, leave it, for debugging purposes.
    fn step(&mut self) -> Result<()> {
        let opcode: Opcode = self.memory.get(self.instruction_pointer)?.try_into()?;
        let result = match opcode {
            Opcode::Add => {
                let [ai, bi, outi] = self.parameters()?;
                let a = self.memory.get(ai)?;
                let b = self.memory.get(bi)?;
                let out = self.memory.get_mut(outi)?;

                *out = a + b;
                Ok(())
            }
            Opcode::Multiply => {
                let [ai, bi, outi] = self.parameters()?;
                let a = self.memory.get(ai)?;
                let b = self.memory.get(bi)?;
                let out = self.memory.get_mut(outi)?;

                *out = a * b;
                Ok(())
            }
            Opcode::Halt => Err(Error::Halt(self.instruction_pointer)),
        };

        if result.is_ok() {
            self.instruction_pointer += 1 + opcode.parameter_count();
        }

        result
    }

    /// Execute the contained program until completion.
    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.step() {
                Ok(()) => {}
                Err(Error::Halt(_)) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }

    pub fn into_memory(self) -> Vec<Word> {
        self.memory.0
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown opcode: `{0}`")]
    UnknownOpcode(Word),
    #[error("attempted to read position {idx} but length is {len}")]
    Underflow { idx: usize, len: usize },
    #[error("failed to convert `Int` value ({0}) to `usize` for indexing")]
    IndexFailed(Word),
    #[error("encountered Halt opcode at instruction pointer `{0}`")]
    Halt(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_example() {
        let example = [1_i64, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut computer = Computer::new(example);
        computer.run().unwrap();
        assert_eq!(computer.memory.0[0], 3500);
    }
}
