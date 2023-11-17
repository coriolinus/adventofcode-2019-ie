use aoclib::{parse, CommaSep};
use intcode::{Computer, Int};
use std::{ops::Deref, path::Path};

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Int>>(input)?.enumerate() {
        let mut program = program.deref().clone();
        program[1] = 12;
        program[2] = 2;
        let mut computer = Computer::new(program);
        computer.run()?;
        let value = computer.into_memory()[0];
        println!("pgm {idx}: {value}");
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    unimplemented!("input file: {:?}", input)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Intcode(#[from] intcode::Error),
    #[error("no solution found")]
    NoSolution,
}
