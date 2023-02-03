use std::{fmt::Display, ops::Range};

use chumsky::{
    error::Simple,
    primitive::{choice, end, just, one_of, take_until},
    recovery::skip_then_retry_until,
    text::{ident, int, keyword, newline, TextParser},
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Keyword(Keyword),
    Type(Type),
    Ident(String),
    UintNum(u32),
    IntNum(i32),
    Ctrl(Ctrl),
    Op(Op),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Unsigned,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ctrl {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    Assign,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(x) => match x {
                Keyword::If => write!(f, "if"),
                Keyword::Else => write!(f, "else"),
                Keyword::Return => write!(f, "return"),
            },
            Self::Type(x) => match x {
                Type::Int => write!(f, "int"),
                Type::Unsigned => write!(f, "unsigned"),
            },
            Self::Ident(x) => write!(f, "{x}"),
            Self::IntNum(x) => write!(f, "{x}"),
            Self::UintNum(x) => write!(f, "{x}"),
            Self::Ctrl(x) => match x {
                Ctrl::LParen => write!(f, "("),
                Ctrl::RParen => write!(f, ")"),
                Ctrl::LBracket => write!(f, "{{"),
                Ctrl::RBracket => write!(f, "}}"),
                Ctrl::Semicolon => write!(f, ";"),
            },
            Self::Op(x) => match x {
                Op::Add => write!(f, "+"),
                Op::Sub => write!(f, "-"),
                Op::Mul => write!(f, "*"),
                Op::Div => write!(f, "/"),
                Op::Lt => write!(f, "<"),
                Op::Gt => write!(f, ">"),
                Op::Le => write!(f, "<="),
                Op::Ge => write!(f, ">="),
                Op::Eq => write!(f, "=="),
                Op::Ne => write!(f, "!="),
                Op::Assign => write!(f, "="),
            },
        }
    }
}

pub type Span = Range<usize>;
pub type Tokens = Vec<(Token, Span)>;

pub fn lexer() -> impl Parser<char, Tokens, Error = Simple<char>> {
    let single_line_comment = just("//").then(take_until(newline())).ignored();

    let multi_line_comment = just("/*").then(take_until(just("*/"))).ignored();

    let comment = single_line_comment.or(multi_line_comment);

    choice((
        keyword("if").to(Token::Keyword(Keyword::If)),
        keyword("else").to(Token::Keyword(Keyword::Else)),
        keyword("return").to(Token::Keyword(Keyword::Return)),
        keyword("int").to(Token::Type(Type::Int)),
        keyword("unsigned").to(Token::Type(Type::Unsigned)),
        ident().map(Token::Ident),
        int(10)
            .then_ignore(just("u").or(just("U")))
            .map(|num: String| Token::UintNum(num.parse().unwrap())),
        one_of("-+")
            .or_not()
            .chain::<char, _, _>(int(10))
            .collect()
            .map(|num: String| Token::IntNum(num.parse().unwrap())),
        just("(").to(Token::Ctrl(Ctrl::LParen)),
        just(")").to(Token::Ctrl(Ctrl::RParen)),
        just("{").to(Token::Ctrl(Ctrl::LBracket)),
        just("}").to(Token::Ctrl(Ctrl::RBracket)),
        just(";").to(Token::Ctrl(Ctrl::Semicolon)),
        just("+").to(Token::Op(Op::Add)),
        just("-").to(Token::Op(Op::Sub)),
        just("*").to(Token::Op(Op::Mul)),
        just("/").to(Token::Op(Op::Div)),
        just("<=").to(Token::Op(Op::Le)),
        just(">=").to(Token::Op(Op::Ge)),
        just("==").to(Token::Op(Op::Eq)),
        just("!=").to(Token::Op(Op::Ne)),
        just("<").to(Token::Op(Op::Lt)),
        just(">").to(Token::Op(Op::Gt)),
        just("=").to(Token::Op(Op::Assign)),
    ))
    .recover_with(skip_then_retry_until([]))
    .padded()
    .padded_by(comment.padded().repeated())
    .map_with_span(|token, span| (token, span))
    .repeated()
    .then_ignore(end())
}
