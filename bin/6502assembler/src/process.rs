use std::collections::HashMap;

use nes_lib::instructions::AddressMode;
use tracing::{info, error};

use crate::{
    directive::Directive,
    instruction::{Address, Operand, Instruction},
    parser::Line, error::AssemblerError,
};

pub fn proccess_instructions(mut lines: Vec<(Line, usize)>) -> Result<(Vec<Instruction>, u32), AssemblerError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    #[allow(unused_assignments)]
    let mut pointer: u32 = 0;
    let mut start_pos: u32 = 0;

    let mut label_map: HashMap<String, u32> = HashMap::new();

    // First, check if we have base address
    for (line, _) in lines.iter() {
        match line {
            Line::Directive(directive) => match directive {
                Directive::Org(address) => start_pos = *address as u32,
            },
            _ => {}
        }
    }

    info!("Found ORG directive. Starting at 0x{:04X}", start_pos);
    pointer = start_pos;

    // Do a first pass, we might not see all labels yet so just keep going
    for (line, _) in lines.iter() {
        match line {
            Line::Label(label) => {
                label_map.insert(label.to_string(), pointer);
            },
            Line::Instruction(instruction) => {
                pointer += instruction.size() as u32;
            },
            _ => {}
        }
    }

    // Second pass, now we have all labels
    for (line, line_num) in lines.iter_mut() {
        match line {
            Line::Instruction(instr) => {
                if let Some(Operand::Label(label)) = &instr.operand {
                    match label_map.get(label) {
                        Some(label_adddress) => {
                            instr.operand = Some(Operand::Address(Address {
                                address: *label_adddress,
                                address_mode: AddressMode::Absolute
                            }))
                        },
                        None => {
                            error!("Didn't find label: {}", label);
                            return Err(AssemblerError::InvalidLabel { msg: format!("Did not find {}", label), line: *line_num })
                        }
                    }
                }
                instructions.push(instr.clone());
            },
            _ => {}
        }
    }

    Ok((instructions, start_pos))
}
