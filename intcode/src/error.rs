use crate::Word;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown opcode: `{0}`")]
    UnknownOpcode(Word),
    #[error("unknown parameter mode: `{0}`")]
    UnknownParameterMode(Word),
    #[error("attempted to access position {idx} but max capacity is {len}")]
    MemoryExhausted { idx: usize, len: usize },
    #[error("failed to convert `Int` value ({0}) to `usize` for indexing")]
    IndexFailed(Word),
    #[error("encountered Halt opcode at instruction pointer `{0}`")]
    Halt(usize),
    #[error("attempted write to parameter in immediate mode")]
    ImmediateWrite,
    #[error("timeout while waiting for input")]
    InputTimeout,
    #[error("timeout while attempting to send output")]
    OutputTimeout,
}
