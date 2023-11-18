use aoclib::{parse, CommaSep};
use intcode::{Computer, Word};
use std::{ops::Deref as _, path::Path};

const N_AMPS: usize = 5;

type PhaseSettings = [Word; N_AMPS];

type Amp = Computer<1>;

struct AmplificationCircuit {
    amplifiers: [Amp; N_AMPS],
}

impl AmplificationCircuit {
    fn new(phase_settings: PhaseSettings, program: &[Word]) -> Self {
        let amplifiers = [
            Amp::new(program),
            Amp::new(program),
            Amp::new(program),
            Amp::new(program),
            Amp::new(program),
        ];
        for (phase, computer) in itertools::zip_eq(phase_settings, amplifiers.iter()) {
            computer
                .input()
                .try_send(phase)
                .expect("fresh computer with a buffer can receive phase without blocking");
        }
        Self { amplifiers }
    }

    fn run(&mut self, recycle: bool) -> Result<Word, Error> {
        std::thread::scope(|scope| {
            // get the transmitter for the first amp, and the receiver for the last
            let circuit_in = self.amplifiers[0].input();
            let circuit_out = self.amplifiers[4].output();

            // hook the amplifiers into each other
            // if either end of the channel closes, that's just the signal to kill the thread
            for window in self.amplifiers.windows(2) {
                let l = &window[0];
                let r = &window[1];

                let rx = l.output();
                let tx = r.input();

                scope.spawn(move || {
                    for msg in rx.iter() {
                        if tx.send(msg).is_err() {
                            return;
                        }
                    }
                });
            }

            // now run each amplifier
            for amp in self.amplifiers.iter_mut() {
                scope.spawn(|| amp.run());
            }

            // kick everything off
            let mut msg = 0;
            let mut first = true;

            // run the loop
            while first || recycle {
                first = false;

                if circuit_in.send(msg).is_err() {
                    // can't send; amp 0 has finished its run
                    break;
                }

                let Ok(msg_r) = circuit_out.recv() else {
                    // can't recv; amp 4 has finished its run
                    break;
                };
                msg = msg_r;
            }

            Ok(msg)
        })
    }
}

fn find_max_value(
    idx: usize,
    part: u8,
    program: &[Word],
    phase_settings: &mut PhaseSettings,
    recycle: bool,
) -> Result<(), Error> {
    let heap = permutohedron::Heap::new(phase_settings);
    let Some((max_value, phase_settings)) = heap
        .map(|phase_settings| {
            let mut circuit = AmplificationCircuit::new(phase_settings, program);
            let value = circuit
                .run(recycle)
                .expect("does not fail to compute a value");
            (value, phase_settings)
        })
        .max()
    else {
        return Err(Error::NoSolution);
    };
    println!("pgm {idx} pt {part}: max value {max_value} with {phase_settings:?}");
    Ok(())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Word>>(input)?.enumerate() {
        let program = program.deref().as_ref();
        find_max_value(idx, 1, program, &mut [0, 1, 2, 3, 4], false)?;
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, program) in parse::<CommaSep<Word>>(input)?.enumerate() {
        let program = program.deref().as_ref();
        find_max_value(idx, 2, program, &mut [5, 6, 7, 8, 9], true)?;
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
