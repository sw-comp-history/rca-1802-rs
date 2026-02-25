use serde::{Deserialize, Serialize};
use thiserror::Error;

/// RCA 1802 (COSMAC) CPU errors
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CpuError {
    #[error("Invalid register: {0}")]
    InvalidRegister(u8),
    #[error("Memory address out of bounds: {0:#06x}")]
    MemoryOutOfBounds(u16),
    #[error("Invalid instruction at address {0:#06x}")]
    InvalidInstruction(u16),
    #[error("CPU is halted")]
    Halted,
}

/// RCA 1802 CPU State
///
/// The RCA 1802 (COSMAC) is an 8-bit microprocessor with a unique architecture:
/// - 16 general-purpose 16-bit registers (R0-RF)
/// - 8-bit accumulator (D)
/// - 1-bit data flag (DF) - carry/borrow/shift out
/// - 4-bit program counter selector (P) - selects which register is PC
/// - 4-bit index register selector (X) - selects which register for indexing
/// - 64KB address space (16-bit addressing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    /// 16 general-purpose 16-bit registers (R0-RF)
    /// Any register can serve as program counter or index register
    pub registers: [u16; 16],

    /// 8-bit accumulator (D register)
    pub d: u8,

    /// Data Flag (DF) - 1-bit carry/borrow/shift out flag
    pub df: bool,

    /// Program counter selector (P) - 4-bit value (0-F)
    /// Selects which register (R0-RF) serves as the program counter
    pub p: u8,

    /// Index register selector (X) - 4-bit value (0-F)
    /// Selects which register (R0-RF) serves as the index register
    pub x: u8,

    /// Interrupt Enable flag
    pub ie: bool,

    /// Q output bit (external output line)
    pub q: bool,

    /// Main memory (64KB)
    pub memory: Vec<u8>,

    /// CPU halted flag
    pub halted: bool,

    /// Cycle counter
    pub cycles: u64,

    /// Instructions executed
    pub instructions_executed: u64,
}

impl Cpu {
    /// Memory size (64KB)
    pub const MEMORY_SIZE: usize = 65536;

    /// Program start address (typically 0x0000 for RCA 1802)
    pub const PROGRAM_START_ADDRESS: u16 = 0x0000;

    /// Create a new CPU instance with power-on reset state
    ///
    /// Reset state matches real RCA 1802 hardware:
    /// - P = 0 (R0 is the program counter)
    /// - X = 0 (R0 is also the index register - UNUSUAL!)
    /// - All registers = 0
    /// - IE = 1 (interrupts enabled)
    ///
    /// Note: Having both P and X = 0 causes conflicts in practice.
    /// Programs typically start with "SEX R2" or "SEX R3" to move
    /// the index register away from the program counter.
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            d: 0,
            df: false,
            p: 0,     // R0 is program counter (power-on reset default)
            x: 0,     // R0 is ALSO index register (unusual but matches hardware)
            ie: true, // Interrupts enabled (power-on reset default)
            q: false,
            memory: vec![0; Self::MEMORY_SIZE],
            halted: false,
            cycles: 0,
            instructions_executed: 0,
        }
    }

    /// Reset CPU to initial state
    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.d = 0;
        self.df = false;
        self.p = 0;
        self.x = 0;
        self.ie = true;
        self.q = false;
        self.memory.fill(0);
        self.halted = false;
        self.cycles = 0;
        self.instructions_executed = 0;
    }

    /// Get program counter value (value of register selected by P)
    pub fn get_pc(&self) -> u16 {
        self.registers[self.p as usize]
    }

    /// Set program counter value (sets value of register selected by P)
    pub fn set_pc(&mut self, value: u16) {
        self.registers[self.p as usize] = value;
    }

    /// Increment program counter
    pub fn increment_pc(&mut self) {
        let pc = self.get_pc();
        self.set_pc(pc.wrapping_add(1));
    }

    /// Get index register value (value of register selected by X)
    pub fn get_x_register(&self) -> u16 {
        self.registers[self.x as usize]
    }

    /// Set index register value (sets value of register selected by X)
    pub fn set_x_register(&mut self, value: u16) {
        self.registers[self.x as usize] = value;
    }

    /// Get a specific register value
    pub fn get_register(&self, reg: u8) -> Result<u16, CpuError> {
        if reg > 15 {
            return Err(CpuError::InvalidRegister(reg));
        }
        Ok(self.registers[reg as usize])
    }

    /// Set a specific register value
    pub fn set_register(&mut self, reg: u8, value: u16) -> Result<(), CpuError> {
        if reg > 15 {
            return Err(CpuError::InvalidRegister(reg));
        }
        self.registers[reg as usize] = value;
        Ok(())
    }

    /// Read a byte from memory
    pub fn read_byte(&self, addr: u16) -> Result<u8, CpuError> {
        Ok(self.memory[addr as usize])
    }

    /// Write a byte to memory
    pub fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), CpuError> {
        self.memory[addr as usize] = value;
        Ok(())
    }

    /// Load program into memory starting at address
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) -> Result<(), CpuError> {
        let end_addr = start_addr as usize + program.len();
        if end_addr > Self::MEMORY_SIZE {
            return Err(CpuError::MemoryOutOfBounds(end_addr as u16));
        }

        self.memory[start_addr as usize..end_addr].copy_from_slice(program);
        Ok(())
    }

    /// Check if CPU is halted
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// Halt the CPU
    pub fn halt(&mut self) {
        self.halted = true;
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_creation() {
        let cpu = Cpu::new();
        assert_eq!(cpu.get_pc(), 0);
        assert_eq!(cpu.d, 0);
        assert!(!cpu.df);
        assert_eq!(cpu.p, 0);
        assert_eq!(cpu.x, 0);
    }

    #[test]
    fn test_pc_operations() {
        let mut cpu = Cpu::new();
        cpu.set_pc(0x100);
        assert_eq!(cpu.get_pc(), 0x100);

        cpu.increment_pc();
        assert_eq!(cpu.get_pc(), 0x101);
    }

    #[test]
    fn test_register_operations() {
        let mut cpu = Cpu::new();
        cpu.set_register(5, 0x1234).unwrap();
        assert_eq!(cpu.get_register(5).unwrap(), 0x1234);

        assert!(cpu.set_register(16, 0).is_err());
        assert!(cpu.get_register(16).is_err());
    }

    #[test]
    fn test_memory_operations() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x100, 0x42).unwrap();
        assert_eq!(cpu.read_byte(0x100).unwrap(), 0x42);
    }

    #[test]
    fn test_load_program() {
        let mut cpu = Cpu::new();
        let program = vec![0x01, 0x02, 0x03, 0x04];
        cpu.load_program(&program, 0x100).unwrap();

        assert_eq!(cpu.read_byte(0x100).unwrap(), 0x01);
        assert_eq!(cpu.read_byte(0x101).unwrap(), 0x02);
        assert_eq!(cpu.read_byte(0x102).unwrap(), 0x03);
        assert_eq!(cpu.read_byte(0x103).unwrap(), 0x04);
    }

    #[test]
    fn test_p_and_x_selectors() {
        let mut cpu = Cpu::new();

        // Set R3 to 0x200
        cpu.set_register(3, 0x200).unwrap();

        // Make R3 the program counter
        cpu.p = 3;
        assert_eq!(cpu.get_pc(), 0x200);

        // Set R5 to 0x300
        cpu.set_register(5, 0x300).unwrap();

        // Make R5 the index register
        cpu.x = 5;
        assert_eq!(cpu.get_x_register(), 0x300);
    }
}
