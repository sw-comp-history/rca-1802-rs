use super::instruction::{Instruction, Opcode};
use super::state::{Cpu, CpuError};

/// Execute a single instruction on the CPU
pub fn execute_instruction(cpu: &mut Cpu, instruction: &Instruction) -> Result<(), CpuError> {
    if cpu.is_halted() {
        return Err(CpuError::Halted);
    }

    match instruction.opcode {
        Opcode::IDL => execute_idl(cpu),
        Opcode::LDN => execute_ldn(cpu, instruction.register),
        Opcode::INC => execute_inc(cpu, instruction.register),
        Opcode::DEC => execute_dec(cpu, instruction.register),
        Opcode::LDA => execute_lda(cpu, instruction.register),
        Opcode::STR => execute_str(cpu, instruction.register),
        Opcode::GLO => execute_glo(cpu, instruction.register),
        Opcode::GHI => execute_ghi(cpu, instruction.register),
        Opcode::PLO => execute_plo(cpu, instruction.register),
        Opcode::PHI => execute_phi(cpu, instruction.register),
        Opcode::SEP => execute_sep(cpu, instruction.register),
        Opcode::SEX => execute_sex(cpu, instruction.register),
        Opcode::LDX => execute_ldx(cpu),
        Opcode::LDXA => execute_ldxa(cpu),
        Opcode::STXD => execute_stxd(cpu),
        Opcode::IRX => execute_irx(cpu),
        Opcode::LDI => execute_ldi(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::ADD => execute_add(cpu),
        Opcode::ADI => execute_adi(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::ADC => execute_adc(cpu),
        Opcode::ADCI => execute_adci(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::SD => execute_sd(cpu),
        Opcode::SDI => execute_sdi(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::SM => execute_sm(cpu),
        Opcode::SMI => execute_smi(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::AND => execute_and(cpu),
        Opcode::ANI => execute_ani(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::OR => execute_or(cpu),
        Opcode::ORI => execute_ori(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::XOR => execute_xor(cpu),
        Opcode::XRI => execute_xri(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::SHR => execute_shr(cpu),
        Opcode::SHL => execute_shl(cpu),
        Opcode::SHRC => execute_shrc(cpu),
        Opcode::SHLC => execute_shlc(cpu),
        Opcode::BR => execute_br(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::BZ => execute_bz(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::BNZ => execute_bnz(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::BDF => execute_bdf(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::BNF => execute_bnf(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::BQ => execute_bq(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::BNQ => execute_bnq(cpu, instruction.immediate.unwrap_or(0)),
        Opcode::SKP => execute_skp(cpu),
        Opcode::LBR => execute_lbr(cpu, instruction.address.unwrap_or(0)),
        Opcode::LBZ => execute_lbz(cpu, instruction.address.unwrap_or(0)),
        Opcode::LBNZ => execute_lbnz(cpu, instruction.address.unwrap_or(0)),
        Opcode::LBDF => execute_lbdf(cpu, instruction.address.unwrap_or(0)),
        Opcode::LBNF => execute_lbnf(cpu, instruction.address.unwrap_or(0)),
        Opcode::LBQ => execute_lbq(cpu, instruction.address.unwrap_or(0)),
        Opcode::LBNQ => execute_lbnq(cpu, instruction.address.unwrap_or(0)),
        Opcode::LSKP => execute_lskp(cpu),
        Opcode::REQ => execute_req(cpu),
        Opcode::SEQ => execute_seq(cpu),
        Opcode::NOP => Ok(()),
        _ => {
            // Unimplemented instructions - just continue for now
            Ok(())
        }
    }?;

    // Increment cycle and instruction counters
    cpu.cycles += 1;
    cpu.instructions_executed += 1;

    Ok(())
}

// Memory Access Instructions

/// IDL - Idle (halt until interrupt)
fn execute_idl(cpu: &mut Cpu) -> Result<(), CpuError> {
    cpu.halt();
    Ok(())
}

/// LDN - Load via N: D = M(RN)
fn execute_ldn(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let addr = cpu.get_register(n)?;
    cpu.d = cpu.read_byte(addr)?;
    Ok(())
}

/// LDA - Load advance: D = M(RN), RN++
fn execute_lda(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let addr = cpu.get_register(n)?;
    cpu.d = cpu.read_byte(addr)?;
    cpu.set_register(n, addr.wrapping_add(1))?;
    Ok(())
}

/// LDX - Load via X: D = M(RX)
fn execute_ldx(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    cpu.d = cpu.read_byte(addr)?;
    Ok(())
}

/// LDXA - Load via X and advance: D = M(RX), RX++
fn execute_ldxa(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    cpu.d = cpu.read_byte(addr)?;
    cpu.set_x_register(addr.wrapping_add(1));
    Ok(())
}

/// STR - Store via N: M(RN) = D
fn execute_str(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let addr = cpu.get_register(n)?;
    cpu.write_byte(addr, cpu.d)?;
    Ok(())
}

/// STXD - Store via X and decrement: M(RX) = D, RX--
fn execute_stxd(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    cpu.write_byte(addr, cpu.d)?;
    cpu.set_x_register(addr.wrapping_sub(1));
    Ok(())
}

/// LDI - Load immediate: D = immediate
fn execute_ldi(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    cpu.d = immediate;
    Ok(())
}

// Register Instructions

/// INC - Increment register: RN++
fn execute_inc(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let val = cpu.get_register(n)?;
    cpu.set_register(n, val.wrapping_add(1))?;
    Ok(())
}

/// DEC - Decrement register: RN--
fn execute_dec(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let val = cpu.get_register(n)?;
    cpu.set_register(n, val.wrapping_sub(1))?;
    Ok(())
}

/// IRX - Increment register X: RX++
fn execute_irx(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    cpu.set_x_register(addr.wrapping_add(1));
    Ok(())
}

/// GLO - Get low byte of RN: D = RN.low
fn execute_glo(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let val = cpu.get_register(n)?;
    cpu.d = (val & 0xFF) as u8;
    Ok(())
}

/// GHI - Get high byte of RN: D = RN.high
fn execute_ghi(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let val = cpu.get_register(n)?;
    cpu.d = ((val >> 8) & 0xFF) as u8;
    Ok(())
}

/// PLO - Put low byte: RN.low = D
fn execute_plo(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let val = cpu.get_register(n)?;
    let new_val = (val & 0xFF00) | (cpu.d as u16);
    cpu.set_register(n, new_val)?;
    Ok(())
}

/// PHI - Put high byte: RN.high = D
fn execute_phi(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    let val = cpu.get_register(n)?;
    let new_val = (val & 0x00FF) | ((cpu.d as u16) << 8);
    cpu.set_register(n, new_val)?;
    Ok(())
}

/// SEP - Set P: select RN as program counter
fn execute_sep(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    cpu.p = n & 0x0F;
    Ok(())
}

/// SEX - Set X: select RN as index register
fn execute_sex(cpu: &mut Cpu, n: u8) -> Result<(), CpuError> {
    cpu.x = n & 0x0F;
    Ok(())
}

// Arithmetic Instructions

/// ADD - Add: D = D + M(RX)
fn execute_add(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    let (result, carry) = cpu.d.overflowing_add(mem_val);
    cpu.d = result;
    cpu.df = carry;
    Ok(())
}

/// ADI - Add immediate: D = D + immediate
fn execute_adi(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    let (result, carry) = cpu.d.overflowing_add(immediate);
    cpu.d = result;
    cpu.df = carry;
    Ok(())
}

/// ADC - Add with carry: D = D + M(RX) + DF
fn execute_adc(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    let carry_in = if cpu.df { 1 } else { 0 };
    let (temp, carry1) = cpu.d.overflowing_add(mem_val);
    let (result, carry2) = temp.overflowing_add(carry_in);
    cpu.d = result;
    cpu.df = carry1 || carry2;
    Ok(())
}

/// ADCI - Add with carry immediate: D = D + immediate + DF
fn execute_adci(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    let carry_in = if cpu.df { 1 } else { 0 };
    let (temp, carry1) = cpu.d.overflowing_add(immediate);
    let (result, carry2) = temp.overflowing_add(carry_in);
    cpu.d = result;
    cpu.df = carry1 || carry2;
    Ok(())
}

/// SD - Subtract D: D = M(RX) - D
fn execute_sd(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    let (result, borrow) = mem_val.overflowing_sub(cpu.d);
    cpu.d = result;
    cpu.df = !borrow; // DF=1 if no borrow
    Ok(())
}

/// SDI - Subtract D immediate: D = immediate - D
fn execute_sdi(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    let (result, borrow) = immediate.overflowing_sub(cpu.d);
    cpu.d = result;
    cpu.df = !borrow;
    Ok(())
}

/// SM - Subtract memory: D = D - M(RX)
fn execute_sm(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    let (result, borrow) = cpu.d.overflowing_sub(mem_val);
    cpu.d = result;
    cpu.df = !borrow;
    Ok(())
}

/// SMI - Subtract memory immediate: D = D - immediate
fn execute_smi(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    let (result, borrow) = cpu.d.overflowing_sub(immediate);
    cpu.d = result;
    cpu.df = !borrow;
    Ok(())
}

// Logical Instructions

/// AND - Logical AND: D = D & M(RX)
fn execute_and(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    cpu.d &= mem_val;
    Ok(())
}

/// ANI - AND immediate: D = D & immediate
fn execute_ani(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    cpu.d &= immediate;
    Ok(())
}

/// OR - Logical OR: D = D | M(RX)
fn execute_or(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    cpu.d |= mem_val;
    Ok(())
}

/// ORI - OR immediate: D = D | immediate
fn execute_ori(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    cpu.d |= immediate;
    Ok(())
}

/// XOR - Logical XOR: D = D ^ M(RX)
fn execute_xor(cpu: &mut Cpu) -> Result<(), CpuError> {
    let addr = cpu.get_x_register();
    let mem_val = cpu.read_byte(addr)?;
    cpu.d ^= mem_val;
    Ok(())
}

/// XRI - XOR immediate: D = D ^ immediate
fn execute_xri(cpu: &mut Cpu, immediate: u8) -> Result<(), CpuError> {
    cpu.d ^= immediate;
    Ok(())
}

// Shift Instructions

/// SHR - Shift right: D >> 1, bit 0 -> DF
fn execute_shr(cpu: &mut Cpu) -> Result<(), CpuError> {
    cpu.df = (cpu.d & 1) != 0;
    cpu.d >>= 1;
    Ok(())
}

/// SHL - Shift left: D << 1, bit 7 -> DF
fn execute_shl(cpu: &mut Cpu) -> Result<(), CpuError> {
    cpu.df = (cpu.d & 0x80) != 0;
    cpu.d <<= 1;
    Ok(())
}

/// SHRC - Shift right with carry: DF -> bit 7, bit 0 -> DF
fn execute_shrc(cpu: &mut Cpu) -> Result<(), CpuError> {
    let old_carry = cpu.df;
    cpu.df = (cpu.d & 1) != 0;
    cpu.d >>= 1;
    if old_carry {
        cpu.d |= 0x80;
    }
    Ok(())
}

/// SHLC - Shift left with carry: DF -> bit 0, bit 7 -> DF
fn execute_shlc(cpu: &mut Cpu) -> Result<(), CpuError> {
    let old_carry = cpu.df;
    cpu.df = (cpu.d & 0x80) != 0;
    cpu.d <<= 1;
    if old_carry {
        cpu.d |= 1;
    }
    Ok(())
}

// Branch Instructions

/// BR - Short branch: PC = (PC.high, offset)
fn execute_br(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    let pc = cpu.get_pc();
    let new_pc = (pc & 0xFF00) | (offset as u16);
    cpu.set_pc(new_pc);
    Ok(())
}

/// BZ - Branch if zero: if D==0 then branch
fn execute_bz(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    if cpu.d == 0 {
        execute_br(cpu, offset)?;
    }
    Ok(())
}

/// BNZ - Branch if not zero: if D!=0 then branch
fn execute_bnz(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    if cpu.d != 0 {
        execute_br(cpu, offset)?;
    }
    Ok(())
}

/// BDF - Branch if DF=1
fn execute_bdf(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    if cpu.df {
        execute_br(cpu, offset)?;
    }
    Ok(())
}

/// BNF - Branch if DF=0
fn execute_bnf(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    if !cpu.df {
        execute_br(cpu, offset)?;
    }
    Ok(())
}

/// BQ - Branch if Q=1
fn execute_bq(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    if cpu.q {
        execute_br(cpu, offset)?;
    }
    Ok(())
}

/// BNQ - Branch if Q=0
fn execute_bnq(cpu: &mut Cpu, offset: u8) -> Result<(), CpuError> {
    if !cpu.q {
        execute_br(cpu, offset)?;
    }
    Ok(())
}

/// SKP - Skip - unconditionally skip next byte
fn execute_skp(cpu: &mut Cpu) -> Result<(), CpuError> {
    let pc = cpu.get_pc();
    cpu.set_pc(pc.wrapping_add(1));
    Ok(())
}

/// LBR - Long branch: PC = address
fn execute_lbr(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    cpu.set_pc(address);
    Ok(())
}

/// LBZ - Long branch if zero
fn execute_lbz(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    if cpu.d == 0 {
        cpu.set_pc(address);
    }
    Ok(())
}

/// LBNZ - Long branch if not zero
fn execute_lbnz(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    if cpu.d != 0 {
        cpu.set_pc(address);
    }
    Ok(())
}

/// LBDF - Long branch if DF=1
fn execute_lbdf(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    if cpu.df {
        cpu.set_pc(address);
    }
    Ok(())
}

/// LBNF - Long branch if DF=0
fn execute_lbnf(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    if !cpu.df {
        cpu.set_pc(address);
    }
    Ok(())
}

/// LBQ - Long branch if Q=1
fn execute_lbq(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    if cpu.q {
        cpu.set_pc(address);
    }
    Ok(())
}

/// LBNQ - Long branch if Q=0
fn execute_lbnq(cpu: &mut Cpu, address: u16) -> Result<(), CpuError> {
    if !cpu.q {
        cpu.set_pc(address);
    }
    Ok(())
}

/// LSKP - Long skip
fn execute_lskp(cpu: &mut Cpu) -> Result<(), CpuError> {
    let pc = cpu.get_pc();
    cpu.set_pc(pc.wrapping_add(2));
    Ok(())
}

// I/O Instructions

/// REQ - Reset Q: Q = 0
fn execute_req(cpu: &mut Cpu) -> Result<(), CpuError> {
    cpu.q = false;
    Ok(())
}

/// SEQ - Set Q: Q = 1
fn execute_seq(cpu: &mut Cpu) -> Result<(), CpuError> {
    cpu.q = true;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldi() {
        let mut cpu = Cpu::new();
        let instr = Instruction::with_immediate(Opcode::LDI, 0, 0x42);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.d, 0x42);
    }

    #[test]
    fn test_inc_dec() {
        let mut cpu = Cpu::new();
        cpu.set_register(5, 100).unwrap();

        let instr = Instruction::new(Opcode::INC, 5);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.get_register(5).unwrap(), 101);

        let instr = Instruction::new(Opcode::DEC, 5);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.get_register(5).unwrap(), 100);
    }

    #[test]
    fn test_ldn_str() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x100, 0x42).unwrap();
        cpu.set_register(3, 0x100).unwrap();

        // LDN R3 - load from memory
        let instr = Instruction::new(Opcode::LDN, 3);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.d, 0x42);

        // Change D and store back
        cpu.d = 0x99;
        let instr = Instruction::new(Opcode::STR, 3);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.read_byte(0x100).unwrap(), 0x99);
    }

    #[test]
    fn test_add() {
        let mut cpu = Cpu::new();
        cpu.d = 10;
        cpu.x = 3;
        cpu.set_register(3, 0x100).unwrap();
        cpu.write_byte(0x100, 20).unwrap();

        let instr = Instruction::new(Opcode::ADD, 0);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.d, 30);
        assert!(!cpu.df); // No carry
    }

    #[test]
    fn test_glo_ghi() {
        let mut cpu = Cpu::new();
        cpu.set_register(5, 0x1234).unwrap();

        // GLO R5
        let instr = Instruction::new(Opcode::GLO, 5);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.d, 0x34);

        // GHI R5
        let instr = Instruction::new(Opcode::GHI, 5);
        execute_instruction(&mut cpu, &instr).unwrap();
        assert_eq!(cpu.d, 0x12);
    }
}
