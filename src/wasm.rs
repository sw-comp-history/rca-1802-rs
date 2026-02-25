use crate::assembler::assemble;
use crate::cpu::{Cpu, Instruction, execute_instruction};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// WASM-exposed CPU wrapper
#[wasm_bindgen]
pub struct WasmCpu {
    cpu: Cpu,
    program_size: usize,
}

/// Register state for JavaScript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterState {
    // 16 general-purpose 16-bit registers
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub r8: u16,
    pub r9: u16,
    pub ra: u16,
    pub rb: u16,
    pub rc: u16,
    pub rd: u16,
    pub re: u16,
    pub rf: u16,

    // Special registers
    pub d: u8,    // Accumulator
    pub df: bool, // Data flag
    pub p: u8,    // Program counter selector (0-F)
    pub x: u8,    // Index register selector (0-F)
    pub ie: bool, // Interrupt enable
    pub q: bool,  // Q output

    // CPU state
    pub cycles: u64,
    pub instructions: u64,
    pub halted: bool,
}

#[wasm_bindgen]
impl WasmCpu {
    /// Create a new CPU instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            program_size: 0,
        }
    }

    /// Reset CPU to initial state
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.program_size = 0;
    }

    /// Assemble source code and load into memory
    pub fn assemble(&mut self, source: &str) -> Result<JsValue, JsValue> {
        let output = assemble(source).map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.program_size = output.machine_code.len();
        self.cpu
            .load_program(&output.machine_code, 0)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // RCA 1802 typically starts execution at address 0
        // P selector points to R0 initially, which contains the program counter
        self.cpu.p = 0;
        self.cpu.registers[0] = 0; // R0 = 0 (start of program)

        // Clear halt flag
        self.cpu.halted = false;

        // Return assembly output (disassembly)
        serde_wasm_bindgen::to_value(&output).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Execute a single instruction
    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        if self.cpu.halted {
            return Err(JsValue::from_str("CPU is halted"));
        }

        let pc = self.cpu.get_pc();

        // Decode instruction at current PC
        let instr_bytes = (0..3)
            .filter_map(|i| self.cpu.read_byte(pc.wrapping_add(i)).ok())
            .collect::<Vec<_>>();

        if instr_bytes.is_empty() {
            return Err(JsValue::from_str("Failed to read instruction"));
        }

        let instruction = Instruction::decode(&instr_bytes)
            .ok_or_else(|| JsValue::from_str("Failed to decode instruction"))?;

        // Advance PC by instruction length BEFORE execution
        // (simulates fetch cycle where PC is incremented as bytes are read)
        let instruction_length = instruction.opcode.length() as u16;
        self.cpu.set_pc(pc.wrapping_add(instruction_length));

        // Execute instruction (branches may overwrite PC)
        execute_instruction(&mut self.cpu, &instruction)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Return register state
        self.get_state()
    }

    /// Run until halt or max cycles
    pub fn run(&mut self, max_cycles: u32) -> Result<JsValue, JsValue> {
        let start_cycles = self.cpu.cycles;

        while !self.cpu.halted && (self.cpu.cycles - start_cycles) < max_cycles as u64 {
            self.step()?;
        }

        self.get_state()
    }

    /// Get current register state
    pub fn get_state(&self) -> Result<JsValue, JsValue> {
        let state = RegisterState {
            r0: self.cpu.registers[0],
            r1: self.cpu.registers[1],
            r2: self.cpu.registers[2],
            r3: self.cpu.registers[3],
            r4: self.cpu.registers[4],
            r5: self.cpu.registers[5],
            r6: self.cpu.registers[6],
            r7: self.cpu.registers[7],
            r8: self.cpu.registers[8],
            r9: self.cpu.registers[9],
            ra: self.cpu.registers[10],
            rb: self.cpu.registers[11],
            rc: self.cpu.registers[12],
            rd: self.cpu.registers[13],
            re: self.cpu.registers[14],
            rf: self.cpu.registers[15],
            d: self.cpu.d,
            df: self.cpu.df,
            p: self.cpu.p,
            x: self.cpu.x,
            ie: self.cpu.ie,
            q: self.cpu.q,
            cycles: self.cpu.cycles,
            instructions: self.cpu.instructions_executed,
            halted: self.cpu.halted,
        };

        serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get memory contents
    pub fn get_memory(&self, start: u16, length: u16) -> Result<JsValue, JsValue> {
        let mut bytes = Vec::new();
        for i in 0..length {
            let addr = start.wrapping_add(i);
            bytes.push(self.cpu.read_byte(addr).unwrap_or(0));
        }

        serde_wasm_bindgen::to_value(&bytes).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get a specific register value
    pub fn get_register(&self, reg: u8) -> Result<u16, JsValue> {
        if reg > 15 {
            return Err(JsValue::from_str("Register must be 0-15"));
        }
        Ok(self.cpu.registers[reg as usize])
    }

    /// Set a specific register value
    pub fn set_register(&mut self, reg: u8, value: u16) -> Result<(), JsValue> {
        if reg > 15 {
            return Err(JsValue::from_str("Register must be 0-15"));
        }
        self.cpu.registers[reg as usize] = value;
        Ok(())
    }

    /// Get program counter (value of register pointed to by P)
    pub fn get_pc(&self) -> u16 {
        self.cpu.get_pc()
    }

    /// Set program counter (updates the register pointed to by P)
    pub fn set_pc(&mut self, addr: u16) {
        self.cpu.set_pc(addr);
    }

    /// Get D register (accumulator)
    pub fn get_d(&self) -> u8 {
        self.cpu.d
    }

    /// Set D register
    pub fn set_d(&mut self, value: u8) {
        self.cpu.d = value;
    }

    /// Get DF (data flag)
    pub fn get_df(&self) -> bool {
        self.cpu.df
    }

    /// Set DF
    pub fn set_df(&mut self, value: bool) {
        self.cpu.df = value;
    }

    /// Get P selector
    pub fn get_p(&self) -> u8 {
        self.cpu.p
    }

    /// Set P selector
    pub fn set_p(&mut self, value: u8) -> Result<(), JsValue> {
        if value > 15 {
            return Err(JsValue::from_str("P must be 0-15"));
        }
        self.cpu.p = value;
        Ok(())
    }

    /// Get X selector
    pub fn get_x(&self) -> u8 {
        self.cpu.x
    }

    /// Set X selector
    pub fn set_x(&mut self, value: u8) -> Result<(), JsValue> {
        if value > 15 {
            return Err(JsValue::from_str("X must be 0-15"));
        }
        self.cpu.x = value;
        Ok(())
    }

    /// Get Q output
    pub fn get_q(&self) -> bool {
        self.cpu.q
    }

    /// Get IE (interrupt enable)
    pub fn get_ie(&self) -> bool {
        self.cpu.ie
    }

    /// Check if halted
    pub fn is_halted(&self) -> bool {
        self.cpu.halted
    }

    /// Get program size
    pub fn get_program_size(&self) -> usize {
        self.program_size
    }

    /// Get cycles executed
    pub fn get_cycles(&self) -> u64 {
        self.cpu.cycles
    }

    /// Get instructions executed
    pub fn get_instructions(&self) -> u64 {
        self.cpu.instructions_executed
    }

    /// Write a byte to memory
    pub fn write_memory(&mut self, addr: u16, value: u8) -> Result<(), JsValue> {
        self.cpu
            .write_byte(addr, value)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a byte from memory
    pub fn read_memory(&self, addr: u16) -> Result<u8, JsValue> {
        self.cpu
            .read_byte(addr)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Default for WasmCpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_cpu_creation() {
        let cpu = WasmCpu::new();
        assert!(!cpu.is_halted());
        assert_eq!(cpu.get_pc(), 0);
    }

    #[test]
    fn test_wasm_reset() {
        let mut cpu = WasmCpu::new();
        cpu.set_register(1, 100).unwrap();
        cpu.set_d(42);

        cpu.reset();
        assert_eq!(cpu.get_register(1).unwrap(), 0);
        assert_eq!(cpu.get_d(), 0);
    }

    #[test]
    fn test_wasm_register_access() {
        let mut cpu = WasmCpu::new();

        // Test all 16 registers
        for i in 0..16 {
            cpu.set_register(i, 0x1234 + i as u16).unwrap();
            assert_eq!(cpu.get_register(i).unwrap(), 0x1234 + i as u16);
        }

        // Test invalid register
        assert!(cpu.set_register(16, 100).is_err());
        assert!(cpu.get_register(16).is_err());
    }

    #[test]
    fn test_wasm_special_registers() {
        let mut cpu = WasmCpu::new();

        cpu.set_d(0x42);
        assert_eq!(cpu.get_d(), 0x42);

        cpu.set_df(true);
        assert!(cpu.get_df());

        cpu.set_p(5).unwrap();
        assert_eq!(cpu.get_p(), 5);

        cpu.set_x(3).unwrap();
        assert_eq!(cpu.get_x(), 3);

        // Test invalid P/X values
        assert!(cpu.set_p(16).is_err());
        assert!(cpu.set_x(16).is_err());
    }

    #[test]
    fn test_wasm_memory_access() {
        let mut cpu = WasmCpu::new();

        cpu.write_memory(0x100, 0xAB).unwrap();
        assert_eq!(cpu.read_memory(0x100).unwrap(), 0xAB);
    }

    #[test]
    fn test_pc_advancement() {
        let mut cpu = WasmCpu::new();

        // Load a simple program: LDI 0x42, PHI R5
        // LDI = 0xF8 0x42 (2 bytes)
        // PHI R5 = 0xB5 (1 byte)
        cpu.write_memory(0, 0xF8).unwrap(); // LDI
        cpu.write_memory(1, 0x42).unwrap(); // immediate value
        cpu.write_memory(2, 0xB5).unwrap(); // PHI R5

        // Initial PC should be 0
        assert_eq!(cpu.get_pc(), 0);

        // Execute first instruction (LDI 0x42)
        // This is a 2-byte instruction, so PC should advance to 2
        let instr_bytes = vec![0xF8, 0x42];
        let instr = crate::cpu::Instruction::decode(&instr_bytes).unwrap();
        let pc = cpu.get_pc();
        cpu.set_pc(pc + instr.opcode.length() as u16);
        crate::cpu::execute_instruction(&mut cpu.cpu, &instr).unwrap();

        // PC should now be 2
        assert_eq!(cpu.get_pc(), 2);

        // D register should be 0x42
        assert_eq!(cpu.get_d(), 0x42);

        // Execute second instruction (PHI R5)
        // This is a 1-byte instruction, so PC should advance to 3
        let instr_bytes = vec![0xB5];
        let instr = crate::cpu::Instruction::decode(&instr_bytes).unwrap();
        let pc = cpu.get_pc();
        cpu.set_pc(pc + instr.opcode.length() as u16);
        crate::cpu::execute_instruction(&mut cpu.cpu, &instr).unwrap();

        // PC should now be 3
        assert_eq!(cpu.get_pc(), 3);

        // R5 high byte should be 0x42
        assert_eq!(cpu.get_register(5).unwrap() >> 8, 0x42);
    }

    // Note: Tests that use JsValue (assemble, step) can only run on wasm32 targets
}
