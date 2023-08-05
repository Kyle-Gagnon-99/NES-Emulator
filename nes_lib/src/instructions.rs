use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy, Serialize)]
pub enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressMode::Accumulator => write!(f, "Accumulator"),
            AddressMode::Absolute => write!(f, "Absolute"),
            AddressMode::AbsoluteX => write!(f, "Absolute X"),
            AddressMode::AbsoluteY => write!(f, "Absolute Y"),
            AddressMode::Immediate => write!(f, "Immediate"),
            AddressMode::Implied => write!(f, "Implied"),
            AddressMode::Indirect => write!(f, "Indirect"),
            AddressMode::IndirectX => write!(f, "Indirect X"),
            AddressMode::IndirectY => write!(f, "Indirect Y"),
            AddressMode::Relative => write!(f, "Relative"),
            AddressMode::ZeroPage => write!(f, "ZeroPage"),
            AddressMode::ZeroPageX => write!(f, "ZeroPage X"),
            AddressMode::ZeroPageY => write!(f, "ZeroPage Y"),
        }
    }
}

pub struct OpCode {
    pub opcode: u8,
    pub mnemonic: String,
    pub len: u8,
    pub cycles: u8,
    pub address_mode: AddressMode
}

impl OpCode {
    pub fn new(opcode: u8, mnemonic: String, len: u8, cycles: u8, address_mode: AddressMode) -> Self {
        Self {
            opcode,
            mnemonic,
            len,
            cycles,
            address_mode
        }
    }
}

lazy_static! {
    pub static ref CPU_OPCODES: Vec<OpCode> = vec![
        // ADC
        OpCode::new(0x69, String::from("ADC"), 2, 2, AddressMode::Immediate),
        OpCode::new(0x65, String::from("ADC"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x75, String::from("ADC"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0x6D, String::from("ADC"), 3, 4, AddressMode::Absolute),
        OpCode::new(0x7D, String::from("ADC"), 3, 4 /* + 1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0x79, String::from("ADC"), 3, 4 /* + 1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0x61, String::from("ADC"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0x71, String::from("ADC"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // AND
        OpCode::new(0x29, String::from("AND"), 2, 2, AddressMode::Immediate),
        OpCode::new(0x25, String::from("AND"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x35, String::from("AND"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0x2D, String::from("AND"), 3, 4, AddressMode::Absolute),
        OpCode::new(0x3D, String::from("AND"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0x39, String::from("AND"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0x21, String::from("AND"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0x31, String::from("AND"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // ASL
        OpCode::new(0x0A, String::from("ASL"), 1, 2, AddressMode::Accumulator),
        OpCode::new(0x06, String::from("ASL"), 2, 5, AddressMode::ZeroPage),
        OpCode::new(0x16, String::from("ASL"), 2, 6, AddressMode::ZeroPageX),
        OpCode::new(0x0E, String::from("ASL"), 3, 6, AddressMode::Absolute),
        OpCode::new(0x1E, String::from("ASL"), 3, 7, AddressMode::AbsoluteX),

        // BCC
        OpCode::new(0x90, String::from("BCC"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BCS
        OpCode::new(0xB0, String::from("BCS"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BEQ
        OpCode::new(0xF0, String::from("BEQ"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BIT
        OpCode::new(0x24, String::from("BIT"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x2C, String::from("BIT"), 3, 4, AddressMode::Absolute),

        // BMI
        OpCode::new(0x30, String::from("BMI"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BNE
        OpCode::new(0xD0, String::from("BNE"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BPL
        OpCode::new(0x10, String::from("BPL"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BRK
        OpCode::new(0x00, String::from("BRK"), 1, 7, AddressMode::Implied),

        // BVC
        OpCode::new(0x50, String::from("BVC"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // BVS
        OpCode::new(0x70, String::from("BVS"), 2, 2 /* +1 if branch succeeds +2 if to a new page */, AddressMode::Relative),

        // CLC
        OpCode::new(0x18, String::from("CLC"), 1, 2, AddressMode::Implied),

        // CLD
        OpCode::new(0xD8, String::from("CLD"), 1, 2, AddressMode::Implied),

        // CLI
        OpCode::new(0x58, String::from("CLI"), 1, 2, AddressMode::Implied),

        // CLV
        OpCode::new(0xB8, String::from("CLV"), 1, 2, AddressMode::Implied),

        // CMP
        OpCode::new(0xC9, String::from("CMP"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xC5, String::from("CMP"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xD5, String::from("CMP"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0xCD, String::from("CMP"), 3, 4, AddressMode::Absolute),
        OpCode::new(0xDD, String::from("CMP"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0xD9, String::from("CMP"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0xC1, String::from("CMP"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0xD1, String::from("CMP"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // CPX
        OpCode::new(0xE0, String::from("CPX"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xE4, String::from("CPX"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xEC, String::from("CPX"), 3, 4, AddressMode::Absolute),

        // CPY
        OpCode::new(0xC0, String::from("CPY"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xC4, String::from("CPY"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xCC, String::from("CPY"), 3, 4, AddressMode::Absolute),

        // DEC
        OpCode::new(0xC6, String::from("DEC"), 2, 5, AddressMode::ZeroPage),
        OpCode::new(0xD6, String::from("DEC"), 2, 6, AddressMode::ZeroPageX),
        OpCode::new(0xCE, String::from("DEC"), 3, 6, AddressMode::Absolute),
        OpCode::new(0xDE, String::from("DEC"), 3, 7, AddressMode::AbsoluteX),

        // DEX
        OpCode::new(0xCA, String::from("DEX"), 1, 2, AddressMode::Implied),

        // DEY
        OpCode::new(0x88, String::from("DEY"), 1, 2, AddressMode::Implied),

        // EOR
        OpCode::new(0x49, String::from("EOR"), 2, 2, AddressMode::Immediate),
        OpCode::new(0x45, String::from("EOR"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x55, String::from("EOR"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0x4D, String::from("EOR"), 3, 4, AddressMode::Absolute),
        OpCode::new(0x5D, String::from("EOR"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0x59, String::from("EOR"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0x41, String::from("EOR"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0x51, String::from("EOR"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // INC
        OpCode::new(0xE6, String::from("INC"), 2, 5, AddressMode::ZeroPage),
        OpCode::new(0xF6, String::from("INC"), 2, 6, AddressMode::ZeroPageX),
        OpCode::new(0xEE, String::from("INC"), 3, 6, AddressMode::Absolute),
        OpCode::new(0xFE, String::from("INC"), 3, 7, AddressMode::AbsoluteX),

        // INX
        OpCode::new(0xE8, String::from("INX"), 1, 2, AddressMode::Implied),

        // INY
        OpCode::new(0xC8, String::from("INY"), 1, 2, AddressMode::Implied),

        // JMP
        OpCode::new(0x4C, String::from("JMP"), 3, 3, AddressMode::Absolute),
        OpCode::new(0x6C, String::from("JMP"), 3, 5, AddressMode::Indirect),

        // JSR
        OpCode::new(0x20, String::from("JSR"), 3, 6, AddressMode::Absolute),

        // LDA
        OpCode::new(0xA9, String::from("LDA"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xA5, String::from("LDA"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xB5, String::from("LDA"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0xAD, String::from("LDA"), 3, 4, AddressMode::Absolute),
        OpCode::new(0xBD, String::from("LDA"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0xB9, String::from("LDA"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0xA1, String::from("LDA"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0xB1, String::from("LDA"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // LDX
        OpCode::new(0xA2, String::from("LDX"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xA6, String::from("LDX"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xB6, String::from("LDX"), 2, 4, AddressMode::ZeroPageY),
        OpCode::new(0xAE, String::from("LDX"), 3, 4, AddressMode::Absolute),

        // LDY
        OpCode::new(0xA0, String::from("LDY"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xA4, String::from("LDY"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xB4, String::from("LDY"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0xAC, String::from("LDY"), 3, 4, AddressMode::Absolute),
        OpCode::new(0xBC, String::from("LDY"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),

        // LSR
        OpCode::new(0x4A, String::from("LSR"), 1, 2, AddressMode::Accumulator),
        OpCode::new(0x46, String::from("LSR"), 2, 5, AddressMode::ZeroPage),
        OpCode::new(0x56, String::from("LSR"), 2, 6, AddressMode::ZeroPageX),
        OpCode::new(0x4E, String::from("LSR"), 3, 6, AddressMode::Absolute),
        OpCode::new(0x5E, String::from("LSR"), 3, 7, AddressMode::AbsoluteX),

        // NOP
        OpCode::new(0xEA, String::from("NOP"), 1, 2, AddressMode::Implied),

        // ORA
        OpCode::new(0x09, String::from("ORA"), 2, 2, AddressMode::Immediate),
        OpCode::new(0x05, String::from("ORA"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x15, String::from("ORA"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0x0D, String::from("ORA"), 3, 4, AddressMode::Absolute),
        OpCode::new(0x1D, String::from("ORA"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0x19, String::from("ORA"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0x01, String::from("ORA"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0x11, String::from("ORA"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // PHA
        OpCode::new(0x48, String::from("PHA"), 1, 3, AddressMode::Implied),

        // PHP
        OpCode::new(0x08, String::from("PHP"), 1, 3, AddressMode::Implied),

        // PLA
        OpCode::new(0x68, String::from("PLA"), 1, 4, AddressMode::Implied),

        // PLP
        OpCode::new(0x28, String::from("PLP"), 1, 4, AddressMode::Implied),

        // ROL
        OpCode::new(0x2A, String::from("ROL"), 1, 2, AddressMode::Accumulator),
        OpCode::new(0x26, String::from("ROL"), 2, 5, AddressMode::ZeroPage),
        OpCode::new(0x36, String::from("ROL"), 2, 6, AddressMode::ZeroPageX),
        OpCode::new(0x2E, String::from("ROL"), 3, 6, AddressMode::Absolute),
        OpCode::new(0x3E, String::from("ROL"), 3, 7, AddressMode::AbsoluteX),

        // ROR
        OpCode::new(0x6A, String::from("ROR"), 1, 2, AddressMode::Accumulator),
        OpCode::new(0x66, String::from("ROR"), 2, 5, AddressMode::ZeroPage),
        OpCode::new(0x76, String::from("ROR"), 2, 6, AddressMode::ZeroPageX),
        OpCode::new(0x6E, String::from("ROR"), 3, 6, AddressMode::Absolute),
        OpCode::new(0x7E, String::from("ROR"), 3, 7, AddressMode::AbsoluteX),

        // RTI
        OpCode::new(0x40, String::from("RTI"), 1, 6, AddressMode::Implied),

        // RTS
        OpCode::new(0x60, String::from("RTS"), 1, 6, AddressMode::Implied),

        // SBC
        OpCode::new(0xE9, String::from("SBC"), 2, 2, AddressMode::Immediate),
        OpCode::new(0xE5, String::from("SBC"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0xF5, String::from("SBC"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0xED, String::from("SBC"), 3, 4, AddressMode::Absolute),
        OpCode::new(0xFD, String::from("SBC"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteX),
        OpCode::new(0xF9, String::from("SBC"), 3, 4 /* +1 if page crossed */, AddressMode::AbsoluteY),
        OpCode::new(0xE1, String::from("SBC"), 2, 6, AddressMode::IndirectX),
        OpCode::new(0xF1, String::from("SBC"), 2, 5 /* +1 if page crossed */, AddressMode::IndirectY),

        // SEC
        OpCode::new(0x38, String::from("SEC"), 1, 2, AddressMode::Implied),

        // SED
        OpCode::new(0xF8, String::from("SED"), 1, 2, AddressMode::Implied),

        // SEI
        OpCode::new(0x78, String::from("SEI"), 1, 2, AddressMode::Implied),

        // STA
        OpCode::new(0x85, String::from("STA"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x95, String::from("STA"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0x8D, String::from("STA"), 3, 4, AddressMode::Absolute),
        OpCode::new(0x9D, String::from("STA"), 3, 5, AddressMode::AbsoluteX),
        OpCode::new(0x99, String::from("STA"), 3, 5, AddressMode::AbsoluteY),
        OpCode::new(0x81, String::from("STA"), 2, 6, AddressMode::IndirectX),

        // STX
        OpCode::new(0x86, String::from("STX"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x96, String::from("STX"), 2, 4, AddressMode::ZeroPageY),
        OpCode::new(0x8E, String::from("STX"), 3, 4, AddressMode::Absolute),

        // STY
        OpCode::new(0x84, String::from("STY"), 2, 3, AddressMode::ZeroPage),
        OpCode::new(0x94, String::from("STY"), 2, 4, AddressMode::ZeroPageX),
        OpCode::new(0x8C, String::from("STY"), 3, 4, AddressMode::Absolute),

        // TAX
        OpCode::new(0xAA, String::from("TAX"), 1, 2, AddressMode::Implied),

        // TAY
        OpCode::new(0xA8, String::from("TAY"), 1, 2, AddressMode::Implied),

        // TSX
        OpCode::new(0xBA, String::from("TSX"), 1, 2, AddressMode::Implied),

        // TXA
        OpCode::new(0x8A, String::from("TXA"), 1, 2, AddressMode::Implied),

        // TXS
        OpCode::new(0x9A, String::from("TXS"), 1, 2, AddressMode::Implied),

        // TYA
        OpCode::new(0x98, String::from("TYA"), 1, 2, AddressMode::Implied),

    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for opcode in CPU_OPCODES.iter() {
            map.insert(opcode.opcode, opcode.clone());
        }
        map
    };

    pub static ref OPCODE_SIZE: HashMap<(&'static str, AddressMode), u8> = {
        let mut map = HashMap::new();
        for opcode in CPU_OPCODES.iter() {
            map.insert((opcode.mnemonic.as_str(), opcode.address_mode), opcode.len);
        }
        map
    };
}