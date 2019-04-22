use super::instruction_parsers::AssemblerInstruction;
use super::operand_parsers::operand;
use super::Token;
use nom::types::CompleteStr;
use nom::{alpha1, multispace};

/// Directive format
/// .directivename
named!(pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            directive_format
        ) >>
        (
            ins
        )
    )
);

named!(directive_format<CompleteStr, AssemblerInstruction>,
    do_parse!(
        tag!(".") >>
        name: directive_declaration >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: None,
                label: None,
                directive: Some(name),
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

named!(directive_declaration<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive{name: name.to_string()}
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directive_declaration() {
        let result = directive_declaration(CompleteStr(".test"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Directive {
                name: "test".to_string()
            }
        );
        let result = directive_declaration(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_directive() {
        let result = directive(CompleteStr("..test"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            AssemblerInstruction {
                opcode: None,
                label: None,
                directive: Some(Token::Directive {
                    name: "test".to_string()
                }),
                operand1: None,
                operand2: None,
                operand3: None
            }
        );

        let result = directive(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }
}
