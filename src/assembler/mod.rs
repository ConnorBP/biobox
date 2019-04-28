use crate::instructions::Opcode;

pub mod directive_parsers;
pub mod instruction_parsers;
pub mod label_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parsers;
pub mod register_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

// #[derive(Debug)]
// pub struct Assembler {
//     phase: AssemblerPhase,
// }

// #[derive(Debug)]
// pub struct Assembler {
//     pub phase: AssemblerPhase,
//     pub symbols: SymbolTable,
// }

// impl Assembler {
//     pub fn new() -> Assembler {
//         Assembler {
//             phase: AssemblerPhase::First(),
//             symbols: SymbolTable::new()
//         }
//     }
// }
