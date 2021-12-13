use crate::token;

pub const TYPE_TOKENS: &[(&str, token::Type)] = &[
    ("i32",     token::Type::I32),
    ("i64",     token::Type::I64),
    ("f32",     token::Type::F32),
    ("f64",     token::Type::F64),
    ("fref",    token::Type::Fref),
    ("xref",    token::Type::Xref),
    ("page",    token::Type::Page)
];

// TODO: >>> (shift right logical) is currently not support
pub const SYMBOL_TOKENS: &[(&str, token::Symbol)] = &[
    (".",       token::Symbol::Dot),
    (",",       token::Symbol::Comma),
    (":",       token::Symbol::Colon),
    (";",       token::Symbol::SemiColon),
    ("+",       token::Symbol::Plus),
    ("-",       token::Symbol::Minus),
    ("*",       token::Symbol::Asterisk),
    ("/",       token::Symbol::Solidus),
    ("%",       token::Symbol::Modulo),
    ("=",       token::Symbol::Assignment),
    ("==",      token::Symbol::Equal),
    ("!=",      token::Symbol::NotEqual),
    ("<",       token::Symbol::LessThan),
    (">",       token::Symbol::GreaterThan),
    ("<=",      token::Symbol::LessThanOrEqual),
    (">=",      token::Symbol::GreaterThanOrEqual),
    ("<-",      token::Symbol::LeftArrow),
    ("->",      token::Symbol::RightArrow),
    ("&",       token::Symbol::BitwiseAnd),
    ("|",       token::Symbol::BitwiseOr),
    ("^",       token::Symbol::BitwiseXor),
    ("~",       token::Symbol::BitwiseNot),
    ("<<",      token::Symbol::ShiftLeftLogical),
    (">>",      token::Symbol::ShiftRightLogical),
    (">>>",     token::Symbol::ShiftRightArithmatic),
    ("!",       token::Symbol::LogicalNegation),
    ("&&",      token::Symbol::LogicalAnd),
    ("||",      token::Symbol::LogicalOr),
    ("|>",      token::Symbol::PipeForward),
    ("?",       token::Symbol::Query),
    ("::",      token::Symbol::DoubleColon),
    ("{",       token::Symbol::LeftBrace),
    ("}",       token::Symbol::RightBrace),
    ("(",       token::Symbol::LeftParenthese),
    (")",       token::Symbol::RightParenthese)
];

pub const KEYWORD_TOKENS: &[(&str, token::Keyword)] = &[
    ("fn",      token::Keyword::Function),
    ("mut",     token::Keyword::Mutable),
    ("let",     token::Keyword::Let),
    ("mem",     token::Keyword::Memory),
    ("tab",     token::Keyword::Table),
    ("type",    token::Keyword::Type),
    ("ret",     token::Keyword::Return),
    ("if",      token::Keyword::If),
    ("else",    token::Keyword::Else),
    ("elif",    token::Keyword::ElseIf),
    ("while",   token::Keyword::While),
    ("brk",     token::Keyword::Break),
    ("cont",    token::Keyword::Cont),
    ("typeof",  token::Keyword::TypeOf),
    ("exp",     token::Keyword::Export),
    ("imp",     token::Keyword::Import),
    ("as",      token::Keyword::As),
    ("from",    token::Keyword::From),
    ("incl",    token::Keyword::Include)
];