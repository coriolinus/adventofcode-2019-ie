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
        let mut computer = Computer::<0>::new(example);
        computer.run().unwrap();
        assert_eq!(computer.into_memory()[0], 3500);
    }

    #[test]
    fn day05_example() {
        let example = [1002, 4, 3, 4, 33];
        let mut computer = Computer::<0>::new(example);
        computer.step().unwrap();
        assert_eq!(computer.into_memory()[4], 99);
    }

    #[test]
    fn day05_example_io() {
        let example = [3, 0, 4, 0, 99];
        let mut computer = Computer::<0>::new(example);

        computer.provide_input([123]);
        let out = computer.collect_outputs::<Vec<_>>().unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0], 123);
    }

    #[test]
    fn day05_example_part2() {
        let example = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        for input in 0..20 {
            let mut computer = Computer::<0>::new(example);
            computer.provide_input([input]);
            let out = computer.collect_outputs::<Vec<_>>().unwrap();

            assert_eq!(out.len(), 1);
            let expect = 1000 + input.cmp(&8) as Word;
            assert_eq!(out[0], expect);
        }
    }

    #[test]
    fn day09_quine_example() {
        let program = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut computer = Computer::<0>::new(program);
        let out = computer.collect_outputs::<Vec<_>>().unwrap();

        assert_eq!(program.as_slice(), &out);
    }
}
