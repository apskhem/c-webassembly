use std::convert::TryFrom;
use std::fmt;

use crate::token_stream;
use crate::token;

// enum section
enum TokenSequence {
    None,
    Word,
    Symbol,
    SignleLineComment,
    MultiLineComment,
    StringLiteral,
    NumericLiteral
}

// struct section
struct CharPositionCounter {
    ln: usize,
    col: usize
}

impl CharPositionCounter {
    const fn new() -> Self {
        return Self {
            ln: 0,
            col: 0
        };
    }

    fn next_char(&mut self) {
        self.col += 0;
    }

    fn next_line(&mut self) {
        self.ln += 1;
        self.col = 0;
    }

    const fn ln(&self) -> usize {
        return self.ln;
    }

    const fn col(&self) -> usize {
        return self.col;
    }
}

impl fmt::Debug for CharPositionCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}:{}", self.ln, self.col);
    }
}

// main program section
pub fn tokenize(text: &str) -> Result<Vec<token::Token>, String> {
    let mut token_collector = token_stream::RawTokenStream::new(text);
    let mut char_pos_counter = CharPositionCounter::new();
    let mut mode = TokenSequence::None;
    let mut offset = 0;

    let mut iter = text.chars().enumerate();
    while let Some((i, c)) = iter.next() {
        char_pos_counter.next_char();
        
        let z = c.len_utf8();

        // (con.) check for identifier
        match &mode {
            TokenSequence::Word => {
                if token::Identifier::is_alphanumeric_valid_char(c) {
                    token_collector.add(z);

                    offset += z;
                    continue;
                }
                else {
                    token_collector.cut();
                    mode = TokenSequence::None;
                }
            },
            TokenSequence::Symbol => {
                let prejoined = token_collector.temp_prejoined(z);
                
                // single line comment
                if prejoined == "//" {
                    token_collector.add(z);
                    mode = TokenSequence::SignleLineComment;

                    offset += z;
                    continue;
                }
                // multi line comment
                else if prejoined == "/*" {
                    token_collector.add(z);
                    mode = TokenSequence::MultiLineComment;

                    offset += z;
                    continue;
                }
                else if token::Symbol::match_str(&prejoined) {
                    token_collector.add(z);

                    offset += z;
                    continue;
                }
                else {
                    token_collector.cut();
                    mode = TokenSequence::None;
                }
            },
            TokenSequence::SignleLineComment => {
                if c == '\n' {
                    token_collector.cut();
                    mode = TokenSequence::None;
                }
                else {
                    token_collector.add(z);
                }
                
                offset += z;
                continue;
            },
            TokenSequence::MultiLineComment => {
                if c == '/' && token_collector.temp().ends_with('*') {
                    token_collector.add(z).cut();
                    mode = TokenSequence::None;
                }
                else {
                    token_collector.add(z);
                }
                
                offset += z;
                continue;
            },
            TokenSequence::StringLiteral => {
                if c == '\"' && !token_collector.temp().ends_with('\\') {
                    token_collector.add(z).cut();
                    mode = TokenSequence::None;
                }
                else {
                    token_collector.add(z);
                }

                offset += z;
                continue;
            },
            TokenSequence::NumericLiteral => {
                //  TODO: maybe add stricter check
                //  TODO: add e+, e-
                if c.is_ascii_alphanumeric() || c == '.' {
                    token_collector.add(z);

                    offset += z;
                    continue;
                }
                else {
                    token_collector.cut();
                    mode = TokenSequence::None;
                }
            },
            _ => {}
        };

        // skip whitespaces and escape keys
        if c.is_whitespace() || c == '\n' || c == '\t' || c == '\r' || c == '\0' {
            token_collector.cut();

            if c == '\n' {
                char_pos_counter.next_line();
            }

            offset += z;
            continue;
        }
        // string literal
        else if c == '\"' {
            mode = TokenSequence::StringLiteral;
        }
        // symbols
        else if token::Symbol::match_char(c) {
            mode = TokenSequence::Symbol;
        }
        // number literal
        else if c.is_ascii_digit() {
            mode = TokenSequence::NumericLiteral;
        }
        // identifier
        else if token::Identifier::is_alphabetic_valid_char(c) {
            mode = TokenSequence::Word;
        }
        // others will be error
        else {
            return Err(format!("unknown start of token: `{}` at {:?}", c, char_pos_counter));
        }
        
        token_collector.set_start(i, offset, z);
        offset += z;
    }

    // termination validation
    if !token_collector.temp().is_empty() {
        return match mode {
            TokenSequence::StringLiteral => Err(format!("unexpected unclosed string")),
            _ => Err(format!("unexpected tokenization error"))
        }
    }

    // validate tokens
    let mut res = Vec::with_capacity(token_collector.count());
    
    let mut iter = token_collector.collect().into_iter();
    while let Some(raw_token) = iter.next() {
        let token = token::Token::try_from(raw_token)?;

        res.push(token);
    }

    return Ok(res);
}