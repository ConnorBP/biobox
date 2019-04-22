use crate::assembler::Token;
use crate::instructions::Opcode;
use nom::types::CompleteStr;

named!(pub opcode_load<CompleteStr, Token>,
    do_parse!(
        tag_no_case!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcodeparse_load() {
        //first of call check that the opcode is detected and parsed correctly
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        //assert that it is indeed casae insensitive
        let result = opcode_load(CompleteStr("LOAD"));
        assert_eq!(result.is_ok(), true);

        //tests that an invalid opcode isn't recognized
        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}
