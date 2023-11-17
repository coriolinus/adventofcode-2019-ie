mod computer;
mod error;
mod instruction;
mod mem_idx;
mod memory;
mod opcode;
mod parameter_mode;
mod parameters;

pub use computer::Computer;
pub use error::Error;

pub type Word = i64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_example() {
        let example = [1_i64, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut computer = Computer::new(example);
        computer.run().unwrap();
        assert_eq!(computer.into_memory()[0], 3500);
    }

    #[test]
    fn day05_example() {
        let example = [1002, 4, 3, 4, 33];
        let mut computer = Computer::new(example);
        computer.step().unwrap();
        assert_eq!(computer.into_memory()[4], 99);
    }

    #[test]
    fn day05_example_io() {
        let example = [3, 0, 4, 0, 99];
        let mut computer = Computer::new(example);

        computer.provide_input([123]);
        let out = computer.collect_outputs::<Vec<_>>().unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0], 123);
    }
}
