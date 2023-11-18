use std::{fmt, time::Duration};

use crossbeam_channel::{Receiver, Sender};

use crate::{
    error::Error,
    error::Result,
    instruction::{Instruction, ParameterModes},
    mem_idx::MemIdx as _,
    memory::Memory,
    opcode::Opcode,
    parameters::Parameters,
    Word,
};

pub struct Computer<const CHANNEL_BUFFER: usize = 0> {
    pub(crate) memory: Memory,
    pub(crate) instruction_pointer: usize,
    pub(crate) relative_base: Word,
    input_tx: Sender<Word>,
    input_rx: Receiver<Word>,
    output_tx: Sender<Word>,
    output_rx: Receiver<Word>,
}

impl<const CHANNEL_BUFFER: usize> fmt::Debug for Computer<CHANNEL_BUFFER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Computer")
            .field("memory", &format!("[...; {}]", self.memory.len()))
            .field("instruction_pointer", &self.instruction_pointer)
            .finish()
    }
}

impl<const CHANNEL_BUFFER: usize> Computer<CHANNEL_BUFFER> {
    const CHANNEL_TIMEOUT: Duration = Duration::from_secs(1);

    pub fn new(program: impl Into<Memory>) -> Self {
        let (input_tx, input_rx) = crossbeam_channel::bounded(CHANNEL_BUFFER);
        let (output_tx, output_rx) = crossbeam_channel::bounded(CHANNEL_BUFFER);
        Self {
            memory: program.into(),
            instruction_pointer: 0,
            relative_base: 0,
            input_tx,
            input_rx,
            output_tx,
            output_rx,
        }
    }

    /// Get `N` raw parameters for the current instruction.
    ///
    /// This means that we have not yet applied the parameter modes to the parameters.
    ///
    /// Does not advance the instruction pointer.
    pub(crate) fn raw_parameters<const N: usize>(&self) -> Result<[Word; N]> {
        let low = self.instruction_pointer + 1;
        let high = low + N;
        let slice = self.memory.get(low..high).ok_or(Error::Underflow {
            idx: high,
            len: self.memory.len(),
        })?;
        Ok(slice
            .try_into()
            .expect("`fn raw_parameters()` produces an appropriately sized slice"))
    }

    pub(crate) fn parameters<'a, P>(&'a mut self, modes: ParameterModes) -> Result<P>
    where
        P: Parameters<'a>,
    {
        P::apply(self, modes)
    }

    /// Execute the opcode at the current instruction pointer.
    ///
    /// If the instruction succeeded, increment the instruction pointer appropriately.
    /// Otherwise, leave it, for debugging purposes.
    pub(crate) fn step(&mut self) -> Result<()> {
        let instruction: Instruction = self.memory.ix(self.instruction_pointer)?.try_into()?;
        let next_ip = match instruction.opcode {
            Opcode::Add => {
                let (a, b, out): (_, _, &mut _) = self.parameters(instruction.modes)?;
                *out = a + b;
                None
            }
            Opcode::Multiply => {
                let (a, b, out): (_, _, &mut _) = self.parameters(instruction.modes)?;
                *out = a * b;
                None
            }
            Opcode::Input => {
                let value = self
                    .input_rx
                    .recv_timeout(Self::CHANNEL_TIMEOUT)
                    .map_err(|_| Error::InputTimeout)?;
                let store: &mut _ = self.parameters(instruction.modes)?;
                *store = value;
                None
            }
            Opcode::Output => {
                let value = self.parameters(instruction.modes)?;
                self.output_tx
                    .send_timeout(value, Self::CHANNEL_TIMEOUT)
                    .map_err(|_| Error::OutputTimeout)?;
                None
            }
            Opcode::Halt => return Err(Error::Halt(self.instruction_pointer)),
            Opcode::JumpIfTrue => {
                let (test, target): (_, Word) = self.parameters(instruction.modes)?;
                (test != 0).then_some(target.try_into().map_err(|_| Error::IndexFailed(target))?)
            }
            Opcode::JumpIfFalse => {
                let (test, target): (_, Word) = self.parameters(instruction.modes)?;
                (test == 0).then_some(target.try_into().map_err(|_| Error::IndexFailed(target))?)
            }
            Opcode::LessThan => {
                let (a, b, out): (_, _, &mut _) = self.parameters(instruction.modes)?;
                *out = if a < b { 1 } else { 0 };
                None
            }
            Opcode::Equals => {
                let (a, b, out): (_, _, &mut _) = self.parameters(instruction.modes)?;
                *out = if a == b { 1 } else { 0 };
                None
            }
            Opcode::RelativeBaseOffset => {
                let adjust: Word = self.parameters(instruction.modes)?;
                self.relative_base += adjust;
                None
            }
        };

        match next_ip {
            Some(explicit) => self.instruction_pointer = explicit,
            None => {
                // implicit IP changes just go to the next thing after the current params are over
                self.instruction_pointer += 1 + instruction.opcode.parameter_count();
            }
        }

        Ok(())
    }

    /// Execute the contained program until completion.
    ///
    /// This drops the output sender on completion, for synchronization.
    pub fn run(&mut self) -> Result<()> {
        fn inner<const CB: usize>(computer: &mut Computer<CB>) -> Result<()> {
            loop {
                match computer.step() {
                    Ok(()) => {}
                    Err(Error::Halt(_)) => return Ok(()),
                    Err(err) => return Err(err),
                }
            }
        }
        let output = inner(self);

        // for synchronization purposes, we have to replace the output channel now.
        let (tx, rx) = crossbeam_channel::bounded(CHANNEL_BUFFER);
        self.output_tx = tx;
        self.output_rx = rx;

        // now return the output
        output
    }

    pub fn into_memory(self) -> Vec<Word> {
        self.memory.into_inner()
    }

    /// Get a sender for the input channel.
    pub fn input(&self) -> Sender<Word> {
        self.input_tx.clone()
    }

    /// Provide a set of inputs to the input channel.
    ///
    /// This is most useful before the computer is running, given a known set of inputs.
    ///
    /// ```rust,ignore
    /// let mut computer = Computer::new(program);
    /// computer.provide_input([1]);
    /// computer.run()?;
    /// ```
    pub fn provide_input(&self, inputs: impl 'static + IntoIterator<Item = Word> + Send) {
        let sender = self.input();
        std::thread::spawn(move || {
            for word in inputs.into_iter() {
                if sender.send(word).is_err() {
                    // receiver disconnected; end thread
                    return;
                }
            }
        });
    }

    /// Get a receiver for the output channel.
    pub fn output(&self) -> Receiver<Word> {
        self.output_rx.clone()
    }

    /// Collect all outputs from the output channel.
    ///
    /// This function implicitly runs `Self` to completion.
    ///
    /// NOTE: if there are external receivers defined on the output channel,
    /// the collected outputs will become nondeterministic.
    pub fn collect_outputs<Collection>(&mut self) -> Result<Collection>
    where
        Collection: Default + Extend<Word> + Send,
    {
        std::thread::scope(|scope| {
            let mut collection = Collection::default();
            let output = self.output();

            let collector_handle = scope.spawn(move || {
                // an error indicates that the sender has disconnected
                while let Ok(word) = output.recv() {
                    collection.extend(std::iter::once(word));
                }
                collection
            });
            let runner_handle = scope.spawn(|| self.run());

            runner_handle
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic))?;

            let collection = collector_handle
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic));

            Ok(collection)
        })
    }
}
