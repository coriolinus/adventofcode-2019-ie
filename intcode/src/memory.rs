use std::ops::{Deref, DerefMut};

use crate::{
    error::{Error, Result},
    mem_idx::MemIdx,
    Word,
};

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

impl Deref for Memory {
    type Target = [Word];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl MemIdx<usize> for Memory {
    #[inline]
    fn ix(&self, idx: usize) -> Result<Word> {
        self.0.get(idx).copied().ok_or(Error::Underflow {
            idx,
            len: self.0.len(),
        })
    }

    #[inline]
    fn ix_mut(&mut self, idx: usize) -> Result<&mut Word> {
        let len = self.0.len();
        self.0.get_mut(idx).ok_or(Error::Underflow { idx, len })
    }
}

impl MemIdx<Word> for Memory {
    #[inline]
    fn ix(&self, idx: Word) -> Result<Word> {
        let idx: usize = idx.try_into().map_err(|_| Error::IndexFailed(idx))?;
        self.ix(idx)
    }

    #[inline]
    fn ix_mut(&mut self, idx: Word) -> Result<&mut Word> {
        let idx: usize = idx.try_into().map_err(|_| Error::IndexFailed(idx))?;
        self.ix_mut(idx)
    }
}

impl Memory {
    pub fn into_inner(self) -> Vec<Word> {
        self.0
    }
}
