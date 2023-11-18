use crate::{
    error::Result, instruction::ParameterModes, mem_idx::MemIdx as _,
    parameter_mode::ParameterMode, Computer, Error, Word,
};

fn pick_param<const CB: usize, const PC: usize>(
    computer: &Computer<CB>,
    modes: ParameterModes,
    raw: [Word; PC],
    idx: usize,
) -> Result<Word> {
    let param = match modes[idx] {
        ParameterMode::Position => computer.memory.ix(raw[idx])?,
        ParameterMode::Immediate => raw[idx],
        ParameterMode::Relative => {
            let addr = raw[idx] + computer.relative_base;
            computer.memory.ix(addr)?
        }
    };
    Ok(param)
}

fn pick_param_mut<const CB: usize, const PC: usize>(
    computer: &mut Computer<CB>,
    modes: ParameterModes,
    raw: [Word; PC],
    idx: usize,
) -> Result<&mut Word> {
    let param = match modes[idx] {
        ParameterMode::Position => computer.memory.ix_mut(raw[idx])?,
        ParameterMode::Relative => {
            let addr = raw[idx] + computer.relative_base;
            computer.memory.ix_mut(addr)?
        }
        ParameterMode::Immediate => return Err(Error::ImmediateWrite),
    };
    Ok(param)
}

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

        let val0 = pick_param::<CB, 1>(computer, modes, raw, 0)?;

        Ok((val0,))
    }
}

impl<'a> Parameters<'a> for (Word, Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<2>()?;

        let val0 = pick_param::<CB, 2>(computer, modes, raw, 0)?;
        let val1 = pick_param::<CB, 2>(computer, modes, raw, 1)?;

        Ok((val0, val1))
    }
}

impl<'a> Parameters<'a> for (Word, Word, Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<3>()?;

        let val0 = pick_param::<CB, 3>(computer, modes, raw, 0)?;
        let val1 = pick_param::<CB, 3>(computer, modes, raw, 1)?;
        let val2 = pick_param::<CB, 3>(computer, modes, raw, 2)?;

        Ok((val0, val1, val2))
    }
}

impl<'a> Parameters<'a> for (Word, Word, Word, Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<4>()?;

        let val0 = pick_param::<CB, 4>(computer, modes, raw, 0)?;
        let val1 = pick_param::<CB, 4>(computer, modes, raw, 1)?;
        let val2 = pick_param::<CB, 4>(computer, modes, raw, 2)?;
        let val3 = pick_param::<CB, 4>(computer, modes, raw, 3)?;

        Ok((val0, val1, val2, val3))
    }
}

impl<'a> Parameters<'a> for (&'a mut Word,) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<1>()?;

        let val0 = pick_param_mut::<CB, 1>(computer, modes, raw, 0)?;

        Ok((val0,))
    }
}

impl<'a> Parameters<'a> for (Word, &'a mut Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<2>()?;

        let val0 = pick_param::<CB, 2>(computer, modes, raw, 0)?;
        let val1 = pick_param_mut::<CB, 2>(computer, modes, raw, 1)?;

        Ok((val0, val1))
    }
}

impl<'a> Parameters<'a> for (Word, Word, &'a mut Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<3>()?;

        let val0 = pick_param::<CB, 3>(computer, modes, raw, 0)?;
        let val1 = pick_param::<CB, 3>(computer, modes, raw, 1)?;
        let val2 = pick_param_mut::<CB, 3>(computer, modes, raw, 2)?;

        Ok((val0, val1, val2))
    }
}

impl<'a> Parameters<'a> for (Word, Word, Word, &'a mut Word) {
    fn apply<const CB: usize>(
        computer: &'a mut Computer<CB>,
        modes: ParameterModes,
    ) -> Result<Self> {
        let raw = computer.raw_parameters::<4>()?;

        let val0 = pick_param::<CB, 4>(computer, modes, raw, 0)?;
        let val1 = pick_param::<CB, 4>(computer, modes, raw, 1)?;
        let val2 = pick_param::<CB, 4>(computer, modes, raw, 2)?;
        let val3 = pick_param_mut::<CB, 4>(computer, modes, raw, 3)?;

        Ok((val0, val1, val2, val3))
    }
}
