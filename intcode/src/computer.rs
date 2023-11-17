use crate::{
    error::Error, error::Result, mem_idx::MemIdx as _, memory::Memory, opcode::Opcode, Word,
};

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
        let slice = self.memory.get(low..high).ok_or(Error::Underflow {
            idx: high,
            len: self.memory.len(),
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
        let opcode: Opcode = self.memory.ix(self.instruction_pointer)?.try_into()?;
        let result = match opcode {
            Opcode::Add => {
                let [ai, bi, outi] = self.parameters()?;
                let a = self.memory.ix(ai)?;
                let b = self.memory.ix(bi)?;
                let out = self.memory.ix_mut(outi)?;

                *out = a + b;
                Ok(())
            }
            Opcode::Multiply => {
                let [ai, bi, outi] = self.parameters()?;
                let a = self.memory.ix(ai)?;
                let b = self.memory.ix(bi)?;
                let out = self.memory.ix_mut(outi)?;

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
        self.memory.into_inner()
    }
}
