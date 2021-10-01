use std::convert::TryFrom;
use std::collections::VecDeque;
use regex::Regex;

use crate::definition;
use crate::grammar;
use crate::grammar::Grammar;

// enums
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Comment(Comment<'a>),
    Keyword(Keyword),
    Type(Type),
    Identifier(Identifier<'a>),
    Literal(Literal<'a>),
    Symbol(Symbol)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment<'a>(&'a str);

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Module,
    Function,
    Local,
    Mutable,
    Global,
    Memory,
    Table,
    Type,
    Return,
    If,
    Else,
    ElseIf,
    While,
    Break,
    Cont,
    TypeOf,
    Export,
    Import,
    As,
    From
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a>(&'a str);

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    // general
    Dot,                    // .
    Comma,                  // ,
    Colon,                  // :
    SemiColon,              // ;

    // operation
    Plus,                   // +
    Minus,                  // -
    Asterisk,               // *
    Solidus,                // /
    Modulo,                 // %
    Assignment,             // =
    Equal,                  // ==
    NotEqual,               // !=
    LessThan,               // <
    GreaterThan,            // >
    LessThanOrEqual,        // <=
    GreaterThanOrEqual,     // >=
    LeftArrow,              // <-
    RightArrow,             // ->
    BitwiseAnd,             // &
    BitwiseOr,              // |
    BitwiseXor,             // ^
    BitwiseNot,             // ~
    ShiftLeftLogical,       // <<
    ShiftRightArithmatic,   // >>
    ShiftRightLogical,      // >>>
    RotateLeft,             // %<
    RotateRight,            // %>
    LogicalNegation,        // !
    LogicalAnd,             // &&
    LogicalOr,              // ||
    Query,                  // ?
    DoubleColon,            // ::

    // brackets
    LeftBrace,              // {
    RightBrace,             // }
    LeftParenthese,         // (
    RightParenthese,        // )

    // regex
    RegexUnary,
    RegexBinary,
    RegexOperator,
    RegexAny
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // number types
    I32,
    I64,
    F32,
    F64,

    // reference types
    Fref,
    Xref,

    // memory types
    Page,

    // for matching
    RegexAny
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    Numeric(&'a str),
    String(&'a str)
}

pub struct RawToken<'a> {
    value: &'a str,
    range: (usize, usize)
}

// implementation
impl Token<'_> {
    pub fn is_match(&self, token: &Token) -> bool {
        return match (token, self) {
            // comment
            (Token::Comment(Comment(x)), Token::Comment(Comment(y))) => {
                Regex::new(y).unwrap().is_match(x)
            },
            // keyword
            (Token::Keyword(x), Token::Keyword(y)) => {
                x == y
            },
            // type
            (Token::Type(x), Token::Type(y)) => {
                y == &Type::RegexAny || x == y
            },
            // identifier
            (Token::Identifier(Identifier(x)), Token::Identifier(Identifier(y))) => {
                Regex::new(y).unwrap().is_match(x)
            },
            // numeric literal
            (Token::Literal(Literal::String(x)), Token::Literal(Literal::String(y))) => {
                Regex::new(y).unwrap().is_match(x)
            },
            // string literal
            (Token::Literal(Literal::Numeric(x)), Token::Literal(Literal::Numeric(y))) => {
                Regex::new(y).unwrap().is_match(x)
            },
            // symbol
            (Token::Symbol(x), Token::Symbol(y)) => {
                x == y
            },
            _ => false
        };
    }
}

impl<'a> Identifier<'a> {
    pub fn from_regex(s: &'a str) -> Self {
        return Identifier(s);
    }

    pub fn is_alphabetic_valid_char(c: char) -> bool {
        return c.is_alphabetic() || Identifier::is_extended_symbol(c);
    }

    pub fn is_alphanumeric_valid_char(c: char) -> bool {
        return c.is_alphanumeric() || Identifier::is_extended_symbol(c);
    }

    const fn is_extended_symbol(c: char) -> bool {
        return c == '_' || c == '$';
    }
}

impl Symbol {
    pub fn match_str(s: &str) -> bool {
        return definition::SYMBOL_TOKENS.iter().any(|x| return x.0.starts_with(s));
    }

    pub fn match_char(c: char) -> bool {
        return definition::SYMBOL_TOKENS.iter().any(|x| return x.0.starts_with(c));
    }
}

impl<'a> Literal<'a> {
    pub fn from_numeric_regex(s: &'a str) -> Self {
        return Literal::Numeric(s);
    }

    pub fn from_string_regex(s: &'a str) -> Self {
        return Literal::String(s);
    }
}

impl<'a> RawToken<'a> {
    pub const fn new(value: &'a str, range: (usize, usize)) -> Self {
        return Self {
            value,
            range
        }
    }

    pub fn value(&self) -> &str {
        return self.value;
    }

    pub const fn range(&self) -> &(usize, usize) {
        return &self.range;
    }
}

// implement tryFrom<T>
impl<'a> TryFrom<RawToken<'a>> for Token<'a> {
    type Error = String;
    fn try_from(value: RawToken<'a>) -> Result<Self, Self::Error> {
        let RawToken { value, range } = value;

        if let Ok(x) = Keyword::try_from(value) {
            return Ok(x.into());
        }
        else if let Ok(x) = Type::try_from(value) {
            return Ok(x.into());
        }
        else if let Ok(x) = Symbol::try_from(value) {
            return Ok(x.into());
        }
        else if let Ok(x) = Identifier::try_from(value) {
            return Ok(x.into());
        }
        else if let Ok(x) = Comment::try_from(value) {
            return Ok(x.into());
        }
        else if let Ok(x) = Literal::try_from(value) {
            return Ok(x.into());
        }
        
        return Err(format!("unexpected token: {}", value));
    }
}

// implement FromStr trait
impl<'a> TryFrom<&'a str> for Comment<'a> {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // single line comment
        if s.starts_with("//") {
            return Ok(Comment(s));
        }
        // multi line comment
        else if s.starts_with("/*") && s.ends_with("*/") {
            return Ok(Comment(s));
        }

        return Err("cannot parse the given raw value");
    }
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some(x) = definition::KEYWORD_TOKENS.iter().find(|x| return x.0 == s) {
            return Ok(x.1.clone());
        }
        
        return Err("cannot parse the given raw value");
    }
}

impl TryFrom<&str> for Type {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some(x) = definition::TYPE_TOKENS.iter().find(|x| return x.0 == s) {
            return Ok(x.1.clone());
        }
        
        return Err("cannot parse the given raw value");
    }
}

impl<'a> TryFrom<&'a str> for Identifier<'a> {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let is_started_valid = match s.chars().next() {
            Some(x) => Identifier::is_alphabetic_valid_char(x),
            _ => false
        };

        let is_all_valid = s.chars().all(|c| return Identifier::is_alphanumeric_valid_char(c));

        if is_started_valid && is_all_valid {
            return Ok(Identifier(s));
        }

        return Err("cannot parse the given raw value");
    }
}

impl<'a> TryFrom<&'a str> for Literal<'a> {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // is string literal
        // TODO: check stricter
        if s.starts_with('\"') && s.ends_with('\"') {
            return Ok(Literal::String(s))
        }
        // is nan
        else if Regex::new(r"^NaN$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        // is inf
        else if Regex::new(r"^Inf$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        // is integer
        else if Regex::new(r"^\d+$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        // is float
        else if Regex::new(r"^\d+\.\d+$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        // is binary
        else if Regex::new(r"^0b[01]+$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        // is octal
        else if Regex::new(r"^0o?[0-7]+$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        // is hex
        else if Regex::new(r"^0x[a-fA-F0-9]+$").unwrap().is_match(s) {
            return Ok(Literal::Numeric(s));
        }
        
        return Err("cannot parse the given raw value");
    }
}

impl TryFrom<&str> for Symbol {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some(x) = definition::SYMBOL_TOKENS.iter().find(|x| return x.0 == s) {
            return Ok(x.1.clone());
        }
        
        return Err("cannot parse the given raw value");
    }
}

// implement From<T> trait
impl<'a> From<Comment<'a>> for Token<'a> {
    fn from(t: Comment<'a>) -> Self {
        return Token::Comment(t);
    }
}

impl From<Keyword> for Token<'_> {
    fn from(t: Keyword) -> Self {
        return Token::Keyword(t);
    }
}

impl From<Type> for Token<'_> {
    fn from(t: Type) -> Self {
        return Token::Type(t);
    }
}

impl<'a> From<Identifier<'a>> for Token<'a> {
    fn from(t: Identifier<'a>) -> Self {
        return Token::Identifier(t);
    }
}

impl<'a> From<Literal<'a>> for Token<'a> {
    fn from(t: Literal<'a>) -> Self {
        return Token::Literal(t);
    }
}

impl From<Symbol> for Token<'_> {
    fn from(t: Symbol) -> Self {
        return Token::Symbol(t);
    }
}

// implement processable trait
impl Grammar for Token<'_> {
    fn process(&mut self, token: &Token) -> grammar::Result {
        if self.is_match(token) {
            return grammar::Result::Consumed(VecDeque::new());
        }
        else {
            return grammar::Result::Unexpected(format!("mismatched token: '{:?}' compared with '{:?}'", self, token));
        }
    }

    fn is_done(&self) -> bool {
        return true;
    }

    fn info(&self) -> String {
        return format!("Token");
    }
}