#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]





// Use Statements Here
//pub use Token_TYPE;
use crate::TokenTypeLib::TOKEN_TYPE;
use crate::CharacterLib::Character;
use crate::TokenLib::Token;
use crate::Filelib::File;
use crate::TokenLib::Get_Token_Type;





// Import Modules






#[derive(Debug)]
pub struct Lexer{
    __File: File,
    pub Line: u64,
    pub Position: u64
}





impl Lexer{

    pub fn new(File_Name: String) -> Self{
        let mut _File = File::new(File_Name);
        _File.Open();
        return Lexer{
            __File: _File,
            Line: 1,
            Position: 1
        }
    }
}




impl Lexer{

    fn __Current(&mut self) -> Character{
        return self.__Peek(0)
    }

    fn __Peek(&mut self, index: u64) -> Character{
        return self.__File.Peek(index);
    }

    fn __Next(&mut self) -> String{
        self.Position += 1;
        return self.__File.Read().To_String();
    }

    fn __Add_Line(&mut self){
        self.Line += 1;
        self.Position = 1;
    }

    fn __GetNumberToken(&mut self, Line: u64, Position: u64) -> Token{
        let mut _Number: String = String::new();
        
        while self.__Current().is_digit(){
            _Number += &self.__Next();
        }

        return Token{
            Token_Type: TOKEN_TYPE::NUMBER,
            Line,
            Position,
            Value: _Number
        }
    }

    fn __GetVariableToken(&mut self, Line: u64, Position: u64) -> Token{

        let mut _Variable = self.__Next();

        while self.__Current().is_alpha() || self.__Current().To_String() == "_" || self.__Current().is_digit(){
            _Variable += &self.__Next();
        }

        return Token{
            Token_Type: Get_Token_Type(&_Variable),
            Line,
            Position,
            Value: _Variable
        }
    }

    fn __GetStringToken(&mut self, Line: u64, Position: u64) -> Token{

        self.__Next();
        let mut _String: String = String::new();

        while !self.__Current().is_eof(){

            if self.__Current().is_newline(){
                _String += &self.__Next();
                self.__Add_Line();
            }

            else if self.__Current().To_String() == "\'"{

                self.__Next();
                if _String.len() < 2{
                    return Token{
                        Token_Type: TOKEN_TYPE::CHARACTER,
                        Line,
                        Position,
                        Value: _String
                    }
                }
                else{
                    return Token{
                        Token_Type: TOKEN_TYPE::STRING_SEQUENCE,
                        Line,
                        Position,
                        Value: _String
                    }
                }

            }

            else{
                _String += &self.__Next();
            }
        }

        panic!(format!("Lexer Exception: End Of File Reached in String in Line: {}:{}\n", Line, Position));
    }

    fn __GetCommentToken(&mut self, Line: u64, Position: u64) -> Token{
        
        let mut _Comment: String = String::from("---");
        self.__Next(); self.__Next(); self.__Next();

        while !self.__Current().is_eof(){

            if self.__Current().is_newline(){
                _Comment += &self.__Next();
                self.__Add_Line();
            }

            else if self.__Current().To_String() == "-" && self.__Peek(1).To_String() == "-" && self.__Peek(2).To_String() == "-"{
                self.__Next(); self.__Next(); self.__Next();
                _Comment.push_str("---");
                return Token{
                    Token_Type: TOKEN_TYPE::COMMENT,
                    Line,
                    Position,
                    Value: _Comment
                }
            }

            else{
                _Comment += &self.__Next();
            }
        }

        panic!(format!("Lexer Exception: Unexcepeted End Of File in Comment in Line: {}:{}", Line, Position));
    }
}






impl Lexer{

    pub fn Next_Token(&mut self) -> Token{

        if self.__Current().is_eof(){
            return Token{
                Token_Type: TOKEN_TYPE::EOF,
                Line: self.Line,
                Position: self.Position,
                Value: String::new()
            }
        }

        else if self.__Current().is_digit(){
            return self.__GetNumberToken(self.Line, self.Position)
        }

        else if self.__Current().is_alpha() || self.__Current().To_String() == "_"{
            return self.__GetVariableToken(self.Line, self.Position)
        }

        else if self.__Current().To_String() == "\r"{
            self.__Next();
            self.Position -= 1;
            return self.Next_Token();
        }

        else if self.__Current().is_newline(){
            self.__Next();
            self.__Add_Line();
            return self.Next_Token();
            /*return Token{
                Token_Type: TOKEN_TYPE::NEW_LINE,
                Line: self.Line -1,
                Position: self.Position -1,
                Value: String::from("\n")
            }*/
        }

        else if self.__Current().is_space(){
            self.__Next();
            return self.Next_Token();
        }

        else if self.__Current().To_String() == "="{
            if self.__Peek(1).To_String() == "="{
                self.__Next(); self.__Next();

                return Token{
                    Token_Type: TOKEN_TYPE::EQUAL,
                    Line: self.Line -2,
                    Position: self.Position -2,
                    Value: String::from("==")
                }
            }

            return Token{
                Token_Type: TOKEN_TYPE::ASSIGN,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "."{
            return Token{
                Token_Type: TOKEN_TYPE::DOT,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "\'"{
            return self.__GetStringToken(self.Line, self.Position)
        }

        else if self.__Current().To_String() == "{"{
            return Token{
                Token_Type: TOKEN_TYPE::OPEN_BRACKET,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "}"{
            return Token{
                Token_Type: TOKEN_TYPE::CLOSE_BRACKET,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == ","{
            return Token{
                Token_Type: TOKEN_TYPE::COMMA,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == ":"{
            return Token{
                Token_Type: TOKEN_TYPE::COLON,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "("{
            return Token{
                Token_Type: TOKEN_TYPE::OPEN_PARENTHESES,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == ")"{
            return Token{
                Token_Type: TOKEN_TYPE::CLOSE_PARENTHESES,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "+"{
            return Token{
                Token_Type: TOKEN_TYPE::PLUS,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "-"{
            if self.__Peek(1).To_String() == "-" && self.__Peek(2).To_String() == "-"{
                /*return*/ self.__GetCommentToken(self.Line, self.Position);
                return self.Next_Token();
            }
            return Token{
                Token_Type: TOKEN_TYPE::MINUS,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "*"{
            return Token{
                Token_Type: TOKEN_TYPE::MUL,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "/"{
            return Token{
                Token_Type: TOKEN_TYPE::DIV,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "%"{
            return Token{
                Token_Type: TOKEN_TYPE::MOD,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == ">"{
            if self.__Peek(1).To_String() == "="{
                self.__Next(); self.__Next();

                return Token{
                    Token_Type: TOKEN_TYPE::GREATER_THAN_OR_EQUAL,
                    Line: self.Line -2,
                    Position: self.Position -2,
                    Value: String::from(">=")
                }
            }

            return Token{
                Token_Type: TOKEN_TYPE::GREATER_THAN,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "<"{
            if self.__Peek(1).To_String() == "="{
                self.__Next(); self.__Next();

                return Token{
                    Token_Type: TOKEN_TYPE::LESS_THAN_OR_EQUAL,
                    Line: self.Line -2,
                    Position: self.Position -2,
                    Value: String::from("<=")
                }
            }

            return Token{
                Token_Type: TOKEN_TYPE::LESS_THAN,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "!"{
            if self.__Peek(1).To_String() == "="{
                self.__Next(); self.__Next();

                return Token{
                    Token_Type: TOKEN_TYPE::NOT_EQUAL,
                    Line: self.Line -2,
                    Position: self.Position -2,
                    Value: String::from("!=")
                }
            }

            return Token{
                Token_Type: TOKEN_TYPE::NOT,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "|"{

            if self.__Peek(1).To_String() == "|"{
                self.__Next(); self.__Next();

                return Token{
                    Token_Type: TOKEN_TYPE::OR,
                    Line: self.Line -2,
                    Position: self.Position -2,
                    Value: String::from("||")
                }
            }

            return Token{
                Token_Type: TOKEN_TYPE::BINARY_OR,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == "&"{

            if self.__Peek(1).To_String() == "&"{

                self.__Next(); self.__Next();

                return Token{
                    Token_Type: TOKEN_TYPE::AND,
                    Line: self.Line -2,
                    Position: self.Position -2,
                    Value: String::from("&&")
                }
            }
            
            return Token{
                Token_Type: TOKEN_TYPE::BINARY_AND,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else if self.__Current().To_String() == ";"{
            return Token{
                Token_Type: TOKEN_TYPE::SEMI_COLON,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }

        else{
            return Token{
                Token_Type: TOKEN_TYPE::BAD_TOKEN,
                Line: self.Line,
                Position: self.Position,
                Value: self.__Next()
            }
        }
    }
}
