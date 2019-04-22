use super::opcode_parsers::*;
use super::operand_parsers::operand;
use super::Token;
use crate::instructions::Opcode;
use nom::multispace;

use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                //bytes are placed in the vector in little-endian order
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }

    pub fn is_valid(&self) -> bool {
        self.opcode != Token::Op { code: Opcode::IGL }
    }
}

/// Will try to parse out any of the Instruction forms
named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_format
        ) >>
        (
            ins
        )
    )
);

/// Handles instructions in the following format:
/// LOAD $0 #42
named!(instruction_format<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )


);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instructions::Opcode;

    // #[test]
    // fn test_parse_instruction_form_one() {
    //     let result = instruction_one(CompleteStr("load $0 #42\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 //label: None,
    //                 opcode: Token::Op { code: Opcode::LOAD },
    //                 operand1: Some(Token::Register { reg_num: 0 }),
    //                 operand2: Some(Token::IntegerOperand { value: 42 }),
    //                 operand3: None
    //             }
    //         ))
    //     );
    // }

    #[test]
    fn test_parse_instruction_format() {
        let result = instruction_format(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
