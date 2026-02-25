use std::collections::HashMap;
use thiserror::Error;

/// Assembly errors
#[derive(Debug, Error, Clone)]
pub enum AssemblyError {
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),

    #[error("Invalid register: {0}")]
    InvalidRegister(String),

    #[error("Invalid operand: {0}")]
    InvalidOperand(String),

    #[error("Undefined label: {0}")]
    UndefinedLabel(String),

    #[error("Parse error on line {line}: {message}")]
    ParseError { line: usize, message: String },
}

/// Assembled output with machine code and disassembly
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssemblyOutput {
    pub machine_code: Vec<u8>,
    pub disassembly: Vec<String>,
}

/// Assemble RCA 1802 (COSMAC) assembly source code
pub fn assemble(source: &str) -> Result<AssemblyOutput, AssemblyError> {
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut current_address: u16 = 0;

    // First pass: collect labels
    for line in source.lines() {
        let line = strip_comments(line).trim();
        if line.is_empty() {
            continue;
        }

        // Check for label (ends with :)
        if let Some(label_end) = line.find(':') {
            let label = line[..label_end].trim().to_uppercase();
            labels.insert(label, current_address);

            // Check if there's an instruction on the same line
            let rest = line[label_end + 1..].trim();
            if !rest.is_empty() {
                current_address += get_instruction_length(rest)?;
            }
        } else {
            // Regular instruction
            current_address += get_instruction_length(line)?;
        }
    }

    // Second pass: assemble instructions
    let mut machine_code = Vec::new();
    let mut disassembly = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        let line = strip_comments(line).trim();
        if line.is_empty() {
            continue;
        }

        // Skip label part if present, get instruction
        let instruction_line = if let Some(label_end) = line.find(':') {
            line[label_end + 1..].trim()
        } else {
            line
        };

        if instruction_line.is_empty() {
            continue;
        }

        match assemble_instruction(instruction_line, &labels) {
            Ok(bytes) => {
                // Format: address: opcodes | assembly
                let addr = machine_code.len();
                let opcodes = bytes
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                let disasm = format!("{:04X}: {:<8} | {}", addr, opcodes, instruction_line);
                disassembly.push(disasm);
                machine_code.extend_from_slice(&bytes);
            }
            Err(e) => {
                return Err(AssemblyError::ParseError {
                    line: line_num + 1,
                    message: e.to_string(),
                });
            }
        }
    }

    Ok(AssemblyOutput {
        machine_code,
        disassembly,
    })
}

/// Strip comments from a line (everything after ; or #)
fn strip_comments(line: &str) -> &str {
    if let Some(idx) = line.find(';').or_else(|| line.find('#')) {
        &line[..idx]
    } else {
        line
    }
}

/// Get the length of an instruction without fully assembling it
fn get_instruction_length(line: &str) -> Result<u16, AssemblyError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(0);
    }

    let mnemonic = parts[0].to_uppercase();

    // Determine instruction length based on mnemonic
    match mnemonic.as_str() {
        // 1-byte instructions
        "IDL" | "IRX" | "RET" | "DIS" | "LDXA" | "STXD" | "ADC" | "SDB" | "SHRC" | "SMB"
        | "SAV" | "MARK" | "REQ" | "SEQ" | "NOP" | "LDX" | "OR" | "AND" | "XOR" | "ADD" | "SD"
        | "SHR" | "SM" | "SHL" | "LDN" | "INC" | "DEC" | "LDA" | "STR" | "GLO" | "GHI" | "PLO"
        | "PHI" | "SEP" | "SEX" | "OUT" | "INP" => Ok(1),

        // 2-byte instructions (short branches and immediates)
        "BR" | "BQ" | "BZ" | "BDF" | "B1" | "B2" | "B3" | "B4" | "SKP" | "BNQ" | "BNZ" | "BNF"
        | "BN1" | "BN2" | "BN3" | "BN4" | "LDI" | "ORI" | "ANI" | "XRI" | "ADI" | "SDI" | "SMI"
        | "ADCI" | "SDBI" | "SHLC" | "SMBI" => Ok(2),

        // 3-byte instructions (long branches)
        "LBR" | "LBQ" | "LBZ" | "LBDF" | "LSNQ" | "LSNZ" | "LSNF" | "LSKP" | "LBNQ" | "LBNZ"
        | "LBNF" | "LSIE" | "LSQ" | "LSZ" | "LSDF" => Ok(3),

        _ => Err(AssemblyError::InvalidInstruction(mnemonic)),
    }
}

/// Assemble a single instruction
fn assemble_instruction(
    line: &str,
    labels: &HashMap<String, u16>,
) -> Result<Vec<u8>, AssemblyError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(vec![]);
    }

    let mnemonic = parts[0].to_uppercase();

    match mnemonic.as_str() {
        // No-operand instructions
        "IDL" => Ok(vec![0x00]),
        "IRX" => Ok(vec![0x60]),
        "RET" => Ok(vec![0x70]),
        "DIS" => Ok(vec![0x71]),
        "LDXA" => Ok(vec![0x72]),
        "STXD" => Ok(vec![0x73]),
        "ADC" => Ok(vec![0x74]),
        "SDB" => Ok(vec![0x75]),
        "SHRC" => Ok(vec![0x76]),
        "SMB" => Ok(vec![0x77]),
        "SAV" => Ok(vec![0x78]),
        "MARK" => Ok(vec![0x79]),
        "REQ" => Ok(vec![0x7A]),
        "SEQ" => Ok(vec![0x7B]),
        "NOP" => Ok(vec![0xC4]),
        "LDX" => Ok(vec![0xF0]),
        "OR" => Ok(vec![0xF1]),
        "AND" => Ok(vec![0xF2]),
        "XOR" => Ok(vec![0xF3]),
        "ADD" => Ok(vec![0xF4]),
        "SD" => Ok(vec![0xF5]),
        "SHR" => Ok(vec![0xF6]),
        "SM" => Ok(vec![0xF7]),
        "SHL" => Ok(vec![0xFE]),

        // Register-based instructions (1 byte)
        "LDN" => assemble_register_op(0x00, &parts),
        "INC" => assemble_register_op(0x10, &parts),
        "DEC" => assemble_register_op(0x20, &parts),
        "LDA" => assemble_register_op(0x40, &parts),
        "STR" => assemble_register_op(0x50, &parts),
        "OUT" => assemble_register_op(0x60, &parts),
        "INP" => assemble_register_op(0x68, &parts),
        "GLO" => assemble_register_op(0x80, &parts),
        "GHI" => assemble_register_op(0x90, &parts),
        "PLO" => assemble_register_op(0xA0, &parts),
        "PHI" => assemble_register_op(0xB0, &parts),
        "SEP" => assemble_register_op(0xD0, &parts),
        "SEX" => assemble_register_op(0xE0, &parts),

        // Immediate instructions (2 bytes)
        "LDI" => assemble_immediate(0xF8, &parts),
        "ORI" => assemble_immediate(0xF9, &parts),
        "ANI" => assemble_immediate(0xFA, &parts),
        "XRI" => assemble_immediate(0xFB, &parts),
        "ADI" => assemble_immediate(0xFC, &parts),
        "SDI" => assemble_immediate(0xFD, &parts),
        "SMI" => assemble_immediate(0xFF, &parts),
        "ADCI" => assemble_immediate(0x7C, &parts),
        "SDBI" => assemble_immediate(0x7D, &parts),
        "SHLC" => assemble_immediate(0x7E, &parts),
        "SMBI" => assemble_immediate(0x7F, &parts),

        // Short branch instructions (2 bytes - opcode + offset)
        "BR" => assemble_short_branch(0x30, &parts, labels),
        "BQ" => assemble_short_branch(0x31, &parts, labels),
        "BZ" => assemble_short_branch(0x32, &parts, labels),
        "BDF" => assemble_short_branch(0x33, &parts, labels),
        "B1" => assemble_short_branch(0x34, &parts, labels),
        "B2" => assemble_short_branch(0x35, &parts, labels),
        "B3" => assemble_short_branch(0x36, &parts, labels),
        "B4" => assemble_short_branch(0x37, &parts, labels),
        "SKP" => assemble_short_branch(0x38, &parts, labels),
        "BNQ" => assemble_short_branch(0x39, &parts, labels),
        "BNZ" => assemble_short_branch(0x3A, &parts, labels),
        "BNF" => assemble_short_branch(0x3B, &parts, labels),
        "BN1" => assemble_short_branch(0x3C, &parts, labels),
        "BN2" => assemble_short_branch(0x3D, &parts, labels),
        "BN3" => assemble_short_branch(0x3E, &parts, labels),
        "BN4" => assemble_short_branch(0x3F, &parts, labels),

        // Long branch instructions (3 bytes - opcode + 16-bit address)
        "LBR" => assemble_long_branch(0xC0, &parts, labels),
        "LBQ" => assemble_long_branch(0xC1, &parts, labels),
        "LBZ" => assemble_long_branch(0xC2, &parts, labels),
        "LBDF" => assemble_long_branch(0xC3, &parts, labels),
        "LSNQ" => assemble_long_branch(0xC5, &parts, labels),
        "LSNZ" => assemble_long_branch(0xC6, &parts, labels),
        "LSNF" => assemble_long_branch(0xC7, &parts, labels),
        "LSKP" => assemble_long_branch(0xC8, &parts, labels),
        "LBNQ" => assemble_long_branch(0xC9, &parts, labels),
        "LBNZ" => assemble_long_branch(0xCA, &parts, labels),
        "LBNF" => assemble_long_branch(0xCB, &parts, labels),
        "LSIE" => assemble_long_branch(0xCC, &parts, labels),
        "LSQ" => assemble_long_branch(0xCD, &parts, labels),
        "LSZ" => assemble_long_branch(0xCE, &parts, labels),
        "LSDF" => assemble_long_branch(0xCF, &parts, labels),

        _ => Err(AssemblyError::InvalidInstruction(mnemonic)),
    }
}

/// Assemble a register-based instruction (opcode | register)
fn assemble_register_op(base_opcode: u8, parts: &[&str]) -> Result<Vec<u8>, AssemblyError> {
    if parts.len() < 2 {
        return Err(AssemblyError::InvalidOperand(
            "Register operand required".to_string(),
        ));
    }

    let reg = parse_register(parts[1])?;
    Ok(vec![base_opcode | reg])
}

/// Assemble an immediate instruction (opcode + immediate byte)
fn assemble_immediate(opcode: u8, parts: &[&str]) -> Result<Vec<u8>, AssemblyError> {
    if parts.len() < 2 {
        return Err(AssemblyError::InvalidOperand(
            "Immediate value required".to_string(),
        ));
    }

    let value = parse_number(parts[1])?;
    Ok(vec![opcode, value as u8])
}

/// Assemble a short branch instruction (opcode + offset)
fn assemble_short_branch(
    opcode: u8,
    parts: &[&str],
    labels: &HashMap<String, u16>,
) -> Result<Vec<u8>, AssemblyError> {
    if parts.len() < 2 {
        return Err(AssemblyError::InvalidOperand(
            "Branch target required".to_string(),
        ));
    }

    let target = parts[1].to_uppercase();

    // Check if it's a label or a direct offset
    let offset = if let Some(&addr) = labels.get(&target) {
        addr as u8 // For short branches, use low byte of address
    } else {
        parse_number(parts[1])? as u8
    };

    Ok(vec![opcode, offset])
}

/// Assemble a long branch instruction (opcode + 16-bit address)
fn assemble_long_branch(
    opcode: u8,
    parts: &[&str],
    labels: &HashMap<String, u16>,
) -> Result<Vec<u8>, AssemblyError> {
    if parts.len() < 2 {
        return Err(AssemblyError::InvalidOperand(
            "Branch target required".to_string(),
        ));
    }

    let target = parts[1].to_uppercase();

    // Check if it's a label or a direct address
    let address = if let Some(&addr) = labels.get(&target) {
        addr
    } else {
        parse_number(parts[1])?
    };

    // Encode as big-endian: high byte, then low byte
    Ok(vec![opcode, (address >> 8) as u8, address as u8])
}

/// Parse a register (R0-RF or 0-F or RA-RF)
fn parse_register(s: &str) -> Result<u8, AssemblyError> {
    let s = s.trim().to_uppercase();

    // Remove 'R' prefix if present
    let num_str = if let Some(stripped) = s.strip_prefix('R') {
        stripped
    } else {
        &s
    };

    // Parse hex digit (0-F)
    if num_str.len() == 1
        && let Some(digit) = num_str.chars().next()
        && let Some(value) = digit.to_digit(16)
        && value <= 15
    {
        return Ok(value as u8);
    }

    Err(AssemblyError::InvalidRegister(format!(
        "Register must be R0-RF or 0-F, got: {}",
        s
    )))
}

/// Parse a number (hex with 0x prefix, or decimal)
fn parse_number(s: &str) -> Result<u16, AssemblyError> {
    let s = s.trim();

    // Remove any comma separators
    let s = s.replace(',', "");

    // Hex with 0x prefix
    if s.starts_with("0x") || s.starts_with("0X") {
        u16::from_str_radix(&s[2..], 16)
            .map_err(|_| AssemblyError::InvalidOperand(format!("Invalid hex number: {}", s)))
    }
    // Hex with $ prefix (alternate notation)
    else if let Some(stripped) = s.strip_prefix('$') {
        u16::from_str_radix(stripped, 16)
            .map_err(|_| AssemblyError::InvalidOperand(format!("Invalid hex number: {}", s)))
    }
    // Pure hex (no prefix) - try hex first
    else if s.chars().all(|c| c.is_ascii_hexdigit()) {
        // Try hex first
        if let Ok(val) = u16::from_str_radix(&s, 16) {
            Ok(val)
        } else {
            // Fall back to decimal
            s.parse::<u16>()
                .map_err(|_| AssemblyError::InvalidOperand(format!("Invalid number: {}", s)))
        }
    }
    // Decimal
    else {
        s.parse::<u16>()
            .map_err(|_| AssemblyError::InvalidOperand(format!("Invalid number: {}", s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register("R0").unwrap(), 0);
        assert_eq!(parse_register("R5").unwrap(), 5);
        assert_eq!(parse_register("RA").unwrap(), 10);
        assert_eq!(parse_register("RF").unwrap(), 15);
        assert_eq!(parse_register("0").unwrap(), 0);
        assert_eq!(parse_register("F").unwrap(), 15);
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("0x42").unwrap(), 0x42);
        assert_eq!(parse_number("$FF").unwrap(), 0xFF);
        assert_eq!(parse_number("FF").unwrap(), 0xFF);
        assert_eq!(parse_number("100").unwrap(), 0x100); // Interpreted as hex
        assert_eq!(parse_number("0x1234").unwrap(), 0x1234);
    }

    #[test]
    fn test_assemble_simple() {
        let source = "LDI 0x42\nPHI R5\nIDL";
        let result = assemble(source).unwrap();

        assert_eq!(result.machine_code, vec![0xF8, 0x42, 0xB5, 0x00]);
    }

    #[test]
    fn test_assemble_with_labels() {
        let source = r#"
START: LDI 0x10
       PHI R3
       BR START
"#;
        let result = assemble(source).unwrap();

        // LDI 0x10 = F8 10
        // PHI R3 = B3
        // BR START (offset 0) = 30 00
        assert_eq!(result.machine_code, vec![0xF8, 0x10, 0xB3, 0x30, 0x00]);
    }

    #[test]
    fn test_assemble_long_branch() {
        let source = "LBR 0x1234";
        let result = assemble(source).unwrap();

        // LBR 0x1234 = C0 12 34
        assert_eq!(result.machine_code, vec![0xC0, 0x12, 0x34]);
    }

    #[test]
    fn test_assemble_register_ops() {
        let source = r#"
INC R3
DEC RA
GLO R5
GHI R7
"#;
        let result = assemble(source).unwrap();

        // INC R3 = 13
        // DEC RA = 2A
        // GLO R5 = 85
        // GHI R7 = 97
        assert_eq!(result.machine_code, vec![0x13, 0x2A, 0x85, 0x97]);
    }

    #[test]
    fn test_comments() {
        let source = r#"
; This is a comment
LDI 0x42  ; Load immediate
PHI R5    # Store to R5 high byte
"#;
        let result = assemble(source).unwrap();

        assert_eq!(result.machine_code, vec![0xF8, 0x42, 0xB5]);
    }

    #[test]
    fn test_invalid_instruction() {
        let source = "INVALID";
        let result = assemble(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_undefined_label() {
        let source = "BR UNDEFINED";
        let result = assemble(source);
        // Undefined label should cause an error when it can't be parsed as a number
        assert!(result.is_err());
    }
}
