use std::collections::VecDeque;

use crate::token;

pub trait Grammar {
    fn process(&mut self, token: &token::Token) -> Result;
    fn is_done(&self) -> bool;
    fn info(&self) -> String;
}

pub enum Result {
    Consumed(VecDeque<Box<dyn Grammar>>),
    Passed,
    Unexpected(String)
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

                return Result::Unexpected(format!("Err!"));
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
pub struct Program {
    pattern: GrammarPattern<'static>
}

impl Program {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ModuleDeclaration::new())
                ])
            ])
        };
    }
}

// module declararion
pub struct ModuleDeclaration {
    pattern: GrammarPattern<'static>
}

impl ModuleDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Module))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftBrace))
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(FunctionDeclaration::new()),
                    || return Box::new(TypeDeclaration::new()),
                    || return Box::new(TableDeclaration::new()),
                    || return Box::new(MemoryDeclaration::new()),
                    || return Box::new(GlobalDeclaration::new()),
                    || return Box::new(ImportDeclaration::new()),
                    || return Box::new(ExportDeclaration::new()),
                    || return Box::new(AssignmentStatement::new()),
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightBrace))
                ])
            ])
        };
    }
}

// type definition
pub struct TypeDefinition {
    pattern: GrammarPattern<'static>
}

impl TypeDefinition {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(UnitType::new()),
                    || return Box::new(ParentheseTypeVariant::new())
                ])
            ])
        };
    }
}

pub struct UnitType {
    pattern: GrammarPattern<'static>
}

impl UnitType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Type(token::Type::RegexAny))
                ])
            ])
        };
    }
}

pub struct VecShorthandType {
    pattern: GrammarPattern<'static>
}

impl VecShorthandType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_numeric_regex(".")))
                ])
            ])
        };
    }
}

pub struct ParentheseTypeVariant {
    pattern: GrammarPattern<'static>
}

impl ParentheseTypeVariant {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ConRangeType::new()),
                    || return Box::new(ConTupleType::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

pub struct ConRangeType {
    pattern: GrammarPattern<'static>
}

impl ConRangeType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_numeric_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Type(token::Type::RegexAny))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_numeric_regex(".")))
                ])
            ])
        };
    }
}

pub struct ConTupleType {
    pattern: GrammarPattern<'static>
}

impl ConTupleType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Type(token::Type::RegexAny))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(TupleTypeRecursiveSequence::new()),
                    || return Box::new(VecShorthandType::new())
                ])
            ])
        };
    }
}

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

pub struct TupleTypeSequence {
    pattern: GrammarPattern<'static>
}

impl TupleTypeSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Type(token::Type::RegexAny))
                ])
            ])
        };
    }
}

// global declaration
pub struct GlobalDeclaration {
    pattern: GrammarPattern<'static>
}

impl GlobalDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Global))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new()),
                    || return Box::new(MultiIdDeclaration::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftArrow))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

pub struct ImportedGlobalDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedGlobalDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Global))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Mutable))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ])
            ])
        };
    }
}

// type declaration
pub struct TypeDeclaration {
    pattern: GrammarPattern<'static>
}

impl TypeDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Type))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionExpression::new()),
                    || return Box::new(TypeOfExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// table declaration
pub struct TableDeclaration {
    pattern: GrammarPattern<'static>
}

impl TableDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Table))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

pub struct ImportedTableDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedTableDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Table))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ])
            ])
        };
    }
}

// memory declaration
pub struct MemoryDeclaration {
    pattern: GrammarPattern<'static>
}

impl MemoryDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Memory))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

pub struct ImportedMemoryDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedMemoryDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Memory))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Assignment))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ])
            ])
        };
    }
}

// import declaration
pub struct ImportDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Import))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ImportedFunctionDeclaration::new()),
                    || return Box::new(ImportedTableDeclaration::new()),
                    || return Box::new(ImportedMemoryDeclaration::new()),
                    || return Box::new(ImportedGlobalDeclaration::new()),
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::From))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_string_regex("."))),
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// export declaration
pub struct ExportDeclaration {
    pattern: GrammarPattern<'static>
}

impl ExportDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Export))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionDeclaration::new()),
                    || return Box::new(TableDeclaration::new()),
                    || return Box::new(MemoryDeclaration::new()),
                    || return Box::new(GlobalDeclaration::new()),
                    || return Box::new(AliasedExportDeclaration::new())
                ])
            ])
        };
    }
}

pub struct AliasedExportDeclaration {
    pattern: GrammarPattern<'static>
}

impl AliasedExportDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::As))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_string_regex("."))),
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// function declaration and its components
pub struct FunctionDeclaration {
    pattern: GrammarPattern<'static>
}

impl FunctionDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Function))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
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

pub struct ImportedFunctionDeclaration {
    pattern: GrammarPattern<'static>
}

impl ImportedFunctionDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Function))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Signature::new())
                ])
            ])
        };
    }
}

// -> signature
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
pub struct Parameter {
    pattern: GrammarPattern<'static>
}

impl Parameter {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ParamSequence::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

// -> parameter sequence
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
pub struct ParamType {
    pattern: GrammarPattern<'static>
}

impl ParamType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Colon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ])
            ])
        };
    }
}

pub struct ConParamType {
    pattern: GrammarPattern<'static>
}

impl ConParamType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(ParamType::new())
                ])
            ])
        };
    }
}

// -> return type
pub struct ResultType {
    pattern: GrammarPattern<'static>
}

impl ResultType {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightArrow))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(TypeDefinition::new())
                ])
            ])
        };
    }
}

// function block
pub struct FunctionBlock {
    pattern: GrammarPattern<'static>
}

impl FunctionBlock {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftBrace))
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(LocalDeclaration::new()),
                    || return Box::new(AssignmentStatement::new()),
                    || return Box::new(IfStatement::new()),
                    || return Box::new(WhileStatement::new()),
                    || return Box::new(ReturnStatement::new()),
                    || return Box::new(BreakStatement::new()),
                    || return Box::new(ContinueStatement::new()),
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightBrace))
                ])
            ])
        };
    }
}

// -> local
pub struct LocalDeclaration {
    pattern: GrammarPattern<'static>
}

impl LocalDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Local))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new()),
                    || return Box::new(MultiIdDeclaration::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftArrow))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

pub struct MutableIdDeclaration {
    pattern: GrammarPattern<'static>
}

impl MutableIdDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Mutable))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ])
            ])
        };
    }
}

pub struct MultiIdDeclaration {
    pattern: GrammarPattern<'static>
}

impl MultiIdDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConMultiIdDeclaration::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

pub struct ConMultiIdDeclaration {
    pattern: GrammarPattern<'static>
}

impl ConMultiIdDeclaration {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(MutableIdDeclaration::new())
                ])
            ])
        };
    }
}

// -> if
pub struct IfStatement {
    pattern: GrammarPattern<'static>
}

impl IfStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::If))
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

pub struct ElseIfStatement {
    pattern: GrammarPattern<'static>
}

impl ElseIfStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::ElseIf))
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

pub struct ElseStatement {
    pattern: GrammarPattern<'static>
}

impl ElseStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Else))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(FunctionBlock::new())
                ])
            ])
        };
    }
}

// -> while
pub struct WhileStatement {
    pattern: GrammarPattern<'static>
}

impl WhileStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::While))
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

pub struct BreakStatement {
    pattern: GrammarPattern<'static>
}

impl BreakStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Break))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

pub struct ContinueStatement {
    pattern: GrammarPattern<'static>
}

impl ContinueStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Cont))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// -> return
pub struct ReturnStatement {
    pattern: GrammarPattern<'static>
}

impl ReturnStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Return))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

// -> assignment
pub struct AssignmentStatement {
    pattern: GrammarPattern<'static>
}

impl AssignmentStatement {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex("."))),
                    || return Box::new(OffsetExpression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftArrow))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::SemiColon))
                ])
            ])
        };
    }
}

pub struct Expression {
    pattern: GrammarPattern<'static>
}

impl Expression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_numeric_regex("."))),
                    || return Box::new(WithIdExpression::new()),
                    || return Box::new(FunctionExpression::new()),
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
pub struct WithIdExpression {
    pattern: GrammarPattern<'static>
}

impl WithIdExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConMemberExpression::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(GroupedOrTupleExpression::new()),
                    || return Box::new(ConCallIndirectExpression::new())
                ])
            ])
        };
    }
}

pub struct ConExprSequence {
    pattern: GrammarPattern<'static>
}

impl ConExprSequence {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> call indirect
pub struct ConCallIndirectExpression {
    pattern: GrammarPattern<'static>
}

impl ConCallIndirectExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::DoubleColon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex("."))),
                    || return Box::new(TypeOfExpression::new())
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(CallIndirectArg::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

// -> call indirect argument
pub struct CallIndirectArg {
    pattern: GrammarPattern<'static>
}

impl CallIndirectArg {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Comma))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(GroupedOrTupleExpression::new())
                ])
            ])
        };
    }
}

// -> unary
pub struct UnaryExpression {
    pattern: GrammarPattern<'static>
}

impl UnaryExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Plus)),
                    || return Box::new(token::Token::Symbol(token::Symbol::Minus)),
                    || return Box::new(token::Token::Symbol(token::Symbol::BitwiseNot)),
                    || return Box::new(token::Token::Symbol(token::Symbol::LogicalNegation))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> binary
pub struct ConBinaryExpression {
    pattern: GrammarPattern<'static>
}

impl ConBinaryExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Plus)),
                    || return Box::new(token::Token::Symbol(token::Symbol::Minus)),
                    || return Box::new(token::Token::Symbol(token::Symbol::Asterisk)),
                    || return Box::new(token::Token::Symbol(token::Symbol::Solidus)),
                    || return Box::new(token::Token::Symbol(token::Symbol::Modulo)),
                    || return Box::new(token::Token::Symbol(token::Symbol::Equal)),
                    || return Box::new(token::Token::Symbol(token::Symbol::NotEqual)),
                    || return Box::new(token::Token::Symbol(token::Symbol::LessThan)),
                    || return Box::new(token::Token::Symbol(token::Symbol::GreaterThan)),
                    || return Box::new(token::Token::Symbol(token::Symbol::LessThanOrEqual)),
                    || return Box::new(token::Token::Symbol(token::Symbol::GreaterThanOrEqual)),
                    || return Box::new(token::Token::Symbol(token::Symbol::BitwiseAnd)),
                    || return Box::new(token::Token::Symbol(token::Symbol::BitwiseOr)),
                    || return Box::new(token::Token::Symbol(token::Symbol::BitwiseXor)),
                    || return Box::new(token::Token::Symbol(token::Symbol::BitwiseNot)),
                    || return Box::new(token::Token::Symbol(token::Symbol::ShiftLeftLogical)),
                    || return Box::new(token::Token::Symbol(token::Symbol::ShiftRightArithmatic)),
                    || return Box::new(token::Token::Symbol(token::Symbol::ShiftRightLogical)),
                    || return Box::new(token::Token::Symbol(token::Symbol::RotateLeft)),
                    || return Box::new(token::Token::Symbol(token::Symbol::RotateRight)),
                    || return Box::new(token::Token::Symbol(token::Symbol::LogicalAnd)),
                    || return Box::new(token::Token::Symbol(token::Symbol::LogicalOr)),
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> conditional (ternary)
pub struct ConConditionalExpression {
    pattern: GrammarPattern<'static>
}

impl ConConditionalExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Query))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Colon))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ])
            ])
        };
    }
}

// -> member
pub struct ConMemberExpression {
    pattern: GrammarPattern<'static>
}

impl ConMemberExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Dot))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ])
            ])
        };
    }
}

// -> grouped
pub struct GroupedOrTupleExpression {
    pattern: GrammarPattern<'static>
}

impl GroupedOrTupleExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Expression::new())
                ]),
                GrammarQuantifier::OptionalMany(&[
                    || return Box::new(ConExprSequence::new())
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightParenthese))
                ])
            ])
        };
    }
}

// -> function
pub struct FunctionExpression {
    pattern: GrammarPattern<'static>
}

impl FunctionExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::Function))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(Signature::new())
                ])
            ])
        };
    }
}

// -> typeof
pub struct TypeOfExpression {
    pattern: GrammarPattern<'static>
}

impl TypeOfExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Keyword(token::Keyword::TypeOf))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex("."))),
                ])
            ])
        };
    }
}

// -> offset
pub struct OffsetExpression {
    pattern: GrammarPattern<'static>
}

impl OffsetExpression {
    pub fn new() -> Self {
        return Self {
            pattern: GrammarPattern::new(&[
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::Asterisk))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Literal(token::Literal::from_numeric_regex("."))),
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::LeftParenthese))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Identifier(token::Identifier::from_regex(".")))
                ]),
                GrammarQuantifier::One(&[
                    || return Box::new(token::Token::Symbol(token::Symbol::RightParenthese))
                ]),
                GrammarQuantifier::OptionalOne(&[
                    || return Box::new(ConCallIndirectExpression::new())
                ])
            ])
        };
    }
}

// implement Grammar
impl Grammar for Program {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("Program, s:{}", self.pattern.state); }
}

impl Grammar for TypeDefinition {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("TypeDefinition, s:{}", self.pattern.state); }
}

impl Grammar for UnitType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("UnitType, s:{}", self.pattern.state); }
}

impl Grammar for VecShorthandType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("VecShorthandType, s:{}", self.pattern.state); }
}

impl Grammar for ParentheseTypeVariant {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ParentheseTypeVariant, s:{}", self.pattern.state); }
}

impl Grammar for ConRangeType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConRangeType, s:{}", self.pattern.state); }
}

impl Grammar for ConTupleType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConTupleType, s:{}", self.pattern.state); }
}

impl Grammar for TupleTypeRecursiveSequence {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("TupleTypeRecursiveSequence, s:{}", self.pattern.state); }
}

impl Grammar for TupleTypeSequence {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("TupleTypeSequence, s:{}", self.pattern.state); }
}

impl Grammar for ModuleDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ModuleDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for FunctionDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("FunctionDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ImportedFunctionDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ImportedFunctionDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for GlobalDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("GlobalDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ImportedGlobalDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ImportedGlobalDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for MemoryDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("MemoryDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ImportedMemoryDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ImportedMemoryDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for TypeDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("TypeDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for TableDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("TableDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ImportedTableDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ImportedTableDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ImportDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ImportDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ExportDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ExportDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for AliasedExportDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("AliasedExportDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for Signature {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("Signature, s:{}", self.pattern.state); }
}

impl Grammar for Parameter {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("Parameter, s:{}", self.pattern.state); }
}

impl Grammar for ParamSequence {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ParamSequence, s:{}", self.pattern.state); }
}

impl Grammar for ParamType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ParamType, s:{}", self.pattern.state); }
}

impl Grammar for ConParamType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConParamType, s:{}", self.pattern.state); }
}

impl Grammar for ResultType {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ResultType, s:{}", self.pattern.state); }
}

impl Grammar for FunctionBlock {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("FunctionBlock, s:{}", self.pattern.state); }
}

impl Grammar for LocalDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("LocalDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for MutableIdDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("MutableIdDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for MultiIdDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("MultiIdDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for ConMultiIdDeclaration {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConMultiIdDeclaration, s:{}", self.pattern.state); }
}

impl Grammar for IfStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("IfStatement, s:{}", self.pattern.state); }
}

impl Grammar for ElseIfStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ElseIfStatement, s:{}", self.pattern.state); }
}

impl Grammar for ElseStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ElseStatement, s:{}", self.pattern.state); }
}

impl Grammar for WhileStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("WhileStatement, s:{}", self.pattern.state); }
}

impl Grammar for BreakStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("BreakStatement, s:{}", self.pattern.state); }
}

impl Grammar for ContinueStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ContinueStatement, s:{}", self.pattern.state); }
}

impl Grammar for ReturnStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ReturnStatement, s:{}", self.pattern.state); }
}

impl Grammar for AssignmentStatement {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("AssignmentStatement, s:{}", self.pattern.state); }
}

impl Grammar for Expression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("Expression, s:{}", self.pattern.state); }
}

impl Grammar for WithIdExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("WithIdExpression, s:{}", self.pattern.state); }
}

impl Grammar for ConExprSequence {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConExprSequence, s:{}", self.pattern.state); }
}

impl Grammar for ConCallIndirectExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConCallIndirectExpression, s:{}", self.pattern.state); }
}

impl Grammar for CallIndirectArg {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("CallIndirectArg, s:{}", self.pattern.state); }
}

impl Grammar for UnaryExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("UnaryExpression, s:{}", self.pattern.state); }
}

impl Grammar for ConBinaryExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConBinaryExpression, s:{}", self.pattern.state); }
}

impl Grammar for ConConditionalExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConConditionalExpression, s:{}", self.pattern.state); }
}

impl Grammar for FunctionExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("FunctionExpression, s:{}", self.pattern.state); }
}

impl Grammar for ConMemberExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("ConMemberExpression, s:{}", self.pattern.state); }
}

impl Grammar for GroupedOrTupleExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("GroupedOrTupleExpression, s:{}", self.pattern.state); }
}

impl Grammar for TypeOfExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("TypeOfExpression, s:{}", self.pattern.state); }
}

impl Grammar for OffsetExpression {
    fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
    fn is_done(&self) -> bool { return self.pattern.is_done; }
    fn info(&self) -> String { return format!("OffsetExpression, s:{}", self.pattern.state); }
}