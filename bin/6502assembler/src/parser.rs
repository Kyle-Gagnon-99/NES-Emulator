use nes_lib::instructions::AddressMode;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while, take_while1, take_while_m_n},
    character::complete::{alpha1, char, line_ending, multispace0, multispace1, not_line_ending},
    combinator::{all_consuming, cut, eof, map, opt, peek, recognize, rest},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    directive::Directive,
    error::AssemblerError,
    instruction::{Address, Instruction, Operand},
    validation::validate_instruction,
};

#[derive(Debug, PartialEq, Serialize)]
pub enum Line {
    EmptyLine,
    Comment,
    Label(String),
    Instruction(Instruction),
    Directive(Directive),
}

fn parse_empty_line(input: &str) -> IResult<&str, ()> {
    let (_, _) = terminated(multispace0, alt((tag("\n"), eof)))(input)?;
    Ok(("", ()))
}

fn parse_comment_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = preceded(
        multispace0,
        tuple((preceded(char(';'), not_line_ending), opt(line_ending))),
    )(input)?;
    Ok((input, ()))
}

fn parse_label(input: &str) -> IResult<&str, String> {
    let (input, (_, label, _, _)) = all_consuming(tuple((
        multispace0,
        recognize(take_while1(|c: char| c.is_alphanumeric() || c == '_')),
        tag(":"),
        multispace0,
    )))(input)?;

    Ok((input, label.to_string()))
}

fn parse_directive(input: &str) -> IResult<&str, Result<Directive, AssemblerError>> {
    let (input, (_, _, directive, _, _, argument, _)) = all_consuming(tuple((
        multispace0,
        tag("."),
        alpha1,
        multispace1,
        tag("$"),
        take_while1(|c: char| c.is_ascii_hexdigit()),
        multispace0,
    )))(input)?;

    let directive = match directive.to_lowercase().as_str() {
        "org" => {
            match u16::from_str_radix(argument, 16)
                .map_err(|_| AssemblerError::ParseErrorNom("Failed to parse to u16".to_string()))
            {
                Ok(address) => Ok(Directive::Org(address)),
                Err(e) => Err(e),
            }
        }
        _ => Err(AssemblerError::InvalidDirective(directive.to_string())),
    };

    Ok((input, directive))
}

fn parse_operation(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() && c != ':')(input)
}

fn parse_operand(input: &str, line_num: usize) -> IResult<&str, Result<Operand, AssemblerError>> {
    alt((
        map(
            tuple((tag("#$"), multispace0, is_not(", ;\n"))),
            |(_, _, operand)| {
                let address = match u32::from_str_radix(operand, 16) {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(AssemblerError::ParseError {
                            msg: e.to_string(),
                            line: line_num,
                        })
                    }
                };
                Ok(Operand::Address(Address {
                    address,
                    address_mode: AddressMode::Immediate,
                }))
            },
        ), // Immediate
        map(
            tuple((
                tag("$"),
                multispace0,
                is_not(", ;\n"),
                opt(tuple((multispace0, tag(","), multispace0, is_not(" ;\n")))),
            )),
            |(_, _, operand, idx)| {
                let address = match u32::from_str_radix(operand, 16) {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(AssemblerError::ParseError {
                            msg: e.to_string(),
                            line: line_num,
                        })
                    }
                };
                let address_mode = match idx {
                    Some((_, _, _, tail)) if tail == "X" => {
                        if operand.len() > 2 {
                            AddressMode::AbsoluteX
                        } else {
                            AddressMode::ZeroPageX
                        }
                    }
                    Some((_, _, _, tail)) if tail == "Y" => {
                        if operand.len() > 2 {
                            AddressMode::AbsoluteY
                        } else {
                            AddressMode::ZeroPageY
                        }
                    }
                    _ => {
                        if operand.len() > 2 {
                            AddressMode::Absolute
                        } else {
                            AddressMode::ZeroPage
                        }
                    }
                };
                Ok(Operand::Address(Address {
                    address,
                    address_mode,
                }))
            },
        ), // Zero Page, Zero Page, X; Absolute, Absolute, Y
        map(
            tuple((
                tag("("),
                multispace0,
                tag("$"),
                multispace0,
                take_while1(|c: char| c != ',' && !c.is_whitespace()),
                multispace0,
                tag(","),
                multispace0,
                alt((tag("X"), tag("x"))),
                multispace0,
                tag(")"),
            )),
            |(_, _, _, _, operand, _, _, _, _, _, _)| {
                let address = match u32::from_str_radix(operand, 16) {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(AssemblerError::ParseError {
                            msg: e.to_string(),
                            line: line_num,
                        })
                    }
                };
                Ok(Operand::Address(Address {
                    address,
                    address_mode: AddressMode::IndirectX,
                }))
            },
        ),
        map(
            tuple((
                tag("("),
                multispace0,
                tag("$"),
                multispace0,
                take_while_m_n(1, 2, |c: char| c.is_ascii_hexdigit()),
                multispace0,
                tag(")"),
                multispace0,
                tag(","),
                multispace0,
                alt((tag("Y"), tag("y"))),
            )),
            |(_, _, _, _, operand, _, _, _, _, _, _)| {
                let address = match u32::from_str_radix(operand, 16) {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(AssemblerError::ParseError {
                            msg: e.to_string(),
                            line: line_num,
                        })
                    }
                };
                Ok(Operand::Address(Address {
                    address,
                    address_mode: AddressMode::IndirectY,
                }))
            },
        ),
        map(
            tuple((
                tag("("),
                multispace0,
                tag("$"),
                multispace0,
                take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()),
                multispace0,
                tag(")"),
            )),
            |(_, _, _, _, operand, _, _)| {
                let address = match u32::from_str_radix(operand, 16) {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(AssemblerError::ParseError {
                            msg: e.to_string(),
                            line: line_num,
                        })
                    }
                };
                Ok(Operand::Address(Address {
                    address,
                    address_mode: AddressMode::Indirect,
                }))
            },
        ),
        map(tuple((alt((tag("A"), tag("a"))), multispace0)), |(_, _)| {
            Ok(Operand::Address(Address {
                address: 0,
                address_mode: AddressMode::Accumulator,
            }))
        }),
        map(
            recognize(pair(
                alt((alpha1::<&str, _>, tag("_"))),
                take_while(|c: char| c.is_alphanumeric() || c == '_'),
            )),
            |label| Ok(Operand::Label(label.to_string())),
        ),
    ))(input)
}

fn parse_until_comment_or_line_ending(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c != ';' && c != '\n' && c != '\r')(input)
}

fn parse_instruction(
    input: &str,
    line_num: usize,
) -> IResult<&str, Result<Instruction, AssemblerError>> {
    let (input, (_, opcode, _, operand, _, comment)) = all_consuming(tuple((
        multispace0,
        parse_operation,
        peek(alt((multispace0, line_ending, eof))),
        opt(preceded(
            multispace1,
            cut(parse_until_comment_or_line_ending),
        )),
        multispace0,
        opt(tuple((multispace0, tag(";"), rest))),
    )))(input)?;

    if comment.is_some() && operand.is_none() {
        return Ok((
            input,
            Ok(Instruction {
                opcode: opcode.to_string(),
                operand: None,
            }),
        ));
    }

    let operand = match operand {
        Some(s) => {
            if s.trim_start().starts_with(';') || s.trim().is_empty() {
                Ok(None)
            } else {
                match parse_operand(&s, line_num) {
                    Ok((_, operand)) => operand.map(Some),
                    Err(e) => return Ok((input, Err(AssemblerError::IOError(e.to_string())))),
                }
            }
        }
        None => Ok(None),
    };

    let operand = match operand {
        Ok(operand) => operand,
        Err(e) => return Ok((input, Err(e))),
    };

    Ok((
        input,
        Ok(Instruction {
            opcode: opcode.to_string(),
            operand,
        }),
    ))
}

pub fn parse_line(input: &str, line_num: usize) -> Result<Line, AssemblerError> {
    let parse_result = terminated(
        alt((
            map(
                |i| parse_instruction(i, line_num),
                |instr_res| match instr_res {
                    Ok(instr) => match validate_instruction(&instr, line_num) {
                        Ok(_) => Ok(Line::Instruction(instr)),
                        Err(e) => return Err(e),
                    },
                    Err(e) => return Err(e),
                },
            ),
            map(parse_comment_line, |_| Ok(Line::Comment)),
            map(parse_label, |label| Ok(Line::Label(label))),
            map(parse_empty_line, |_| Ok(Line::EmptyLine)),
            map(parse_directive, |directive_res| match directive_res {
                Ok(directive) => Ok(Line::Directive(directive)),
                Err(e) => return Err(e),
            }),
        )),
        opt(line_ending),
    )(input);

    match parse_result {
        Ok((_, line)) => line,
        Err(_) => Err(AssemblerError::ParseError {
            line: line_num,
            msg: "Unexpected token".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    use super::*;

    fn setup() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    #[test]
    fn test_parse_empty_new_line_no_spaces() {
        assert_eq!(parse_empty_line("\n"), Ok(("", ())))
    }

    #[test]
    fn test_parse_empty_line_with_spaces() {
        assert_eq!(parse_empty_line("    \n"), Ok(("", ())))
    }

    #[test]
    fn test_parse_empty_line_with_tabs() {
        assert_eq!(parse_empty_line("\t\n"), Ok(("", ())))
    }

    #[test]
    fn test_parse_comment_no_space_no_line_end() {
        assert_eq!(parse_comment_line("; Hi"), Ok(("", ())))
    }

    #[test]
    fn test_parse_comment_space_line_end_no_content() {
        assert_eq!(parse_comment_line("    ;\n"), Ok(("", ())))
    }

    #[test]
    fn test_parse_label_no_space() {
        assert_eq!(parse_label("decrement:"), Ok(("", "decrement".to_string())))
    }

    #[test]
    fn test_parse_label_with_spaces() {
        assert_eq!(
            parse_label("    decrement:"),
            Ok(("", "decrement".to_string()))
        )
    }

    #[test]
    fn test_parse_label_with_spaces_end() {
        assert_eq!(
            parse_label("    decrement:  "),
            Ok(("", "decrement".to_string()))
        )
    }

    #[test]
    fn test_parse_label_with_underscore_in_front() {
        assert_eq!(
            parse_label("_decrement: "),
            Ok(("", "_decrement".to_string()))
        )
    }

    #[test]
    fn test_parse_label_snake_case() {
        assert_eq!(
            parse_label("decrement_now:"),
            Ok(("", "decrement_now".to_string()))
        )
    }

    #[test]
    fn test_parse_label_capital_letters() {
        assert_eq!(parse_label("DECREMENT:"), Ok(("", "DECREMENT".to_string())))
    }

    #[test]
    fn test_parse_directive_org_valid_address() {
        assert_eq!(
            parse_directive(".ORG $8000"),
            Ok(("", Ok(Directive::Org(0x8000))))
        )
    }

    #[test]
    fn test_parse_directive_org_lowercase_valid_address() {
        assert_eq!(
            parse_directive(".org $8000"),
            Ok(("", Ok(Directive::Org(0x8000))))
        )
    }

    #[test]
    fn test_parse_directive_org_invalid_address() {
        assert_eq!(
            parse_directive(".ORG $80000"),
            Ok((
                "",
                Err(AssemblerError::ParseErrorNom(
                    "Failed to parse to u16".to_string()
                ))
            ))
        )
    }

    #[test]
    fn test_parse_directive_invalid_directive() {
        assert_eq!(
            parse_directive(".NOTHING $8000"),
            Ok((
                "",
                Err(AssemblerError::InvalidDirective("NOTHING".to_string()))
            ))
        )
    }

    #[test]
    fn test_parse_jmp_with_label() {
        assert_eq!(
            parse_operand("JUMP", 0),
            Ok(("", Ok(Operand::Label("JUMP".to_string()))))
        )
    }

    #[test]
    fn test_parse_operation_only() {
        assert_eq!(parse_operation("LDA"), Ok(("", "LDA")))
    }

    #[test]
    fn test_parse_operand_accumalator() {
        assert_eq!(
            parse_operand("A", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0,
                    address_mode: AddressMode::Accumulator
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_accumalator_lowercase() {
        assert_eq!(
            parse_operand("a", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0,
                    address_mode: AddressMode::Accumulator
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_immediate() {
        assert_eq!(
            parse_operand("#$44", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::Immediate
                }))
            ))
        );
    }

    #[test]
    fn test_parse_operand_absolute() {
        assert_eq!(
            parse_operand("$4400", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x4400,
                    address_mode: AddressMode::Absolute
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_absolute_x() {
        assert_eq!(
            parse_operand("$4400,X", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x4400,
                    address_mode: AddressMode::AbsoluteX
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_absolute_y() {
        assert_eq!(
            parse_operand("$4400,Y", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x4400,
                    address_mode: AddressMode::AbsoluteY
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_indirect() {
        assert_eq!(
            parse_operand("($4400)", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x4400,
                    address_mode: AddressMode::Indirect
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_indirect_x() {
        assert_eq!(
            parse_operand("($44,X)", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::IndirectX
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_indirect_y() {
        assert_eq!(
            parse_operand("($44),Y", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::IndirectY
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_zero_page() {
        assert_eq!(
            parse_operand("$44", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::ZeroPage
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_zero_page_x() {
        assert_eq!(
            parse_operand("$44,X", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::ZeroPageX
                }))
            ))
        )
    }

    #[test]
    fn test_parse_operand_zero_page_y() {
        assert_eq!(
            parse_operand("$44,Y", 0),
            Ok((
                "",
                Ok(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::ZeroPageY
                }))
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_no_comment_beginning_spacing() {
        assert_eq!(
            parse_instruction("    LDA #$44", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0x44,
                        address_mode: AddressMode::Immediate
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_with_comment_multi_spacing() {
        assert_eq!(
            parse_instruction("    LDA #$44    ; This is a comment", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0x44,
                        address_mode: AddressMode::Immediate
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_with_comment_multi_spacing_zero_page_address() {
        assert_eq!(
            parse_instruction("    LDA $44,X          ; Another comment", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0x44,
                        address_mode: AddressMode::ZeroPageX
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_absolute_address() {
        assert_eq!(
            parse_instruction("LDA $c000", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0xC000,
                        address_mode: AddressMode::Absolute
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_absolute_x_address() {
        assert_eq!(
            parse_instruction("LDA $c000,X", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0xC000,
                        address_mode: AddressMode::AbsoluteX
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_absolute_y_address() {
        assert_eq!(
            parse_instruction("LDA $c000,Y", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0xC000,
                        address_mode: AddressMode::AbsoluteY
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_indirect_address() {
        assert_eq!(
            parse_instruction("LDA ($c000)", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0xC000,
                        address_mode: AddressMode::Indirect
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_indexed_indirect_address() {
        assert_eq!(
            parse_instruction("LDA ($c0,X)", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0xC0,
                        address_mode: AddressMode::IndirectX
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_brk_with_comment_no_operand() {
        assert_eq!(
            parse_instruction("    BRK  ; End of program", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "BRK".to_string(),
                    operand: None
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_brk_with_eof() {
        assert_eq!(
            parse_instruction("BRK", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "BRK".to_string(),
                    operand: None
                })
            ))
        )
    }

    #[test]
    fn test_parse_instruction_lda_with_comment_no_content() {
        assert_eq!(
            parse_instruction("LDA #$44 ;", 0),
            Ok((
                "",
                Ok(Instruction {
                    opcode: "LDA".to_string(),
                    operand: Some(Operand::Address(Address {
                        address: 0x44,
                        address_mode: AddressMode::Immediate
                    }))
                })
            ))
        )
    }

    #[test]
    fn test_parse_line_instruction() {
        assert_eq!(
            parse_line("LDA #$44", 0),
            Ok(Line::Instruction(Instruction {
                opcode: "LDA".to_string(),
                operand: Some(Operand::Address(Address {
                    address: 0x44,
                    address_mode: AddressMode::Immediate
                }))
            }))
        )
    }

    #[test]
    fn test_parse_line_label() {
        assert_eq!(
            parse_line("decrement:", 0),
            Ok(Line::Label("decrement".to_string()))
        )
    }

    #[test]
    fn test_parse_line_comment() {
        assert_eq!(parse_line("; This is a comment line", 0), Ok(Line::Comment))
    }

    #[test]
    fn test_parse_line_empty_line() {
        setup();
        assert_eq!(parse_line("\n\n", 0), Ok(Line::EmptyLine))
    }

    #[test]
    fn test_parse_invalid_line() {
        assert_eq!(
            parse_line("======", 1),
            Err(AssemblerError::ParseError {
                line: 1,
                msg: "Unexpected token".to_string()
            })
        );
    }

    #[test]
    fn test_parse_line_directive() {
        assert_eq!(
            parse_line(".ORG $8000", 0),
            Ok(Line::Directive(Directive::Org(0x8000)))
        )
    }
}
