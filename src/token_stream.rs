use crate::token;

pub struct RawTokenStream<'a> {
    ctx: &'a str,
    tokens: Vec<token::RawToken<'a>>,
    str_start: Option<usize>,
    str_end: usize,
    begin: Option<usize>,
    count: usize
}

impl<'a> RawTokenStream<'a> {
    pub const fn new(ctx: &'a str) -> Self {
        return Self {
            ctx,
            tokens: vec![],
            str_start: None,
            str_end: 0,
            begin: None,
            count: 0
        };
    }

    pub fn count(&self) -> usize {
        return self.tokens.len();
    }

    pub fn collect(self) -> Vec<token::RawToken<'a>> {
        return self.tokens;
    }

    pub fn temp(&self) -> &'a str {
        if let Some(start) = self.str_start {
            return &self.ctx[start..self.str_end];
        }
        
        return "";
    }

    pub fn temp_prejoined(&self, dif: usize) -> &'a str {
        if let Some(start) = self.str_start {
            let pre_len = self.str_end + dif;
            return &self.ctx[start..pre_len];
        }

        return self.temp();
    }

    pub fn set_start(&mut self, idx: usize, offset: usize, dif: usize) {
        self.begin = Some(idx);
        self.str_start = Some(offset);
        self.str_end = offset + dif;
    }

    pub fn add(&mut self, dif: usize) -> &mut Self {
        self.str_end += dif;
        self.count += 1;

        return self;
    }

    pub fn cut(&mut self) {
        match (self.begin, self.str_start) {
            (Some(begin), Some(start)) if self.str_end > start => {
                let end = begin + self.count;
                let x_str = &self.ctx[start..self.str_end];
                let new_token = token::RawToken::new(x_str, (begin, end));

                self.tokens.push(new_token);
            },
            _ => {}
        };
        
       self.reset();
    }

    fn reset(&mut self) {
        self.str_start = None;
        self.str_end = 0;
        self.begin = None;
        self.count = 0;
    }
}