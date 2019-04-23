//our virtual machines list of OPCODES

use nom::types::CompleteStr;

use self::Opcode::*;
use std::slice::Iter;

//opcodes
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    //system
    LOAD = 1,
    ALOC = 18,

    //math
    ADD = 2,
    SUB = 3,
    INC = 19,
    DEC = 20,
    MUL = 4,
    DIV = 5,

    //comparison
    EQ = 9,
    NEQ = 10,
    GT = 11,
    LT = 12,
    GTEQ = 13,
    LTEQ = 14,
    BETW = 15,

    //jumps
    JMP = 6,
    JMPF = 7,
    JMPB = 8,
    JEQ = 16,

    //defaults
    HLT = 0,
    NOP = 17,
    IGL = 254,
}

impl Opcode {
    pub fn iterator() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode; 22] = [
            //system
            LOAD, ALOC, //math
            ADD, SUB, INC, DEC, MUL, DIV, //comparison
            EQ, NEQ, GT, LT, GTEQ, LTEQ, BETW, //jumps
            JMP, JMPF, JMPB, JEQ, //defaults
            HLT, NOP, IGL,
        ];
        OPCODES.iter()
    }

    pub fn get_list() {
        //let out = String::new();
        for opc in Opcode::iterator() {
            println!("{:?}", opc);
            //out.push_str();
        }
    }
}

//for converting a byte into the relivant opcode
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTEQ,
            14 => Opcode::LTEQ,
            15 => Opcode::BETW,
            16 => Opcode::JEQ,
            17 => Opcode::NOP,
            18 => Opcode::ALOC,
            19 => Opcode::INC,
            20 => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v.to_lowercase().as_str() {
            "hlt" => Opcode::HLT,
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
            "jmpb" => Opcode::JMPB,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gteq" => Opcode::GTEQ,
            "lteq" => Opcode::LTEQ,
            "betw" => Opcode::BETW,
            "jeq" => Opcode::JEQ,
            "nop" => Opcode::NOP,
            "aloc" => Opcode::ALOC,
            "inc" => Opcode::INC,
            "dec" => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

//instructions
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("gteq"));
        assert_eq!(opcode, Opcode::GTEQ);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
