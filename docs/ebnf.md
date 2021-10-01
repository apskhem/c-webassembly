Program
    : ModuleDeclaration*
    ;

ModuleDeclaration
    : 'mod' '(' ModuleBlock ')'
    ;

ModuleBlock
    : MemoryDeclaration*
    | TableDeclaration*
    | GlobalDeclaration*
    | FunctionDeclaration*
    | ImportDeclaration*
    | ExportDeclaration*
    ;

MemoryDeclaration
    : 'mem' Identifier '=' Vec_Memory_Type ';'
    ;

TableDeclaration
    : 'tab' Identifier '=' Vec_Reference_Type ';'
    ;

GlobalDeclaration
    : 'gb' Identifier '<-' Value ';'
    ;

FunctionDeclaration
    : 'fn' Identifier Signature FunctionBlock
    ;

Signature
    : Parameter Result?
    ;

Parameter
    : '(' (ParamType (SYMBOL_COMMA ParamType)*)? ')'
    ;

ParamType
    : Identifier ':' Unit_Value_Type
    ;

Result
    : '->' All_Value_Type
    ;


// Literal

Literal
    : StringLiteral
    | NumericLiteral
    ;

StringLiteral
    : '"' .* '"'
    ;

Numeric
    : BinaryLiteral
    | OctalLiteral
    | DecimalLiteral
    | HexLiteral
    ;

BinaryLiteral
    : 0b[0-1]+(.[0-1]+)?
    ;

OctalLiteral
    : 0o?[0-7]+(.[0-7]+)?
    ;

DecimalLiteral
    : [0-9]+(.[0-9]+)?((e|E)(\+|\-)[0-9]+)?
    ;

HexLiteral
    : 0x[0-9a-fA-F]+(.[0-9a-fA-F]+)?
    ;