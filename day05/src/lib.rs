use aoclib::{parse, CommaSep};
use intcode::{Computer, Word};
use std::{ops::Deref, path::Path};

fn get_diagnostic_code(program: Vec<Word>, device_id: Word) -> Result<Word, Error> {
    let mut computer = Computer::new(program);

    computer.provide_input([device_id]);
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

    Ok(*diagnostic_code)
}

fn run_programs(input: &Path, part: u8, device_id: Word) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Word>>(input)?.enumerate() {
        let diagnostic_code = get_diagnostic_code(program.deref().clone(), device_id)?;
        println!("pgm {idx} pt {part}: {diagnostic_code}");
    }
    Ok(())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    run_programs(input, 1, 1)
}

pub fn part2(input: &Path) -> Result<(), Error> {
    run_programs(input, 2, 5)
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
