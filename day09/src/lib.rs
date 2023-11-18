use aoclib::{parse, CommaSep};
use intcode::{Computer, Word};
use std::{ops::Deref as _, path::Path};

fn get_boost_keycode(idx: usize, part: u8, program: &[Word]) -> Result<(), Error> {
    let input = match part {
        1 => 1,
        2 => 2,
        _ => return Err(Error::UnknownPart(part)),
    };
    let mut computer = Computer::<0>::new(program);
    computer.provide_input([input]);
    let output = computer.collect_outputs::<Vec<_>>()?;
    let [malfunctioning_opcodes @ .., boost] = output.as_slice() else {
        return Err(Error::NoSolution);
    };
    for malf in malfunctioning_opcodes {
        println!("pgm {idx}: malfunctioning opcode: {malf}");
    }
    println!("pgm {idx} part {part}: boost {boost}");
    Ok(())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Word>>(input)?.enumerate() {
        let program = program.deref().as_ref();
        get_boost_keycode(idx, 1, program)?;
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Word>>(input)?.enumerate() {
        let program = program.deref().as_ref();
        get_boost_keycode(idx, 2, program)?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Intcode(#[from] intcode::Error),
    #[error("unknown part: {0}")]
    UnknownPart(u8),
    #[error("no solution found")]
    NoSolution,
}
