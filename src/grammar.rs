use std::collections::VecDeque;
use std::error::Error;

use crate::token;
use crate::token_grammar;

pub trait Grammar {
    fn process(&mut self, token: &token::Token) -> Result;
    fn is_done(&self) -> bool;
    fn info(&self) -> String;
}

pub enum Result {
    Consumed(VecDeque<Box<dyn Grammar>>),
    Passed,
    Unexpected(Box<dyn Error>)
}

pub enum GrammarQuantifier<'a> {
    One(&'a [fn() -> Box<dyn Grammar>]),
    OptionalOne(&'a [fn() -> Box<dyn Grammar>]),
    OptionalMany(&'a [fn() -> Box<dyn Grammar>])
}

pub enum GrammarError {
    TypeExpected,
    SymbolExpected,
    IdentifierExpected,
    KeywordExpected,
    ExpressionExpected,
}

pub struct GrammarPattern<'a> {
    pattern: &'a [GrammarQuantifier<'a>],
    is_done: bool,
    state: u8
}

impl<'a> GrammarPattern<'a> {
    pub const fn new(pattern: &'a [GrammarQuantifier]) -> Self {
        return Self {
            pattern,
            is_done: false,
            state: 0
        };
    }

    pub fn execute(&mut self, token: &token::Token) -> Result {
        if self.is_done {
            return Result::Passed;
        }

        match self.current() {
            GrammarQuantifier::One(prototypes) => {
                for proto in prototypes.iter() {
                    let mut dupl = proto();

                    if let Result::Consumed(mut list) = dupl.process(token) {
                        if !dupl.is_done() {
                            list.push_front(dupl);
                        }
                        
                        self.next();
                        
                        return Result::Consumed(list);
                    }
                }

                return Result::Unexpected("Err!".into());
            },
            GrammarQuantifier::OptionalOne(prototypes) => {
                for proto in prototypes.iter() {
                    let mut dupl = proto();

                    if let Result::Consumed(mut list) = dupl.process(token) {
                        if !dupl.is_done() {
                            list.push_front(dupl);
                        }
                            
                        self.next();
                            
                        return Result::Consumed(list);
                    }
                }

                return self.execute_next(token);
            },
            GrammarQuantifier::OptionalMany(prototypes) => {
                for proto in prototypes.iter() {
                    let mut dupl = proto();

                    if let Result::Consumed(mut list) = dupl.process(token) {
                        if !dupl.is_done() {
                            list.push_front(dupl);
                        }
                            
                        return Result::Consumed(list);
                    }
                }

                return self.execute_next(token);
            }
        };
    }

    fn execute_next(&mut self, token: &token::Token) -> Result {
        self.next();
        return self.execute(token);
    }

    fn next(&mut self) {
        self.state += 1;

        if usize::from(self.state) >= self.pattern.len() {
            self.is_done = true;
        }
    }

    pub fn current(&self) -> &GrammarQuantifier {
        return self.pattern.get(usize::from(self.state)).expect("Something went wrong");
    }
}

// construction rules
// 1. the first step cannot be self, it will cause infinite recusive calls.
// 2. first grammar of each return argument must not collide with sibling members.

// start of definition
#[derive(c_webassembly::Grammar)]
pub struct Program {
    pattern: GrammarPattern<'static>
}

impl Program {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(FunctionDeclaration::new()),
                    || return Box::new(TypeDeclaration::new()),
                    || return Box::new(TableDeclaration::new()),
                    || return Box::new(MemoryDeclaration::new()),
                    || return Box::new(VariableDeclaration::new()),
                    || return Box::new(ImportDeclaration::new()),
                    || return Box::new(ExportDeclaration::new()),
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// con type definition
#[derive(c_webassembly::Grammar)]
pub struct ConTypeAssignment {
    pattern: GrammarPattern<'static>
}

impl ConTypeAssignment {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeExpression::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct VecShorthandType {
    pattern: GrammarPattern<'static>
}

impl VecShorthandType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_numeric_literal())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ParentheseTypeVariant {
    pattern: GrammarPattern<'static>
}

impl ParentheseTypeVariant {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConRangeType::new()),
                    || return Box::new(ConTupleType::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ConRangeType {
    pattern: GrammarPattern<'static>
}

impl ConRangeType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_numeric_literal())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_type())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_numeric_literal())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ConTupleType {
    pattern: GrammarPattern<'static>
}

impl ConTupleType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_type())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(TupleTypeRecursiveSequence::new()),
                    || return Box::new(VecShorthandType::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct TupleTypeRecursiveSequence {
    pattern: GrammarPattern<'static>
}

impl TupleTypeRecursiveSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(TupleTypeSequence::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct TupleTypeSequence {
    pattern: GrammarPattern<'static>
}

impl TupleTypeSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_type())
                ])
            ])
        };
    }
}

// global declaration
#[derive(c_webassembly::Grammar)]
pub struct GlobalDeclaration {
    pattern: GrammarPattern<'static>
}

#[derive(c_webassembly::Grammar)]
pub struct ImportedVariableDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedVariableDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Let))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Mutable))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConTypeAssignment::new())
                ])
            ])
        };
    }
}

// type declaration
#[derive(c_webassembly::Grammar)]
pub struct TypeDeclaration {
    pattern: GrammarPattern<'static>
}

impl TypeDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Type))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConTypeAssignment::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// table declaration
#[derive(c_webassembly::Grammar)]
pub struct TableDeclaration {
    pattern: GrammarPattern<'static>
}

impl TableDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Table))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConTypeAssignment::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ImportedTableDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedTableDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Table))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConTypeAssignment::new())
                ])
            ])
        };
    }
}

// memory declaration
#[derive(c_webassembly::Grammar)]
pub struct MemoryDeclaration {
    pattern: GrammarPattern<'static>
}

impl MemoryDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Memory))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConTypeAssignment::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ImportedMemoryDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedMemoryDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Memory))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConTypeAssignment::new())
                ])
            ])
        };
    }
}

// import declaration
#[derive(c_webassembly::Grammar)]
pub struct ImportDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Import))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ImportedFunctionDeclaration::new()),
                    || return Box::new(ImportedTableDeclaration::new()),
                    || return Box::new(ImportedMemoryDeclaration::new()),
                    || return Box::new(ImportedVariableDeclaration::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::From))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_string_literal())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// export declaration
#[derive(c_webassembly::Grammar)]
pub struct ExportDeclaration {
    pattern: GrammarPattern<'static>
}

impl ExportDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Export))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(token_grammar::TokenGrammar::any_string_literal())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionDeclaration::new()),
                    || return Box::new(TableDeclaration::new()),
                    || return Box::new(MemoryDeclaration::new()),
                    || return Box::new(VariableDeclaration::new()),
                    || return Box::new(AliasedExportDeclaration::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct AliasedExportDeclaration {
    pattern: GrammarPattern<'static>
}

impl AliasedExportDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::As))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_string_literal()),
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// function declaration and its components
#[derive(c_webassembly::Grammar)]
pub struct FunctionDeclaration {
    pattern: GrammarPattern<'static>
}

impl FunctionDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Function))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Signature::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionBlock::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ImportedFunctionDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedFunctionDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Function))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Signature::new())
                ])
            ])
        };
    }
}

// -> type signature
#[derive(c_webassembly::Grammar)]
pub struct TypeSignature {
    pattern: GrammarPattern<'static>
}

impl TypeSignature {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(TypeParameter::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ResultType::new())
                ])
            ])
        };
    }
}

// -> type parameter
#[derive(c_webassembly::Grammar)]
pub struct TypeParameter {
    pattern: GrammarPattern<'static>
}

impl TypeParameter {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(TypeParamSequence::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

// -> type param sequence
#[derive(c_webassembly::Grammar)]
pub struct TypeParamSequence {
    pattern: GrammarPattern<'static>
}

impl TypeParamSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_type())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConTypeParamSequence::new())
                ])
            ])
        };
    }
}

// -> con: type param sequence
#[derive(c_webassembly::Grammar)]
pub struct ConTypeParamSequence {
    pattern: GrammarPattern<'static>
}

impl ConTypeParamSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_type())
                ])
            ])
        };
    }
}

// -> signature
#[derive(c_webassembly::Grammar)]
pub struct Signature {
    pattern: GrammarPattern<'static>
}

impl Signature {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(Parameter::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ResultType::new())
                ])
            ])
        };
    }
}

// -> parameter
#[derive(c_webassembly::Grammar)]
pub struct Parameter {
    pattern: GrammarPattern<'static>
}

impl Parameter {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ParamSequence::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

// -> parameter sequence
#[derive(c_webassembly::Grammar)]
pub struct ParamSequence {
    pattern: GrammarPattern<'static>
}

impl ParamSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(ParamType::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConParamType::new())
                ])
            ])
        };
    }
}

// -> parameter type
#[derive(c_webassembly::Grammar)]
pub struct ParamType {
    pattern: GrammarPattern<'static>
}

impl ParamType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Colon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeExpression::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ConParamType {
    pattern: GrammarPattern<'static>
}

impl ConParamType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ParamType::new())
                ])
            ])
        };
    }
}

// -> return type
#[derive(c_webassembly::Grammar)]
pub struct ResultType {
    pattern: GrammarPattern<'static>
}

impl ResultType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightArrow))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeExpression::new())
                ])
            ])
        };
    }
}

// function block
#[derive(c_webassembly::Grammar)]
pub struct FunctionBlock {
    pattern: GrammarPattern<'static>
}

impl FunctionBlock {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftBrace))
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(VariableDeclaration::new()),
                    || return Box::new(ExpressionStatement::new()),
                    || return Box::new(IfStatement::new()),
                    || return Box::new(WhileStatement::new()),
                    || return Box::new(ReturnStatement::new()),
                    || return Box::new(BreakStatement::new()),
                    || return Box::new(ContinueStatement::new()),
                    || return Box::new(FunctionBlock::new()),
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightBrace))
                ])
            ])
        };
    }
}

// -> local
#[derive(c_webassembly::Grammar)]
pub struct VariableDeclaration {
    pattern: GrammarPattern<'static>
}

impl VariableDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Let))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new()),
                    || return Box::new(MultiIdDeclaration::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConAssignmentExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct MutableIdDeclaration {
    pattern: GrammarPattern<'static>
}

impl MutableIdDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Mutable))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct MultiIdDeclaration {
    pattern: GrammarPattern<'static>
}

impl MultiIdDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConMultiIdDeclaration::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ConMultiIdDeclaration {
    pattern: GrammarPattern<'static>
}

impl ConMultiIdDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new())
                ])
            ])
        };
    }
}

// -> if
#[derive(c_webassembly::Grammar)]
pub struct IfStatement {
    pattern: GrammarPattern<'static>
}

impl IfStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::If))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(GroupedOrTupleExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionBlock::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ElseIfStatement::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ElseStatement::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ElseIfStatement {
    pattern: GrammarPattern<'static>
}

impl ElseIfStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::ElseIf))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(GroupedOrTupleExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionBlock::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ElseStatement {
    pattern: GrammarPattern<'static>
}

impl ElseStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Else))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionBlock::new())
                ])
            ])
        };
    }
}

// -> while
#[derive(c_webassembly::Grammar)]
pub struct WhileStatement {
    pattern: GrammarPattern<'static>
}

impl WhileStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::While))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(GroupedOrTupleExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionBlock::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct BreakStatement {
    pattern: GrammarPattern<'static>
}

impl BreakStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Break))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ContinueStatement {
    pattern: GrammarPattern<'static>
}

impl ContinueStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Cont))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// -> return
#[derive(c_webassembly::Grammar)]
pub struct ReturnStatement {
    pattern: GrammarPattern<'static>
}

impl ReturnStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Return))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// -> expression statement
#[derive(c_webassembly::Grammar)]
pub struct ExpressionStatement {
    pattern: GrammarPattern<'static>
}

impl ExpressionStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ConAssignmentExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// -> assignment
#[derive(c_webassembly::Grammar)]
pub struct ConAssignmentExpression {
    pattern: GrammarPattern<'static>
}

impl ConAssignmentExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftArrow))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct Expression {
    pattern: GrammarPattern<'static>
}

impl Expression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_numeric_literal()),
                    || return Box::new(WithIdExpression::new()),
                    || return Box::new(TypeOfExpression::new()),
                    || return Box::new(OffsetExpression::new()),
                    || return Box::new(GroupedOrTupleExpression::new()),
                    || return Box::new(UnaryExpression::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConBinaryExpression::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ConConditionalExpression::new())
                ]),
            ])
        };
    }
}

// -> with id expression
#[derive(c_webassembly::Grammar)]
pub struct WithIdExpression {
    pattern: GrammarPattern<'static>
}

impl WithIdExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConMemberExpression::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(FuncCallArg::new()),
                    || return Box::new(ConCallIndirectExpression::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ConExprSequence {
    pattern: GrammarPattern<'static>
}

impl ConExprSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> call indirect
#[derive(c_webassembly::Grammar)]
pub struct ConCallIndirectExpression {
    pattern: GrammarPattern<'static>
}

impl ConCallIndirectExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::DoubleColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(GenericArgument::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FuncCallArg::new())
                ])
            ])
        };
    }
}

// -> call indirect argument
#[derive(c_webassembly::Grammar)]
pub struct FuncCallArg {
    pattern: GrammarPattern<'static>
}

impl FuncCallArg {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(FuncCallArgSequence::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct FuncCallArgSequence {
    pattern: GrammarPattern<'static>
}

impl FuncCallArgSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConFuncCallArgSequence::new())
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct ConFuncCallArgSequence {
    pattern: GrammarPattern<'static>
}

impl ConFuncCallArgSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> unary
#[derive(c_webassembly::Grammar)]
pub struct UnaryExpression {
    pattern: GrammarPattern<'static>
}

impl UnaryExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_unary_symbol())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> binary
#[derive(c_webassembly::Grammar)]
pub struct ConBinaryExpression {
    pattern: GrammarPattern<'static>
}

impl ConBinaryExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_binary_symbol())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> conditional (ternary)
#[derive(c_webassembly::Grammar)]
pub struct ConConditionalExpression {
    pattern: GrammarPattern<'static>
}

impl ConConditionalExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Query))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Colon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> member
#[derive(c_webassembly::Grammar)]
pub struct ConMemberExpression {
    pattern: GrammarPattern<'static>
}

impl ConMemberExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Dot))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ])
            ])
        };
    }
}

// -> grouped
#[derive(c_webassembly::Grammar)]
pub struct GroupedOrTupleExpression {
    pattern: GrammarPattern<'static>
}

impl GroupedOrTupleExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConExprSequence::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

// -> type function
#[derive(c_webassembly::Grammar)]
pub struct TypeFunctionExpression {
    pattern: GrammarPattern<'static>
}

impl TypeFunctionExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::Function))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeSignature::new())
                ])
            ])
        };
    }
}

// -> typeof
#[derive(c_webassembly::Grammar)]
pub struct TypeOfExpression {
    pattern: GrammarPattern<'static>
}

impl TypeOfExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_keyword(token::Keyword::TypeOf))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier()),
                ])
            ])
        };
    }
}

// -> offset
#[derive(c_webassembly::Grammar)]
pub struct OffsetExpression {
    pattern: GrammarPattern<'static>
}

impl OffsetExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::Asterisk))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_numeric_literal()),
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(GenericArgument::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::RightParenthese))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ConCallIndirectExpression::new())
                ])
            ])
        };
    }
}

// -> ganeric
#[derive(c_webassembly::Grammar)]
pub struct GenericArgument {
    pattern: GrammarPattern<'static>
}

impl GenericArgument {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::LessThan))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::from_symbol(token::Symbol::GreaterThan))
                ])
            ])
        };
    }
}

#[derive(c_webassembly::Grammar)]
pub struct TypeExpression {
    pattern: GrammarPattern<'static>
}

impl TypeExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token_grammar::TokenGrammar::any_identifier()),
                    || return Box::new(token_grammar::TokenGrammar::any_type()),
                    || return Box::new(TypeFunctionExpression::new()),
                    || return Box::new(ParentheseTypeVariant::new()),
                    || return Box::new(TypeOfExpression::new()),
                ])
            ])
        };
    }
}
