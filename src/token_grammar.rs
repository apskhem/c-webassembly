use std::collections::VecDeque;

use crate::token;
use crate::grammar;
use crate::grammar::Grammar;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenGrammar {
    Comment(Comment),
    Keyword(Keyword),
    Type(Type),
    Identifier(Identifier),
    Literal(Literal),
    Symbol(Symbol)
}

#[derive(Debug, Clone, PartialEq)]
enum Comment {
    Any
}

#[derive(Debug, Clone, PartialEq)]
enum Keyword {
    ByOriginal(token::Keyword),
    Any
}

#[derive(Debug, Clone, PartialEq)]
enum Type {
    ByOriginal(token::Type),
    Any
}

#[derive(Debug, Clone, PartialEq)]
enum Identifier {
    Any
}

#[derive(Debug, Clone, PartialEq)]
enum Literal {
    AnyString,
    AnyNumeric,
    Any
}

#[derive(Debug, Clone, PartialEq)]
enum Symbol {
    ByOriginal(token::Symbol),
    AnyUnary,
    AnyBinary,
    Any
}

impl TokenGrammar {
    pub fn is_match(&self, token: &token::Token) -> bool {
        return match (self, token) {
            // comments
            (TokenGrammar::Comment(Comment::Any), token::Token::Comment(_)) => {
                true
            },
            // keywords
            (TokenGrammar::Keyword(Keyword::Any), token::Token::Keyword(_)) => {
                true
            },
            (TokenGrammar::Keyword(Keyword::ByOriginal(x)), token::Token::Keyword(y)) => {
                x == y
            },
            // types
            (TokenGrammar::Type(Type::Any), token::Token::Type(_)) => {
                true
            },
            (TokenGrammar::Type(Type::ByOriginal(x)), token::Token::Type(y)) => {
                x == y
            },
            // identifiers
            (TokenGrammar::Identifier(Identifier::Any), token::Token::Identifier(_)) => {
                true
            },
            // literals
            (TokenGrammar::Literal(Literal::Any), token::Token::Literal(_)) => {
                true
            },
            (TokenGrammar::Literal(Literal::AnyNumeric), token::Token::Literal(token::Literal::Numeric(_))) => {
                true
            },
            (TokenGrammar::Literal(Literal::AnyString), token::Token::Literal(token::Literal::String(_))) => {
                true
            },
            // symbols
            (TokenGrammar::Symbol(Symbol::Any), token::Token::Symbol(_)) => {
                true
            },
            (TokenGrammar::Symbol(Symbol::ByOriginal(x)), token::Token::Symbol(y)) => {
                x == y
            },
            (TokenGrammar::Symbol(Symbol::AnyUnary), token::Token::Symbol(y)) => {
                y == &token::Symbol::Plus
                || y == &token::Symbol::Minus
                || y == &token::Symbol::BitwiseNot
                || y == &token::Symbol::LogicalNegation
            },
            (TokenGrammar::Symbol(Symbol::AnyBinary), token::Token::Symbol(y)) => {
                y == &token::Symbol::Plus
                || y == &token::Symbol::Minus
                || y == &token::Symbol::Asterisk
                || y == &token::Symbol::Solidus
                || y == &token::Symbol::Modulo
                || y == &token::Symbol::Equal
                || y == &token::Symbol::NotEqual
                || y == &token::Symbol::LessThan
                || y == &token::Symbol::GreaterThan
                || y == &token::Symbol::LessThanOrEqual
                || y == &token::Symbol::GreaterThanOrEqual
                || y == &token::Symbol::BitwiseAnd
                || y == &token::Symbol::BitwiseOr
                || y == &token::Symbol::BitwiseXor
                || y == &token::Symbol::BitwiseNot
                || y == &token::Symbol::ShiftLeftLogical
                || y == &token::Symbol::ShiftRightArithmatic
                || y == &token::Symbol::ShiftRightLogical
                || y == &token::Symbol::LogicalAnd
                || y == &token::Symbol::LogicalOr
                || y == &token::Symbol::PipeForward
            },
            _ => false
        };
    }

    pub const fn any_comment() -> Self {
        return TokenGrammar::Comment(Comment::Any);
    }

    pub const fn from_keyword(o: token::Keyword) -> Self {
        return TokenGrammar::Keyword(Keyword::ByOriginal(o));
    }

    pub const fn any_keyword() -> Self {
        return TokenGrammar::Keyword(Keyword::Any);
    }

    pub const fn from_type(o: token::Type) -> Self {
        return TokenGrammar::Type(Type::ByOriginal(o));
    }

    pub const fn any_type() -> Self {
        return TokenGrammar::Type(Type::Any);
    }

    pub const fn any_identifier() -> Self {
        return TokenGrammar::Identifier(Identifier::Any);
    }

    pub const fn any_numeric_literal() -> Self {
        return TokenGrammar::Literal(Literal::AnyNumeric);
    }

    pub const fn any_string_literal() -> Self {
        return TokenGrammar::Literal(Literal::AnyString);
    }

    pub const fn any_literal() -> Self {
        return TokenGrammar::Literal(Literal::Any);
    }

    pub const fn from_symbol(o: token::Symbol) -> Self {
        return TokenGrammar::Symbol(Symbol::ByOriginal(o));
    }

    pub const fn any_unary_symbol() -> Self {
        return TokenGrammar::Symbol(Symbol::AnyUnary);
    }

    pub const fn any_binary_symbol() -> Self {
        return TokenGrammar::Symbol(Symbol::AnyBinary);
    }

    pub const fn any_symbol() -> Self {
        return TokenGrammar::Symbol(Symbol::Any);
    }
}

impl Grammar for TokenGrammar {
    fn process(&mut self, token: &token::Token) -> grammar::Result {
        if self.is_match(token) {
            return grammar::Result::Consumed(VecDeque::new());
        }
        else {
            return grammar::Result::Unexpected(format!("mismatched token: '{:?}' compared with '{:?}'", self, token).into());
        }
    }

    fn is_done(&self) -> bool {
        return true;
    }

    fn info(&self) -> String {
        return format!("Token");
    }
}