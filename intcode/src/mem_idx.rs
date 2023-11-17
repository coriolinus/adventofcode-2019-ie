use crate::{error::Result, Word};

pub(crate) trait MemIdx<Idx> {
    /// Index into this computer's memory.
    fn ix(&self, idx: Idx) -> Result<Word>;
    fn ix_mut(&mut self, idx: Idx) -> Result<&mut Word>;
}
