use crate::{Error, Word};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum::FromRepr)]
#[repr(u8)]
pub enum ParameterMode {
    #[default]
    Position = 0,
    Immediate = 1,
}

impl TryFrom<Word> for ParameterMode {
    type Error = Error;

    fn try_from(value: Word) -> Result<Self, Self::Error> {
        value
            .try_into()
            .ok()
            .and_then(Self::from_repr)
            .ok_or(Error::UnknownParameterMode(value))
    }
}
