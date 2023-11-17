use aoclib::{parse, CommaSep};
use intcode::{Computer, Int};
use std::{ops::Deref, path::Path};

fn execute(program: &[Int], noun: Int, verb: Int) -> Result<Int, Error> {
    let mut program = program.to_owned();
    program[1] = noun;
    program[2] = verb;
    let mut computer = Computer::new(program);
    computer.run()?;
    Ok(computer.into_memory()[0])
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Int>>(input)?.enumerate() {
        let program = program.deref();
        let value = execute(program, 12, 2)?;
        println!("pgm {idx} pt 1: {value}");
    }
    Ok(())
}

fn find_noun_verb(program: &[Int]) -> Result<Int, Error> {
    const WANT_VALUE: Int = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            if execute(program, noun, verb)? == WANT_VALUE {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(Error::NoSolution)
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Int>>(input)?.enumerate() {
        let program = program.deref();
        let noun_verb = find_noun_verb(program)?;
        println!("pgm {idx} pt 2: {noun_verb}");
    }
    Ok(())
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
