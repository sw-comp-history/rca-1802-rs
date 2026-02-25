pub mod executor;
pub mod instruction;
pub mod state;

pub use executor::execute_instruction;
pub use instruction::{Instruction, Opcode};
pub use state::{Cpu, CpuError};
