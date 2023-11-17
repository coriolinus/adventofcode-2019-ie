use crate::{opcode::Opcode, parameter_mode::ParameterMode, Error, Word};

/// How many parameters should we support per opcode?
pub(crate) const MAX_PARAMETERS: usize = 4;

// Note that while `MAX_PARAMETERS <= 8`, it is at least as efficient
// to transfer `ParameterModes` by value instead of by reference.
pub(crate) type ParameterModes = [ParameterMode; MAX_PARAMETERS];

pub(crate) struct Instruction {
    pub(crate) modes: ParameterModes,
    pub(crate) opcode: Opcode,
}

impl TryFrom<Word> for Instruction {
    type Error = Error;

    fn try_from(mut value: Word) -> Result<Self, Self::Error> {
        let opcode = (value % 100).try_into()?;
        value /= 100;

        let mut modes = ParameterModes::default();

        for mode in modes.iter_mut() {
            *mode = (value % 10).try_into()?;
            value /= 10;
        }

        debug_assert_eq!(
            value, 0,
            "we should not be leaving parameter modes on the table"
        );

        Ok(Self { modes, opcode })
    }
}
