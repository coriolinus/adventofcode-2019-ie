mod computer;
mod error;
mod mem_idx;
mod memory;
mod opcode;

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
}
