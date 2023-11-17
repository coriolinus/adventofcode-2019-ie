use aoclib::{parse, CommaSep};
use intcode::{Computer, Word};
use std::{ops::Deref, path::Path};

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Word>>(input)?.enumerate() {
        let program = program.deref().clone();
        let mut computer = Computer::new(program);

        computer.provide_input([1]);
        let outputs = computer.collect_outputs::<Vec<_>>()?;

        let [test_codes @ .., diagnostic_code] = outputs.as_slice() else {
            return Err(Error::NoSolution);
        };

        for (idx, test_code) in test_codes.iter().enumerate() {
            if *test_code != 0 {
                return Err(Error::TestFailure {
                    idx,
                    value: *test_code,
                });
            }
        }

        println!("pgm {idx} pt 1: {diagnostic_code}");
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Intcode(#[from] intcode::Error),
    #[error("test code at idx {idx} produced a non-0 value {value}")]
    TestFailure { idx: usize, value: Word },
    #[error("no solution found")]
    NoSolution,
}
