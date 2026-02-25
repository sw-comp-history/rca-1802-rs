use serde::{Deserialize, Serialize};
use std::fmt;

/// RCA 1802 instruction opcodes
///
/// The RCA 1802 uses 8-bit instructions where:
/// - High nibble (bits 4-7) is the opcode
/// - Low nibble (bits 0-3) is typically the register number N
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Opcode {
    /// IDL (00) - Idle - wait for interrupt or DMA
    IDL,

    /// LDN (0N) - Load via N - D = M(RN)
    LDN,

    /// INC (1N) - Increment register RN
    INC,

    /// DEC (2N) - Decrement register RN
    DEC,

    /// BR (30) - Short branch - unconditional branch
    BR,

    /// BQ (31) - Branch if Q=1
    BQ,

    /// BZ (32) - Branch if D=0
    BZ,

    /// BDF (33) - Branch if DF=1
    BDF,

    /// B1-B3 (34-36) - Branch on external flags
    B1,
    B2,
    B3,
    B4,

    /// SKP (38) - Skip - unconditional skip 2 bytes
    SKP,

    /// BNQ (39) - Branch if Q=0
    BNQ,

    /// BNZ (3A) - Branch if D!=0
    BNZ,

    /// BNF (3B) - Branch if DF=0
    BNF,

    /// BN1-BN4 (3C-3F) - Branch on NOT external flags
    BN1,
    BN2,
    BN3,
    BN4,

    /// LDA (4N) - Load advance - D = M(RN), RN++
    LDA,

    /// STR (5N) - Store via N - M(RN) = D
    STR,

    /// IRX (60) - Increment register X
    IRX,

    /// OUT (6N) - Output - output M(RX) to port N, RX++
    OUT,

    /// INP (6N) - Input - input from port N to M(RX) and D
    INP,

    /// RET (70) - Return - disable interrupts, return from interrupt
    RET,

    /// DIS (71) - Disable - return from interrupt and disable
    DIS,

    /// LDXA (72) - Load via X and advance - D = M(RX), RX++
    LDXA,

    /// STXD (73) - Store via X and decrement - M(RX) = D, RX--
    STXD,

    /// ADC (74) - Add with carry - D = D + M(RX) + DF
    ADC,

    /// SDB (75) - Subtract D with borrow - D = M(RX) - D - (NOT DF)
    SDB,

    /// SHRC (76) - Shift right with carry - shift D right, DF into bit 7
    SHRC,

    /// SMB (77) - Subtract memory with borrow - D = D - M(RX) - (NOT DF)
    SMB,

    /// SAV (78) - Save - M(RX) = T (save X,P registers)
    SAV,

    /// MARK (79) - Mark - X = P, M(R2) = T, R2--, (X,P) = M(R2)
    MARK,

    /// REQ (7A) - Reset Q - Q = 0
    REQ,

    /// SEQ (7B) - Set Q - Q = 1
    SEQ,

    /// ADCI (7C) - Add with carry immediate - D = D + M(PC) + DF, PC++
    ADCI,

    /// SDBI (7D) - Subtract D with borrow immediate
    SDBI,

    /// SHLC (7E) - Shift left with carry - shift D left, DF into bit 0
    SHLC,

    /// SMBI (7F) - Subtract memory with borrow immediate
    SMBI,

    /// GLO (8N) - Get low byte of RN - D = RN.low
    GLO,

    /// GHI (9N) - Get high byte of RN - D = RN.high
    GHI,

    /// PLO (AN) - Put low byte - RN.low = D
    PLO,

    /// PHI (BN) - Put high byte - RN.high = D
    PHI,

    /// LBR (C0) - Long branch - unconditional branch (3 bytes)
    LBR,

    /// LBQ (C1) - Long branch if Q=1
    LBQ,

    /// LBZ (C2) - Long branch if D=0
    LBZ,

    /// LBDF (C3) - Long branch if DF=1
    LBDF,

    /// NOP (C4) - No operation
    NOP,

    /// LSNQ (C5) - Long skip if Q=0
    LSNQ,

    /// LSNZ (C6) - Long skip if D!=0
    LSNZ,

    /// LSNF (C7) - Long skip if DF=0
    LSNF,

    /// LSKP (C8) - Long skip - unconditional skip 3 bytes
    LSKP,

    /// LBNQ (C9) - Long branch if Q=0
    LBNQ,

    /// LBNZ (CA) - Long branch if D!=0
    LBNZ,

    /// LBNF (CB) - Long branch if DF=0
    LBNF,

    /// LSIE (CC) - Long skip if interrupts enabled
    LSIE,

    /// LSQ (CD) - Long skip if Q=1
    LSQ,

    /// LSZ (CE) - Long skip if D=0
    LSZ,

    /// LSDF (CF) - Long skip if DF=1
    LSDF,

    /// SEP (DN) - Set P - select RN as program counter
    SEP,

    /// SEX (EN) - Set X - select RN as index register
    SEX,

    /// LDX (F0) - Load via X - D = M(RX)
    LDX,

    /// OR (F1) - Logical OR - D = D | M(RX)
    OR,

    /// AND (F2) - Logical AND - D = D & M(RX)
    AND,

    /// XOR (F3) - Logical XOR - D = D ^ M(RX)
    XOR,

    /// ADD (F4) - Add - D = D + M(RX)
    ADD,

    /// SD (F5) - Subtract D - D = M(RX) - D
    SD,

    /// SHR (F6) - Shift right - D = D >> 1, bit 0 -> DF
    SHR,

    /// SM (F7) - Subtract memory - D = D - M(RX)
    SM,

    /// LDI (F8) - Load immediate - D = M(PC), PC++
    LDI,

    /// ORI (F9) - OR immediate - D = D | M(PC), PC++
    ORI,

    /// ANI (FA) - AND immediate - D = D & M(PC), PC++
    ANI,

    /// XRI (FB) - XOR immediate - D = D ^ M(PC), PC++
    XRI,

    /// ADI (FC) - Add immediate - D = D + M(PC), PC++
    ADI,

    /// SDI (FD) - Subtract D immediate - D = M(PC) - D, PC++
    SDI,

    /// SHL (FE) - Shift left - D = D << 1, bit 7 -> DF
    SHL,

    /// SMI (FF) - Subtract memory immediate - D = D - M(PC), PC++
    SMI,
}

impl Opcode {
    /// Decode an opcode from a byte
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Opcode::IDL),
            0x01..=0x0F => Some(Opcode::LDN),
            0x10..=0x1F => Some(Opcode::INC),
            0x20..=0x2F => Some(Opcode::DEC),
            0x30 => Some(Opcode::BR),
            0x31 => Some(Opcode::BQ),
            0x32 => Some(Opcode::BZ),
            0x33 => Some(Opcode::BDF),
            0x34 => Some(Opcode::B1),
            0x35 => Some(Opcode::B2),
            0x36 => Some(Opcode::B3),
            0x37 => Some(Opcode::B4),
            0x38 => Some(Opcode::SKP),
            0x39 => Some(Opcode::BNQ),
            0x3A => Some(Opcode::BNZ),
            0x3B => Some(Opcode::BNF),
            0x3C => Some(Opcode::BN1),
            0x3D => Some(Opcode::BN2),
            0x3E => Some(Opcode::BN3),
            0x3F => Some(Opcode::BN4),
            0x40..=0x4F => Some(Opcode::LDA),
            0x50..=0x5F => Some(Opcode::STR),
            0x60 => Some(Opcode::IRX),
            0x61..=0x67 => Some(Opcode::OUT),
            0x68 => Some(Opcode::IRX), // 68 is also IRX
            0x69..=0x6F => Some(Opcode::INP),
            0x70 => Some(Opcode::RET),
            0x71 => Some(Opcode::DIS),
            0x72 => Some(Opcode::LDXA),
            0x73 => Some(Opcode::STXD),
            0x74 => Some(Opcode::ADC),
            0x75 => Some(Opcode::SDB),
            0x76 => Some(Opcode::SHRC),
            0x77 => Some(Opcode::SMB),
            0x78 => Some(Opcode::SAV),
            0x79 => Some(Opcode::MARK),
            0x7A => Some(Opcode::REQ),
            0x7B => Some(Opcode::SEQ),
            0x7C => Some(Opcode::ADCI),
            0x7D => Some(Opcode::SDBI),
            0x7E => Some(Opcode::SHLC),
            0x7F => Some(Opcode::SMBI),
            0x80..=0x8F => Some(Opcode::GLO),
            0x90..=0x9F => Some(Opcode::GHI),
            0xA0..=0xAF => Some(Opcode::PLO),
            0xB0..=0xBF => Some(Opcode::PHI),
            0xC0 => Some(Opcode::LBR),
            0xC1 => Some(Opcode::LBQ),
            0xC2 => Some(Opcode::LBZ),
            0xC3 => Some(Opcode::LBDF),
            0xC4 => Some(Opcode::NOP),
            0xC5 => Some(Opcode::LSNQ),
            0xC6 => Some(Opcode::LSNZ),
            0xC7 => Some(Opcode::LSNF),
            0xC8 => Some(Opcode::LSKP),
            0xC9 => Some(Opcode::LBNQ),
            0xCA => Some(Opcode::LBNZ),
            0xCB => Some(Opcode::LBNF),
            0xCC => Some(Opcode::LSIE),
            0xCD => Some(Opcode::LSQ),
            0xCE => Some(Opcode::LSZ),
            0xCF => Some(Opcode::LSDF),
            0xD0..=0xDF => Some(Opcode::SEP),
            0xE0..=0xEF => Some(Opcode::SEX),
            0xF0 => Some(Opcode::LDX),
            0xF1 => Some(Opcode::OR),
            0xF2 => Some(Opcode::AND),
            0xF3 => Some(Opcode::XOR),
            0xF4 => Some(Opcode::ADD),
            0xF5 => Some(Opcode::SD),
            0xF6 => Some(Opcode::SHR),
            0xF7 => Some(Opcode::SM),
            0xF8 => Some(Opcode::LDI),
            0xF9 => Some(Opcode::ORI),
            0xFA => Some(Opcode::ANI),
            0xFB => Some(Opcode::XRI),
            0xFC => Some(Opcode::ADI),
            0xFD => Some(Opcode::SDI),
            0xFE => Some(Opcode::SHL),
            0xFF => Some(Opcode::SMI),
        }
    }

    /// Get the mnemonic for this opcode
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Opcode::IDL => "IDL",
            Opcode::LDN => "LDN",
            Opcode::INC => "INC",
            Opcode::DEC => "DEC",
            Opcode::BR => "BR",
            Opcode::BQ => "BQ",
            Opcode::BZ => "BZ",
            Opcode::BDF => "BDF",
            Opcode::B1 => "B1",
            Opcode::B2 => "B2",
            Opcode::B3 => "B3",
            Opcode::B4 => "B4",
            Opcode::SKP => "SKP",
            Opcode::BNQ => "BNQ",
            Opcode::BNZ => "BNZ",
            Opcode::BNF => "BNF",
            Opcode::BN1 => "BN1",
            Opcode::BN2 => "BN2",
            Opcode::BN3 => "BN3",
            Opcode::BN4 => "BN4",
            Opcode::LDA => "LDA",
            Opcode::STR => "STR",
            Opcode::IRX => "IRX",
            Opcode::OUT => "OUT",
            Opcode::INP => "INP",
            Opcode::RET => "RET",
            Opcode::DIS => "DIS",
            Opcode::LDXA => "LDXA",
            Opcode::STXD => "STXD",
            Opcode::ADC => "ADC",
            Opcode::SDB => "SDB",
            Opcode::SHRC => "SHRC",
            Opcode::SMB => "SMB",
            Opcode::SAV => "SAV",
            Opcode::MARK => "MARK",
            Opcode::REQ => "REQ",
            Opcode::SEQ => "SEQ",
            Opcode::ADCI => "ADCI",
            Opcode::SDBI => "SDBI",
            Opcode::SHLC => "SHLC",
            Opcode::SMBI => "SMBI",
            Opcode::GLO => "GLO",
            Opcode::GHI => "GHI",
            Opcode::PLO => "PLO",
            Opcode::PHI => "PHI",
            Opcode::LBR => "LBR",
            Opcode::LBQ => "LBQ",
            Opcode::LBZ => "LBZ",
            Opcode::LBDF => "LBDF",
            Opcode::NOP => "NOP",
            Opcode::LSNQ => "LSNQ",
            Opcode::LSNZ => "LSNZ",
            Opcode::LSNF => "LSNF",
            Opcode::LSKP => "LSKP",
            Opcode::LBNQ => "LBNQ",
            Opcode::LBNZ => "LBNZ",
            Opcode::LBNF => "LBNF",
            Opcode::LSIE => "LSIE",
            Opcode::LSQ => "LSQ",
            Opcode::LSZ => "LSZ",
            Opcode::LSDF => "LSDF",
            Opcode::SEP => "SEP",
            Opcode::SEX => "SEX",
            Opcode::LDX => "LDX",
            Opcode::OR => "OR",
            Opcode::AND => "AND",
            Opcode::XOR => "XOR",
            Opcode::ADD => "ADD",
            Opcode::SD => "SD",
            Opcode::SHR => "SHR",
            Opcode::SM => "SM",
            Opcode::LDI => "LDI",
            Opcode::ORI => "ORI",
            Opcode::ANI => "ANI",
            Opcode::XRI => "XRI",
            Opcode::ADI => "ADI",
            Opcode::SDI => "SDI",
            Opcode::SHL => "SHL",
            Opcode::SMI => "SMI",
        }
    }

    /// Get the length of the instruction in bytes
    pub fn length(&self) -> u8 {
        match self {
            // Most instructions are 1 byte
            Opcode::IDL
            | Opcode::LDN
            | Opcode::INC
            | Opcode::DEC
            | Opcode::LDA
            | Opcode::STR
            | Opcode::IRX
            | Opcode::OUT
            | Opcode::INP
            | Opcode::RET
            | Opcode::DIS
            | Opcode::LDXA
            | Opcode::STXD
            | Opcode::ADC
            | Opcode::SDB
            | Opcode::SHRC
            | Opcode::SMB
            | Opcode::SAV
            | Opcode::MARK
            | Opcode::REQ
            | Opcode::SEQ
            | Opcode::GLO
            | Opcode::GHI
            | Opcode::PLO
            | Opcode::PHI
            | Opcode::NOP
            | Opcode::SEP
            | Opcode::SEX
            | Opcode::LDX
            | Opcode::OR
            | Opcode::AND
            | Opcode::XOR
            | Opcode::ADD
            | Opcode::SD
            | Opcode::SHR
            | Opcode::SM
            | Opcode::SHL => 1,

            // Short branches and immediate operations are 2 bytes
            Opcode::BR
            | Opcode::BQ
            | Opcode::BZ
            | Opcode::BDF
            | Opcode::B1
            | Opcode::B2
            | Opcode::B3
            | Opcode::B4
            | Opcode::SKP
            | Opcode::BNQ
            | Opcode::BNZ
            | Opcode::BNF
            | Opcode::BN1
            | Opcode::BN2
            | Opcode::BN3
            | Opcode::BN4
            | Opcode::ADCI
            | Opcode::SDBI
            | Opcode::SHLC
            | Opcode::SMBI
            | Opcode::LDI
            | Opcode::ORI
            | Opcode::ANI
            | Opcode::XRI
            | Opcode::ADI
            | Opcode::SDI
            | Opcode::SMI => 2,

            // Long branches and skips are 3 bytes
            Opcode::LBR
            | Opcode::LBQ
            | Opcode::LBZ
            | Opcode::LBDF
            | Opcode::LSNQ
            | Opcode::LSNZ
            | Opcode::LSNF
            | Opcode::LSKP
            | Opcode::LBNQ
            | Opcode::LBNZ
            | Opcode::LBNF
            | Opcode::LSIE
            | Opcode::LSQ
            | Opcode::LSZ
            | Opcode::LSDF => 3,
        }
    }
}

/// Decoded instruction with opcode and operand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub opcode: Opcode,
    pub register: u8,          // N field (0-15) for register-based instructions
    pub immediate: Option<u8>, // Immediate byte for 2-byte instructions
    pub address: Option<u16>,  // 16-bit address for long branches
}

impl Instruction {
    /// Create a new instruction
    pub fn new(opcode: Opcode, register: u8) -> Self {
        Self {
            opcode,
            register,
            immediate: None,
            address: None,
        }
    }

    /// Create instruction with immediate value
    pub fn with_immediate(opcode: Opcode, register: u8, immediate: u8) -> Self {
        Self {
            opcode,
            register,
            immediate: Some(immediate),
            address: None,
        }
    }

    /// Create instruction with 16-bit address
    pub fn with_address(opcode: Opcode, register: u8, address: u16) -> Self {
        Self {
            opcode,
            register,
            immediate: None,
            address: Some(address),
        }
    }

    /// Decode an instruction from bytes
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        let byte = bytes[0];
        let opcode = Opcode::from_byte(byte)?;
        let register = byte & 0x0F; // Low nibble is register number

        match opcode.length() {
            1 => Some(Instruction::new(opcode, register)),
            2 => {
                if bytes.len() < 2 {
                    return None;
                }
                Some(Instruction::with_immediate(opcode, register, bytes[1]))
            }
            3 => {
                if bytes.len() < 3 {
                    return None;
                }
                let addr = ((bytes[1] as u16) << 8) | (bytes[2] as u16);
                Some(Instruction::with_address(opcode, register, addr))
            }
            _ => None,
        }
    }

    /// Encode instruction to bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![self.get_base_opcode()];

        if let Some(imm) = self.immediate {
            bytes.push(imm);
        }

        if let Some(addr) = self.address {
            bytes.push((addr >> 8) as u8); // High byte
            bytes.push(addr as u8); // Low byte
        }

        bytes
    }

    /// Get the base opcode byte (with register field)
    fn get_base_opcode(&self) -> u8 {
        // This is a simplified version - would need full opcode table
        match self.opcode {
            Opcode::IDL => 0x00,
            Opcode::LDN => self.register & 0x0F,
            Opcode::INC => 0x10 | (self.register & 0x0F),
            Opcode::DEC => 0x20 | (self.register & 0x0F),
            Opcode::LDI => 0xF8,
            Opcode::NOP => 0xC4,
            // Add remaining opcodes as needed
            _ => 0x00, // Placeholder
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.opcode.mnemonic())?;

        // Add register for applicable instructions
        match self.opcode {
            Opcode::LDN
            | Opcode::INC
            | Opcode::DEC
            | Opcode::LDA
            | Opcode::STR
            | Opcode::GLO
            | Opcode::GHI
            | Opcode::PLO
            | Opcode::PHI
            | Opcode::SEP
            | Opcode::SEX
            | Opcode::OUT
            | Opcode::INP => {
                write!(f, " R{:X}", self.register)?;
            }
            _ => {}
        }

        // Add immediate value
        if let Some(imm) = self.immediate {
            write!(f, " {:02X}", imm)?;
        }

        // Add address
        if let Some(addr) = self.address {
            write!(f, " {:04X}", addr)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_decode() {
        assert_eq!(Opcode::from_byte(0x00), Some(Opcode::IDL));
        assert_eq!(Opcode::from_byte(0x15), Some(Opcode::INC));
        assert_eq!(Opcode::from_byte(0xF8), Some(Opcode::LDI));
        assert_eq!(Opcode::from_byte(0xC4), Some(Opcode::NOP));
    }

    #[test]
    fn test_instruction_decode() {
        // LDN R5 (0x05)
        let bytes = vec![0x05];
        let instr = Instruction::decode(&bytes).unwrap();
        assert_eq!(instr.opcode, Opcode::LDN);
        assert_eq!(instr.register, 5);
        assert_eq!(instr.immediate, None);

        // LDI #42 (0xF8 0x2A)
        let bytes = vec![0xF8, 0x2A];
        let instr = Instruction::decode(&bytes).unwrap();
        assert_eq!(instr.opcode, Opcode::LDI);
        assert_eq!(instr.immediate, Some(0x2A));
    }

    #[test]
    fn test_instruction_length() {
        assert_eq!(Opcode::LDN.length(), 1);
        assert_eq!(Opcode::LDI.length(), 2);
        assert_eq!(Opcode::LBR.length(), 3);
    }

    #[test]
    fn test_mnemonic() {
        assert_eq!(Opcode::LDN.mnemonic(), "LDN");
        assert_eq!(Opcode::INC.mnemonic(), "INC");
        assert_eq!(Opcode::LDI.mnemonic(), "LDI");
    }
}
