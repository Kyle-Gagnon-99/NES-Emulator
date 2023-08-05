use core::fmt;
use std::collections::HashMap;

use lazy_static::lazy_static;
use nes_lib::instructions::AddressMode;
use serde::Serialize;

use crate::error::AssemblerError;

lazy_static! {
    static ref OPCODE_LOOKUP: HashMap<(&'static str, AddressMode), u8> = {
        let mut m = HashMap::new();

        // ADC - Add with Carry
        m.insert(("ADC", AddressMode::Immediate), 0x69);
        m.insert(("ADC", AddressMode::ZeroPage), 0x65);
        m.insert(("ADC", AddressMode::ZeroPageX), 0x75);
        m.insert(("ADC", AddressMode::Absolute), 0x6D);
        m.insert(("ADC", AddressMode::AbsoluteX), 0x7D);
        m.insert(("ADC", AddressMode::AbsoluteY), 0x79);
        m.insert(("ADC", AddressMode::IndirectX), 0x61);
        m.insert(("ADC", AddressMode::IndirectY), 0x71);

        // AND - Logical AND (with accumulator)
        m.insert(("AND", AddressMode::Immediate), 0x29);
        m.insert(("AND", AddressMode::ZeroPage), 0x25);
        m.insert(("AND", AddressMode::ZeroPageX), 0x35);
        m.insert(("AND", AddressMode::Absolute), 0x2D);
        m.insert(("AND", AddressMode::AbsoluteX), 0x3D);
        m.insert(("AND", AddressMode::AbsoluteY), 0x39);
        m.insert(("AND", AddressMode::IndirectX), 0x21);
        m.insert(("AND", AddressMode::IndirectY), 0x31);

        // ASL - Arithmetic Shift Left
        m.insert(("ASL", AddressMode::Accumulator), 0x0A);
        m.insert(("ASL", AddressMode::ZeroPage), 0x06);
        m.insert(("ASL", AddressMode::ZeroPageX), 0x16);
        m.insert(("ASL", AddressMode::Absolute), 0x0E);
        m.insert(("ASL", AddressMode::AbsoluteX), 0x1E);

        // BIT - test BITs
        m.insert(("BIT", AddressMode::ZeroPage), 0x24);
        m.insert(("BIT", AddressMode::Absolute), 0x2C);

        // BRK - BReaK
        m.insert(("BRK", AddressMode::Implied), 0x00);

        // CMP - Compare (with accumulator)
        m.insert(("CMP", AddressMode::Immediate), 0xC9);
        m.insert(("CMP", AddressMode::ZeroPage), 0xC5);
        m.insert(("CMP", AddressMode::ZeroPageX), 0xD5);
        m.insert(("CMP", AddressMode::Absolute), 0xCD);
        m.insert(("CMP", AddressMode::AbsoluteX), 0xDD);
        m.insert(("CMP", AddressMode::AbsoluteY), 0xD9);
        m.insert(("CMP", AddressMode::IndirectX), 0xC1);
        m.insert(("CMP", AddressMode::IndirectY), 0xD1);

        // CPX - Compare with X
        m.insert(("CPX", AddressMode::Immediate), 0xE0);
        m.insert(("CPX", AddressMode::ZeroPage), 0xE4);
        m.insert(("CPX", AddressMode::Absolute), 0xEC);

        // CPY - Compare with Y
        m.insert(("CPY", AddressMode::Immediate), 0xC0);
        m.insert(("CPY", AddressMode::ZeroPage), 0xC4);
        m.insert(("CPY", AddressMode::Absolute), 0xCC);

        // DEC - DECrement memory
        m.insert(("DEC", AddressMode::ZeroPage), 0xC6);
        m.insert(("DEC", AddressMode::ZeroPageX), 0xD6);
        m.insert(("DEC", AddressMode::Absolute), 0xCE);
        m.insert(("DEC", AddressMode::AbsoluteX), 0xDE);

        // EOR - Exclusive OR (with accumulator)
        m.insert(("EOR", AddressMode::Immediate), 0x49);
        m.insert(("EOR", AddressMode::ZeroPage), 0x45);
        m.insert(("EOR", AddressMode::ZeroPageX), 0x55);
        m.insert(("EOR", AddressMode::Absolute), 0x4D);
        m.insert(("EOR", AddressMode::AbsoluteX), 0x5D);
        m.insert(("EOR", AddressMode::AbsoluteY), 0x59);
        m.insert(("EOR", AddressMode::IndirectX), 0x41);
        m.insert(("EOR", AddressMode::IndirectY), 0x51);

        // INC - INCrement memory
        m.insert(("INC", AddressMode::ZeroPage), 0xE6);
        m.insert(("INC", AddressMode::ZeroPageX), 0xF6);
        m.insert(("INC", AddressMode::Absolute), 0xEE);
        m.insert(("INC", AddressMode::AbsoluteX), 0xFE);

        // JMP - JuMP
        m.insert(("JMP", AddressMode::Absolute), 0x4C);
        m.insert(("JMP", AddressMode::Indirect), 0x6C);

        // JSR - Jump to SubRoutine
        m.insert(("JSR", AddressMode::Absolute), 0x20);

        // LDA - Load Accumulator
        m.insert(("LDA", AddressMode::Immediate), 0xA9);
        m.insert(("LDA", AddressMode::ZeroPage), 0xA5);
        m.insert(("LDA", AddressMode::ZeroPageX), 0xB5);
        m.insert(("LDA", AddressMode::Absolute), 0xAD);
        m.insert(("LDA", AddressMode::AbsoluteX), 0xBD);
        m.insert(("LDA", AddressMode::AbsoluteY), 0xB9);
        m.insert(("LDA", AddressMode::IndirectX), 0xA1);
        m.insert(("LDA", AddressMode::IndirectY), 0xB1);

        // LDX - Load X
        m.insert(("LDX", AddressMode::Immediate), 0xA2);
        m.insert(("LDX", AddressMode::ZeroPage), 0xA6);
        m.insert(("LDX", AddressMode::ZeroPageY), 0xB6);
        m.insert(("LDX", AddressMode::Absolute), 0xAE);
        m.insert(("LDX", AddressMode::AbsoluteY), 0xBE);

        // LDY - Load Y
        m.insert(("LDY", AddressMode::Immediate), 0xA0);
        m.insert(("LDY", AddressMode::ZeroPage), 0xA4);
        m.insert(("LDY", AddressMode::ZeroPageX), 0xB4);
        m.insert(("LDY", AddressMode::Absolute), 0xAC);
        m.insert(("LDY", AddressMode::AbsoluteX), 0xBC);

        // LSR - Logical Shift Right
        m.insert(("LSR", AddressMode::Accumulator), 0x4A);
        m.insert(("LSR", AddressMode::ZeroPage), 0x46);
        m.insert(("LSR", AddressMode::ZeroPageX), 0x56);
        m.insert(("LSR", AddressMode::Absolute), 0x4E);
        m.insert(("LSR", AddressMode::AbsoluteX), 0x5E);

        // NOP - No OPeration
        m.insert(("NOP", AddressMode::Implied), 0xEA);

        // ORA - OR with Accumulator
        m.insert(("ORA", AddressMode::Immediate), 0x09);
        m.insert(("ORA", AddressMode::ZeroPage), 0x05);
        m.insert(("ORA", AddressMode::ZeroPageX), 0x15);
        m.insert(("ORA", AddressMode::Absolute), 0x0D);
        m.insert(("ORA", AddressMode::AbsoluteX), 0x1D);
        m.insert(("ORA", AddressMode::AbsoluteY), 0x19);
        m.insert(("ORA", AddressMode::IndirectX), 0x01);
        m.insert(("ORA", AddressMode::IndirectY), 0x11);

        // ROL - ROtate Left
        m.insert(("ROL", AddressMode::Accumulator), 0x2A);
        m.insert(("ROL", AddressMode::ZeroPage), 0x26);
        m.insert(("ROL", AddressMode::ZeroPageX), 0x36);
        m.insert(("ROL", AddressMode::Absolute), 0x2E);
        m.insert(("ROL", AddressMode::AbsoluteX), 0x3E);

        // ROR - ROtate Right
        m.insert(("ROR", AddressMode::Accumulator), 0x6A);
        m.insert(("ROR", AddressMode::ZeroPage), 0x66);
        m.insert(("ROR", AddressMode::ZeroPageX), 0x76);
        m.insert(("ROR", AddressMode::Absolute), 0x6E);
        m.insert(("ROR", AddressMode::AbsoluteX), 0x7E);

        // RTI - ReTurn from Interrupt
        m.insert(("RTI", AddressMode::Implied), 0x40);

        // RTS - ReTurn from Subroutine
        m.insert(("RTS", AddressMode::Implied), 0x60);

        // SBC - SuBtract with Carry
        m.insert(("SBC", AddressMode::Immediate), 0xE9);
        m.insert(("SBC", AddressMode::ZeroPage), 0xE5);
        m.insert(("SBC", AddressMode::ZeroPageX), 0xF5);
        m.insert(("SBC", AddressMode::Absolute), 0xED);
        m.insert(("SBC", AddressMode::AbsoluteX), 0xFD);
        m.insert(("SBC", AddressMode::AbsoluteY), 0xF9);
        m.insert(("SBC", AddressMode::IndirectX), 0xE1);
        m.insert(("SBC", AddressMode::IndirectY), 0xF1);

        // STA - STore Accumulator
        m.insert(("STA", AddressMode::ZeroPage), 0x85);
        m.insert(("STA", AddressMode::ZeroPageX), 0x95);
        m.insert(("STA", AddressMode::Absolute), 0x8D);
        m.insert(("STA", AddressMode::AbsoluteX), 0x9D);
        m.insert(("STA", AddressMode::AbsoluteY), 0x99);
        m.insert(("STA", AddressMode::IndirectX), 0x81);
        m.insert(("STA", AddressMode::IndirectY), 0x91);

        // STX - STore X
        m.insert(("STX", AddressMode::ZeroPage), 0x86);
        m.insert(("STX", AddressMode::ZeroPageY), 0x96);
        m.insert(("STX", AddressMode::Absolute), 0x8E);

        // STY - STore Y
        m.insert(("STY", AddressMode::ZeroPage), 0x84);
        m.insert(("STY", AddressMode::ZeroPageX), 0x94);
        m.insert(("STY", AddressMode::Absolute), 0x8C);

        // Branching Instructions
        // BPL - Branch on PLus
        m.insert(("BPL", AddressMode::Relative), 0x10);

        // BMI - Branch on MInus
        m.insert(("BMI", AddressMode::Relative), 0x30);

        // BVC - Branch on oVerflow Clear
        m.insert(("BVC", AddressMode::Relative), 0x50);

        // BVS - Branch on oVerflow Set
        m.insert(("BVS", AddressMode::Relative), 0x70);

        // BCC - Branch on Carry Clear
        m.insert(("BCC", AddressMode::Relative), 0x90);

        // BCS - Branch on Carry Set
        m.insert(("BCS", AddressMode::Relative), 0xB0);

        // BNE - Branch on Not Equal
        m.insert(("BNE", AddressMode::Relative), 0xD0);

        // BEQ - Branch on EQual
        m.insert(("BEQ", AddressMode::Relative), 0xF0);

        // Flag Instructions
        // CLC - CLear Carry
        m.insert(("CLC", AddressMode::Implied), 0x18);

        // SEC - SEt Carry
        m.insert(("SEC", AddressMode::Implied), 0x38);

        // CLI - CLear Interrupt
        m.insert(("CLI", AddressMode::Implied), 0x58);

        // SEI - SEt Interrupt
        m.insert(("SEI", AddressMode::Implied), 0x78);

        // CLV - CLear oVerflow
        m.insert(("CLV", AddressMode::Implied), 0xB8);

        // CLD - CLear Decimal
        m.insert(("CLD", AddressMode::Implied), 0xD8);

        // SED - SEt Decimal
        m.insert(("SED", AddressMode::Implied), 0xF8);

        // Stack Instructions
        // TXS - Transfer X to Stack pointer
        m.insert(("TXS", AddressMode::Implied), 0x9A);

        // TSX - Transfer Stack pointer to X
        m.insert(("TSX", AddressMode::Implied), 0xBA);

        // PHA - PusH Accumulator
        m.insert(("PHA", AddressMode::Implied), 0x48);

        // PLA - PulL Accumulator
        m.insert(("PLA", AddressMode::Implied), 0x68);

        // PHP - PusH Processor status
        m.insert(("PHP", AddressMode::Implied), 0x08);

        // PLP - PulL Processor status
        m.insert(("PLP", AddressMode::Implied), 0x28);

        // Register Instructions
        // TAX - Transfer Accumulator to X
        m.insert(("TAX", AddressMode::Implied), 0xAA);

        // TXA - Transfer X to Accumulator
        m.insert(("TXA", AddressMode::Implied), 0x8A);

        // DEX - DEcrement X
        m.insert(("DEX", AddressMode::Implied), 0xCA);

        // INX - INcrement X
        m.insert(("INX", AddressMode::Implied), 0xE8);

        // TAY - Transfer Accumulator to Y
        m.insert(("TAY", AddressMode::Implied), 0xA8);

        // TYA - Transfer Y to Accumulator
        m.insert(("TYA", AddressMode::Implied), 0x98);

        // DEY - DEcrement Y
        m.insert(("DEY", AddressMode::Implied), 0x88);

        // INY - INcrement Y
        m.insert(("INY", AddressMode::Implied), 0xC8);

        m
    };
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Instruction {
    pub opcode: String,
    pub operand: Option<Operand>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Operand {
    Address(Address),
    Label(String),
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Address {
    pub address: u32,
    pub address_mode: AddressMode,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address Mode - {}, Address: {}",
            self.address_mode, self.address
        )
    }
}

impl Instruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>, AssemblerError> {
        let mut bytes = Vec::new();

        let mut address_mode = match &self.operand {
            Some(operand) => match operand {
                Operand::Address(address) => address.address_mode,
                _ => AddressMode::Implied,
            },
            None => AddressMode::Implied,
        };

        if self.opcode.as_str() == "JSR" {
            address_mode = AddressMode::Absolute;
        }

        let opcode_byte = match OPCODE_LOOKUP.get(&(self.opcode.as_str(), address_mode)) {
            Some(byte) => *byte,
            None => return Err(AssemblerError::OpCodeConversionError(self.opcode.clone())),
        };

        bytes.push(opcode_byte);

        if let Some(operand) = &self.operand {
            match operand {
                Operand::Address(address) => {
                    let operand_bytes = address.address.to_le_bytes();
                    match address_mode {
                        AddressMode::Absolute | AddressMode::AbsoluteX | AddressMode::AbsoluteY => {
                            if address.address > 0xFFFF {
                                return Err(AssemblerError::OperandOutOfRange(format!(
                                    "{}",
                                    address
                                )));
                            }
                            bytes.push(operand_bytes[0]);
                            bytes.push(operand_bytes[1]);
                        }
                        AddressMode::ZeroPage
                        | AddressMode::ZeroPageX
                        | AddressMode::ZeroPageY
                        | AddressMode::Indirect
                        | AddressMode::IndirectY
                        | AddressMode::Immediate => {
                            if address.address > 0xFF {
                                return Err(AssemblerError::OperandOutOfRange(format!(
                                    "{}",
                                    address
                                )));
                            }
                            bytes.push(operand_bytes[0]);
                        }
                        _ => {}
                    }
                },
                Operand::Label(_) => {}
            }
        };

        Ok(bytes)
    }

    pub fn size(&self) -> u8 {
        let address_mode = match &self.operand {
            Some(operand) => match operand {
                Operand::Address(address) => address.address_mode,
                Operand::Label(_) => AddressMode::Absolute,
            },
            None => AddressMode::Implied,
        };

        match address_mode {
            AddressMode::Absolute | AddressMode::AbsoluteX | AddressMode::AbsoluteY => 3,
            AddressMode::ZeroPage
            | AddressMode::ZeroPageX
            | AddressMode::ZeroPageY
            | AddressMode::Indirect
            | AddressMode::IndirectY
            | AddressMode::Immediate => 2,
            _ => 1,
        }
    }
}
