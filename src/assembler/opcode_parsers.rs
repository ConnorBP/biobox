use crate::assembler::Token;
use crate::instructions::Opcode;
use nom::types::CompleteStr;
use nom::*;

named!(pub opcode_load<CompleteStr, Token>,
    do_parse!(
        tag_no_case!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

named!(pub opcode<CompleteStr, Token>,
        do_parse!(
            opcode: alpha1 >> ( Token::Op{code: Opcode::from(opcode)} )
        )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcodeparse_load() {
        //first of call check that the opcode is detected and parsed correctly
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        //assert that it is indeed casae insensitive
        let result = opcode(CompleteStr("LOAD"));
        assert_eq!(result.is_ok(), true);

        //tests that an invalid opcode isn't recognized
        let result = opcode(CompleteStr("aold"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }

    #[test]
    fn test_opcode_parse() {
        //first of call check that the opcode is detected and parsed correctly
        let result = opcode(CompleteStr("nop"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::NOP });
        assert_eq!(rest, CompleteStr(""));

        //assert that it is indeed casae insensitive
        let result = opcode(CompleteStr("BETW"));
        assert_eq!(result.is_ok(), true);

        //tests that an invalid opcode isn't recognized
        let result = opcode(CompleteStr("nope"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
