use nes_lib::instructions::AddressMode;

use crate::{
    error::AssemblerError,
    instruction::{Instruction, Operand},
};

pub fn validate_instruction(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    let instr_name = &instr.opcode;
    let instr_name = instr_name.to_ascii_uppercase();

    match instr_name.as_str() {
        "ADC" => validate_adc(instr, line_num),
        "AND" => validate_and(instr, line_num),
        "ASL" => validate_asl(instr, line_num),
        "BIT" => validate_bit(instr, line_num),
        "BPL" => validate_relative_operand(instr, line_num),
        "BMI" => validate_relative_operand(instr, line_num),
        "BVC" => validate_relative_operand(instr, line_num),
        "BVS" => validate_relative_operand(instr, line_num),
        "BCC" => validate_relative_operand(instr, line_num),
        "BCS" => validate_relative_operand(instr, line_num),
        "BNE" => validate_relative_operand(instr, line_num),
        "BEQ" => validate_relative_operand(instr, line_num),
        "BRK" => validate_brk(instr, line_num),
        "CMP" => validate_cmp(instr, line_num),
        "CPX" => validate_cpx(instr, line_num),
        "CPY" => validate_cpy(instr, line_num),
        "DEC" => validate_dec(instr, line_num),
        "EOR" => validate_eor(instr, line_num),
        "CLC" => validate_no_operand(instr, line_num),
        "SEC" => validate_no_operand(instr, line_num),
        "CLI" => validate_no_operand(instr, line_num),
        "SEI" => validate_no_operand(instr, line_num),
        "CLV" => validate_no_operand(instr, line_num),
        "CLD" => validate_no_operand(instr, line_num),
        "SED" => validate_no_operand(instr, line_num),
        "INC" => validate_inc(instr, line_num),
        "JMP" => validate_jmp(instr, line_num),
        "JSR" => validate_jsr(instr, line_num),
        "LDA" => validate_lda(instr, line_num),
        "LDX" => validate_ldx(instr, line_num),
        "LDY" => validate_ldy(instr, line_num),
        "LSR" => validate_lsr(instr, line_num),
        "NOP" => validate_no_operand(instr, line_num),
        "ORA" => validate_ora(instr, line_num),
        "TAX" => validate_no_operand(instr, line_num),
        "TXA" => validate_no_operand(instr, line_num),
        "DEX" => validate_no_operand(instr, line_num),
        "INX" => validate_no_operand(instr, line_num),
        "TAY" => validate_no_operand(instr, line_num),
        "TYA" => validate_no_operand(instr, line_num),
        "DEY" => validate_no_operand(instr, line_num),
        "INY" => validate_no_operand(instr, line_num),
        "ROL" => validate_rol(instr, line_num),
        "ROR" => validate_ror(instr, line_num),
        "RTI" => validate_no_operand(instr, line_num),
        "RTS" => validate_no_operand(instr, line_num),
        "SBC" => validate_sbc(instr, line_num),
        "STA" => validate_sta(instr, line_num),
        "TXS" => validate_no_operand(instr, line_num),
        "TSX" => validate_no_operand(instr, line_num),
        "PHA" => validate_no_operand(instr, line_num),
        "PLA" => validate_no_operand(instr, line_num),
        "PHP" => validate_no_operand(instr, line_num),
        "PLP" => validate_no_operand(instr, line_num),
        "STX" => validate_stx(instr, line_num),
        "STY" => validate_sty(instr, line_num),
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} is invalid.", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_relative_operand(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate instruction has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for instruction
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Relative => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => Ok(()),
    }
}

fn validate_no_operand(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate instruction has no operand
    if instr.operand.is_some() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} does not take an address.", instr.opcode),
            line: line_num,
        });
    }

    // Nothing else to check
    Ok(())
}

fn validate_adc(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate ADC has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for ADC
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_and(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate AND has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for AND
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_asl(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate ASL has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for ASL
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Accumulator => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_bit(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate BIT has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for BIT
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::ZeroPage => Ok(()),
            AddressMode::Absolute => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_brk(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate BRK has no operand
    if instr.operand.is_some() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} does not take an address.", instr.opcode),
            line: line_num,
        });
    }

    // Nothing else to check
    Ok(())
}

fn validate_cmp(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate CMP has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for CMP
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_cpx(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate CPX has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for CPX
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::Absolute => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_cpy(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate CPY has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for CPY
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::Absolute => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_dec(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate DEC has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for DEC
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_eor(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate EOR has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for EOR
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_inc(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate INC has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for INC
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_jmp(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate JMP has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for JMP
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Absolute => Ok(()),
            AddressMode::Indirect => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => Ok(())
    }
}

fn validate_jsr(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate JSR has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for JSR
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Absolute => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => Ok(())
    }
}

fn validate_lda(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate LDA has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for LDA
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_ldx(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate LDX has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for LDX
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageY => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_ldy(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate LDY has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for LDY
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_lsr(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate LSR has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an operand.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for LSR
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Accumulator => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_ora(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate ORA has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an operand.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for ORA
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_rol(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate ROL has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an operand.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for ROL
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Accumulator => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_ror(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate ROR has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an operand.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for ROR
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Accumulator => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_sbc(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate SBC has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an operand.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for SBC
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::Immediate => Ok(()),
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_sta(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate STA has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for STA
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            AddressMode::AbsoluteX => Ok(()),
            AddressMode::AbsoluteY => Ok(()),
            AddressMode::IndirectX => Ok(()),
            AddressMode::IndirectY => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_stx(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate STX has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for STX
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageY => Ok(()),
            AddressMode::Absolute => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}

fn validate_sty(instr: &Instruction, line_num: usize) -> Result<(), AssemblerError> {
    // Validate STY has an operand
    if instr.operand.is_none() {
        return Err(AssemblerError::InvalidOpCode {
            msg: format!("{} requires an address.", instr.opcode),
            line: line_num,
        });
    }

    // Now validate that the addressing mode is valid for STY
    let operand = instr.clone().operand.unwrap();
    match operand {
        Operand::Address(address) => match address.address_mode {
            AddressMode::ZeroPage => Ok(()),
            AddressMode::ZeroPageX => Ok(()),
            AddressMode::Absolute => Ok(()),
            _ => {
                return Err(AssemblerError::InvalidOpCode {
                    msg: format!(
                        "{} does not support {} addressing.",
                        instr.opcode, address.address_mode
                    ),
                    line: line_num,
                })
            }
        },
        _ => {
            return Err(AssemblerError::InvalidOpCode {
                msg: format!("{} does not support labels", instr.opcode),
                line: line_num,
            })
        }
    }
}
