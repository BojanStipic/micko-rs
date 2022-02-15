use chumsky::{
    error::Simple,
    primitive::{choice, end, filter_map, just},
    recursive::recursive,
    Error, Parser,
};

use crate::{
    ast::{
        Arop, Exp, Function, Identifier, Literal, Parameter, Program, RelExp, Relop, Statement,
        Type, Variable,
    },
    lexer::{self, Ctrl, Keyword, Op, Token},
};

impl Program {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        Function::parser()
            .repeated()
            .at_least(1)
            .then_ignore(end())
            .map(|functions| Self { functions })
    }
}

impl Function {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        Type::parser()
            .then(Identifier::parser())
            .then(Parameter::parser().or_not().delimited_by(
                just(Token::Ctrl(Ctrl::LParen)),
                just(Token::Ctrl(Ctrl::RParen)),
            ))
            .then_ignore(just(Token::Ctrl(Ctrl::LBracket)))
            .then(Variable::parser().repeated())
            .then(Statement::parser().repeated())
            .then_ignore(just(Token::Ctrl(Ctrl::RBracket)))
            .map(
                |((((return_type, name), parameter), variables), statements)| Self {
                    return_type,
                    name,
                    parameter,
                    variables,
                    statements,
                },
            )
    }
}

impl Parameter {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        Type::parser()
            .then(Identifier::parser())
            .map(|(kind, identifier)| Self { kind, identifier })
    }
}

impl Variable {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        Type::parser()
            .then(Identifier::parser())
            .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
            .map(|(kind, identifier)| Self { kind, identifier })
    }
}

impl Statement {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        recursive(|statement| {
            let assignment_statement = Identifier::parser()
                .then_ignore(just(Token::Op(Op::Assign)))
                .then(Exp::parser())
                .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
                .map(|(lhs, rhs)| Self::Assignment { lhs, rhs });

            let if_statement = just(Token::Keyword(Keyword::If))
                .ignore_then(RelExp::parser().delimited_by(
                    just(Token::Ctrl(Ctrl::LParen)),
                    just(Token::Ctrl(Ctrl::RParen)),
                ))
                .then(statement.clone())
                .then(
                    just(Token::Keyword(Keyword::Else))
                        .ignore_then(statement.clone())
                        .or_not(),
                )
                .map(|((condition, if_part), else_part)| Self::If {
                    condition,
                    if_part: Box::new(if_part),
                    else_part: else_part.map(Box::new),
                });

            let return_statement = just(Token::Keyword(Keyword::Return))
                .ignore_then(Exp::parser())
                .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
                .map(|value| Self::Return { value });

            let compound_statement = statement
                .repeated()
                .delimited_by(
                    just(Token::Ctrl(Ctrl::LBracket)),
                    just(Token::Ctrl(Ctrl::RBracket)),
                )
                .map(Self::Compound);

            choice((
                assignment_statement,
                if_statement,
                return_statement,
                compound_statement,
            ))
        })
    }
}

impl RelExp {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        Exp::parser()
            .then(Relop::parser())
            .then(Exp::parser())
            .map(|((lhs, operator), rhs)| Self { lhs, operator, rhs })
    }
}

impl Exp {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        recursive(|exp| {
            let atom = choice((
                Identifier::parser()
                    .then(exp.clone().map(Box::new).or_not().delimited_by(
                        just(Token::Ctrl(Ctrl::LParen)),
                        just(Token::Ctrl(Ctrl::RParen)),
                    ))
                    .map(|(name, arg)| Self::FunctionCall { name, arg }),
                Identifier::parser().map(Exp::Identifier),
                Literal::parser().map(Exp::Literal),
                exp.delimited_by(
                    just(Token::Ctrl(Ctrl::LParen)),
                    just(Token::Ctrl(Ctrl::RParen)),
                ),
            ));

            let product_op = just(Token::Op(Op::Mul))
                .to(Arop::Mul)
                .or(just(Token::Op(Op::Div)).to(Arop::Div));
            let product = atom
                .clone()
                .then(product_op.then(atom).repeated())
                .foldl(|a, (op, b)| Exp::Binary(Box::new(a), op, Box::new(b)));

            let sum_op = just(Token::Op(Op::Add))
                .to(Arop::Add)
                .or(just(Token::Op(Op::Sub)).to(Arop::Sub));
            product
                .clone()
                .then(sum_op.then(product).repeated())
                .foldl(|a, (op, b)| Exp::Binary(Box::new(a), op, Box::new(b)))
        })
    }
}

impl Type {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        choice((
            just(Token::Type(lexer::Type::Int)).to(Self::Int),
            just(Token::Type(lexer::Type::Unsigned)).to(Self::Unsigned),
        ))
    }
}

impl Relop {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        choice((
            just(Token::Op(Op::Lt)).to(Self::Lt),
            just(Token::Op(Op::Gt)).to(Self::Gt),
            just(Token::Op(Op::Le)).to(Self::Le),
            just(Token::Op(Op::Ge)).to(Self::Ge),
            just(Token::Op(Op::Eq)).to(Self::Eq),
            just(Token::Op(Op::Ne)).to(Self::Ne),
        ))
    }
}

impl Identifier {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        filter_map(|span, tok| match tok {
            Token::Ident(x) => Ok(Self(x)),
            _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
        })
        .labelled("identifier")
    }
}

impl Literal {
    pub fn parser() -> impl Parser<Token, Self, Error = Simple<Token>> + Clone {
        filter_map(|span, tok| match tok {
            Token::IntNum(num) => Ok(Self::IntNum(num)),
            Token::UintNum(num) => Ok(Self::UintNum(num)),
            _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
        })
        .labelled("literal")
    }
}
