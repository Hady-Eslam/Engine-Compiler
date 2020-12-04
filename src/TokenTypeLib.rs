#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]



// import Modules




// Use Statements Here






#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TOKEN_TYPE{
    EOF,
    
    NEW_LINE,
        
    ASSIGN,
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,

    OPEN_PARENTHESES,
    CLOSE_PARENTHESES,

    OPEN_BRACKET,
    CLOSE_BRACKET,

    COMMA,
    COLON,
    DOT,
    STRING_SEQUENCE,
    CHARACTER,


    // > >= < <= == !=
    GREATER_THAN_OR_EQUAL,
    GREATER_THAN,
    LESS_THAN_OR_EQUAL,
    LESS_THAN,
    EQUAL,
    NOT,
    NOT_EQUAL,
    OR,
    BINARY_OR,
    AND,
    BINARY_AND,


    NUMBER,
    SPACE,
    VARIABLE,

    BAD_TOKEN,

    COMMENT,


    BOOL,
    INT,
    DOUBLE,
    CHAR,
    STRING,
    //VAR,
    //CONST,

    IF,
    LOOP,
    BREAK,
    CONTINUE,

    VOID,

    TRUE,
    FALSE,

    PRINT,
    INPUT,

    NONE,

    CARIAGE,    // \r

    RETURN,

    SEMI_COLON
}
