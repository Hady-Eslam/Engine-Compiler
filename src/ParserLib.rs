#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]





// Use Statements Here
use std::vec::Vec;

use crate::LexerLib::Lexer;
use crate::TokenLib::Token;
use crate::TokenTypeLib::TOKEN_TYPE;
use crate::SyntaxTreeLib::SyntaxTree;





// Import Modules







#[derive(Debug)]
pub struct Parser{
    __Lexer: Lexer,
    __Current_Token: Token,
    __Token_Stack: Vec<Token>,
    pub Syntax_Tree: SyntaxTree
}






impl Parser{

    pub fn new(File_Name: String) -> Self{

        let mut _lexer = Lexer::new(File_Name);
        let mut _Current_Token = _lexer.Next_Token();

        return Parser{
            __Lexer: _lexer,
            __Current_Token: _Current_Token,
            __Token_Stack: Vec::new(),
            Syntax_Tree: SyntaxTree::new()
        }
    }
}






impl Parser{

    fn __Move(&mut self){
        if self.__Token_Stack.len() == 0{
            self.__Current_Token = self.__Lexer.Next_Token();
        }
        else{
            self.__Current_Token = self.__Token_Stack.pop().unwrap();
        }
    }
}






impl Parser{

    pub fn Current(&mut self) -> &Token{
        return &self.__Current_Token
    }


    pub fn Raise_Error(&mut self){
        panic!(format!(
            "Parser Error: Unexepected Token `{}` in Line: {}:{}",
            self.__Current_Token.Value,
            self.__Current_Token.Line,
            self.__Current_Token.Position
        ));
    }


    pub fn Match(&mut self, Type: TOKEN_TYPE){
        let _Token_Type = self.__Current_Token.Token_Type;

        if _Token_Type == Type{
            self.__Move();
        }
        else{
            self.Raise_Error();
        }
    }


    pub fn PrintToken(&self){
        println!("");
        print!("(");
            println!("\tType: {:?}", self.__Current_Token.Token_Type);
            println!("\tLine: {}", self.__Current_Token.Line);
            println!("\tPosition: {}", self.__Current_Token.Position);
            println!("\tValue: `{}`", self.__Current_Token.Value);
        println!(")");
        println!("");
    }


    pub fn Peek(&mut self, Number: usize) -> &Token{
        if Number == 0{
            return &self.__Current_Token
        }

        else if Number < self.__Token_Stack.len(){
            return &self.__Token_Stack[Number]
        }

        else{
            let mut Counter = Number - self.__Token_Stack.len();

            while Counter != 0{
                self.__Token_Stack.insert(0, self.__Lexer.Next_Token());
                Counter -= 1;
            }

            return &self.__Token_Stack[0]
        }
    }
}
