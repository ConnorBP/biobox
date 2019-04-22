use super::directive_parsers::directive;
use super::label_parsers::*;
use super::opcode_parsers::*;
use super::operand_parsers::operand;
use super::Token;
use crate::instructions::Opcode;
use nom::multispace;

use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(Token::Op { code }) => match code {
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

        //pad any empty space out of the total 32 bits with 0
        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            //Add a register token to the results if found
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            //Add an integer token to the results if found
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                //bytes are placed in the vector in little-endian order
                //required to store as u16 (integers take up 16 bits of an instructions 32) for larger numbers
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                //opcodes (load, jmp, add, etc..) should not be in an operand field (after another opcode)
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }

    pub fn is_valid(&self) -> bool {
        //if there is no opcode then there has to be a directive TODO: CHECK FOR DIRECTIVES
        self.opcode != Some(Token::Op { code: Opcode::IGL }) && self.opcode != None
    }
}

/// Will try to parse out any of the Instruction forms
named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_format |
            directive
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
        o: opt!(opcode) >>
        l: opt!(label_declaration) >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                label: l,
                directive: None,
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

    #[test]
    fn test_parse_instruction_format() {
        let result = instruction_format(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    label: None,
                    directive: None,
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
