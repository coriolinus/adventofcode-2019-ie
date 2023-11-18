use crate::{
    error::Result, instruction::ParameterModes, mem_idx::MemIdx as _,
    parameter_mode::ParameterMode, Computer, Error, Word,
};

pub(crate) trait Parameters<'a>: Sized {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self>;
}

impl<'a> Parameters<'a> for Word {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        <(Word,)>::apply(computer, modes).map(|word_tuple| word_tuple.0)
    }
}

impl<'a> Parameters<'a> for &'a mut Word {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        <(&mut Word,)>::apply(computer, modes).map(|word_tuple| word_tuple.0)
    }
}

impl<'a> Parameters<'a> for () {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        _modes: ParameterModes,
    ) -> Result<Self> {
        let _raw = computer.raw_parameters::<0>()?;
        Ok(())
    }
}

impl<'a> Parameters<'a> for (Word,) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<1>()?;
        let i0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        Ok((i0,))
    }
}

impl<'a> Parameters<'a> for (Word, Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<2>()?;
        let v0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        let v1 = match modes[1] {
            ParameterMode::Position => computer.memory.ix(raw[1])?,
            ParameterMode::Immediate => raw[1],
        };
        Ok((v0, v1))
    }
}

impl<'a> Parameters<'a> for (Word, Word, Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<3>()?;
        let v0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        let v1 = match modes[1] {
            ParameterMode::Position => computer.memory.ix(raw[1])?,
            ParameterMode::Immediate => raw[1],
        };
        let v2 = match modes[2] {
            ParameterMode::Position => computer.memory.ix(raw[2])?,
            ParameterMode::Immediate => raw[2],
        };
        Ok((v0, v1, v2))
    }
}

impl<'a> Parameters<'a> for (Word, Word, Word, Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<4>()?;
        let v0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        let v1 = match modes[1] {
            ParameterMode::Position => computer.memory.ix(raw[1])?,
            ParameterMode::Immediate => raw[1],
        };
        let v2 = match modes[2] {
            ParameterMode::Position => computer.memory.ix(raw[2])?,
            ParameterMode::Immediate => raw[2],
        };
        let v3 = match modes[3] {
            ParameterMode::Position => computer.memory.ix(raw[3])?,
            ParameterMode::Immediate => raw[3],
        };
        Ok((v0, v1, v2, v3))
    }
}

impl<'a> Parameters<'a> for (&'a mut Word,) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<1>()?;
        let i0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix_mut(raw[0])?,
            ParameterMode::Immediate => return Err(Error::ImmediateWrite),
        };
        Ok((i0,))
    }
}

impl<'a> Parameters<'a> for (Word, &'a mut Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<2>()?;
        let v0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        let v1 = match modes[1] {
            ParameterMode::Position => computer.memory.ix_mut(raw[1])?,
            ParameterMode::Immediate => return Err(Error::ImmediateWrite),
        };
        Ok((v0, v1))
    }
}

impl<'a> Parameters<'a> for (Word, Word, &'a mut Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<3>()?;
        let v0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        let v1 = match modes[1] {
            ParameterMode::Position => computer.memory.ix(raw[1])?,
            ParameterMode::Immediate => raw[1],
        };
        let v2 = match modes[2] {
            ParameterMode::Position => computer.memory.ix_mut(raw[2])?,
            ParameterMode::Immediate => return Err(Error::ImmediateWrite),
        };
        Ok((v0, v1, v2))
    }
}

impl<'a> Parameters<'a> for (Word, Word, Word, &'a mut Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<4>()?;
        let v0 = match modes[0] {
            ParameterMode::Position => computer.memory.ix(raw[0])?,
            ParameterMode::Immediate => raw[0],
        };
        let v1 = match modes[1] {
            ParameterMode::Position => computer.memory.ix(raw[1])?,
            ParameterMode::Immediate => raw[1],
        };
        let v2 = match modes[2] {
            ParameterMode::Position => computer.memory.ix(raw[2])?,
            ParameterMode::Immediate => raw[2],
        };
        let v3 = match modes[3] {
            ParameterMode::Position => computer.memory.ix_mut(raw[3])?,
            ParameterMode::Immediate => return Err(Error::ImmediateWrite),
        };
        Ok((v0, v1, v2, v3))
    }
}
