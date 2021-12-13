use std::ops::Range;

use crate::token;

pub struct RawTokenStream<'a> {
    ctx: &'a str,
    tokens: Vec<token::RawToken<'a>>,
    range: Range<usize>
}

impl<'a> RawTokenStream<'a> {
    pub const fn new(ctx: &'a str) -> Self {
        return Self {
            ctx,
            tokens: vec![],
            range: usize::MAX..0
        };
    }

    pub fn collect(self) -> Vec<token::RawToken<'a>> {
        return self.tokens;
    }

    pub fn temp(&self) -> &'a str {
        if self.range.is_empty() {
            return "";
        }
        
        return &self.ctx[self.range.clone()];
    }

    pub fn temp_prejoined(&self, dif: usize) -> &'a str {
        if self.range.start != usize::MAX {
            let pre_len = self.range.end + dif;
            return &self.ctx[self.range.start..pre_len];
        }

        return self.temp();
    }

    pub fn set_start(&mut self, offset: usize, dif: usize) {
        self.range.start = offset;
        self.range.end = offset + dif;
    }

    pub fn add(&mut self, dif: usize) -> &mut Self {
        self.range.end += dif;

        return self;
    }

    pub fn cut(&mut self) {
        if !self.range.is_empty() {
            let x_str = &self.ctx[self.range.clone()];
            let new_token = token::RawToken::new(x_str, self.range.clone());

            self.tokens.push(new_token);
        }
        
        self.reset_range();
    }

    fn reset_range(&mut self) {
        self.range = usize::MAX..0;
    }
}