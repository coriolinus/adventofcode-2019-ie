use crate::{
    error::Error,
    error::Result,
    instruction::{Instruction, ParameterModes},
    mem_idx::MemIdx as _,
    memory::Memory,
    opcode::Opcode,
    parameters::Parameters,
    Word,
};

pub struct Computer {
    pub(crate) memory: Memory,
    pub(crate) instruction_pointer: usize,
}

impl Computer {
    pub fn new(program: impl Into<Memory>) -> Self {
        Self {
            memory: program.into(),
            instruction_pointer: 0,
        }
    }

    /// Get `N` raw parameters for the current instruction.
    ///
    /// This means that we have not yet applied the parameter modes to the parameters.
    ///
    /// Does not advance the instruction pointer.
    pub(crate) fn raw_parameters<const N: usize>(&self) -> Result<[Word; N]> {
        let low = self.instruction_pointer + 1;
        let high = low + N;
        let slice = self.memory.get(low..high).ok_or(Error::Underflow {
            idx: high,
            len: self.memory.len(),
        })?;
        Ok(slice
            .try_into()
            .expect("`fn raw_parameters()` produces an appropriately sized slice"))
    }

    pub(crate) fn parameters<'a, P>(&'a mut self, modes: ParameterModes) -> Result<P>
    where
        P: Parameters<'a>,
    {
        P::apply(self, modes)
    }

    /// Execute the opcode at the current instruction pointer.
    ///
    /// If the instruction succeeded, increment the instruction pointer appropriately.
    /// Otherwise, leave it, for debugging purposes.
    pub(crate) fn step(&mut self) -> Result<()> {
        let instruction: Instruction = self.memory.ix(self.instruction_pointer)?.try_into()?;
        let result = match instruction.opcode {
            Opcode::Add => {
                let (a, b, out): (_, _, &mut _) = self.parameters(instruction.modes)?;
                *out = a + b;
                Ok(())
            }
            Opcode::Multiply => {
                let (a, b, out): (_, _, &mut _) = self.parameters(instruction.modes)?;
                *out = a * b;
                Ok(())
            }
            Opcode::Halt => Err(Error::Halt(self.instruction_pointer)),
        };

        if result.is_ok() {
            self.instruction_pointer += 1 + instruction.opcode.parameter_count();
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
