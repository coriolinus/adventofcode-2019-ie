use std::ops::{Deref, DerefMut};

use crate::{
    error::{Error, Result},
    mem_idx::MemIdx,
    Word,
};

const MEMORY_LIMIT: usize = 256 * 1024 * 1024; // 256 Mb

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
        self.get(idx)
    }

    #[inline]
    fn ix_mut(&mut self, idx: usize) -> Result<&mut Word> {
        self.ensure_capacity(idx)?;
        Ok(&mut self.0[idx])
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
        self.ensure_capacity(idx)?;
        self.ix_mut(idx)
    }
}

impl Memory {
    pub fn into_inner(self) -> Vec<Word> {
        self.0
    }

    fn get(&self, idx: usize) -> Result<Word> {
        if idx > MEMORY_LIMIT {
            return Err(Error::MemoryExhausted {
                idx,
                len: MEMORY_LIMIT,
            });
        }
        Ok(self.0.get(idx).copied().unwrap_or_default())
    }

    fn ensure_capacity(&mut self, idx: usize) -> Result<()> {
        if idx >= self.len() {
            if idx > MEMORY_LIMIT {
                return Err(Error::MemoryExhausted {
                    idx,
                    len: MEMORY_LIMIT,
                });
            }
            // eprintln!("reallocating from {} to {}", self.0.len(), idx + 1);
            self.0.resize(idx + 1, Word::default());
        }
        Ok(())
    }
}
