use std::collections::VecDeque;
use std::error::Error;

use crate::token;
use crate::grammar;
use crate::grammar::Grammar;

pub struct Parser {
    process_stack: VecDeque<Box<dyn Grammar>>,
    counter: usize
}

impl Parser {
    pub fn new() -> Self {
        let mut process_stack = VecDeque::<Box<dyn Grammar>>::new();

        process_stack.push_back(Box::new(grammar::Program::new()));

        return Self {
            process_stack,
            counter: 0,
        };
    }

    pub fn show_status(&self, token: &token::Token) {
        println!("proc: {}, {:?}, stack len: {}", self.counter, token, self.process_stack.len());

        for p in self.process_stack.iter() {
            print!("-> {} ", p.info());
        }

        println!("\n--");
    }

    pub fn process(&mut self, token: &token::Token) -> Result<(), Box<dyn Error>> {
        self.counter += 1;
        self.show_status(token);

        // skip comments
        if let token::Token::Comment(_) = token {
            return Ok(());
        }

        // while the token is not consumed
        loop {
            let top = self.top_process();

            match top.process(token) {
                grammar::Result::Consumed(mut list) => {
                    self.process_stack.append(&mut list);

                    self.update_process_stack();

                    return Ok(());
                },
                grammar::Result::Passed => {
                    self.update_process_stack();

                    continue;
                },
                grammar::Result::Unexpected(err) => {
                    return Err(err);
                },
            }
        }
    }

    fn update_process_stack(&mut self) {
        let mut pop_count = 0;
        for proc in self.process_stack.iter().rev() {
            if proc.is_done() {
                pop_count += 1;
            }
            else {
                break;
            }
        }

        for _ in 0..pop_count {
            let removed = self.process_stack.pop_back();
            
            println!("--#( remove: {:?} )", removed.unwrap().info());
        }
    }

    fn top_process(&mut self) -> &mut Box<dyn Grammar> {
        return self.process_stack.back_mut().expect("unexpected empty process stack");
    }
}

pub fn parse_syntax(tokens: &Vec<token::Token>) -> Result<(), Box<dyn Error>> {
    let mut process_state_machine = Parser::new();

    for token in tokens.iter() {
        process_state_machine.process(token)?;
    }

    return Ok(());
}
