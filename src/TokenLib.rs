#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]



// import Modules




// Use Statements Here
use crate::TokenTypeLib::TOKEN_TYPE;






pub fn Get_Token_Type(Variable: &String) -> TOKEN_TYPE{

    if Variable == "bool"{
        return TOKEN_TYPE::BOOL
    }

    else if Variable == "int"{
        return TOKEN_TYPE::INT
    }

    else if Variable == "double"{
        return TOKEN_TYPE::DOUBLE
    }

    else if Variable == "char"{
        return TOKEN_TYPE::CHAR
    }

    else if Variable == "string"{
        return TOKEN_TYPE::STRING
    }

    /*else if Variable == "var"{
        return TOKEN_TYPE::VAR
    }

    else if Variable == "const"{
        return TOKEN_TYPE::CONST
    }*/

    else if Variable == "if"{
        return TOKEN_TYPE::IF
    }

    else if Variable == "Loop"{
        return TOKEN_TYPE::LOOP
    }

    else if Variable == "break"{
        return TOKEN_TYPE::BREAK
    }

    else if Variable == "continue"{
        return TOKEN_TYPE::CONTINUE
    }

    else if Variable == "void"{
        return TOKEN_TYPE::VOID
    }

    else if Variable == "True"{
        return TOKEN_TYPE::TRUE
    }

    else if Variable == "False"{
        return TOKEN_TYPE::FALSE
    }

    else if Variable == "print"{
        return TOKEN_TYPE::PRINT
    }

    else if Variable == "input"{
        return TOKEN_TYPE::INPUT
    }

    else if Variable == "return"{
        return TOKEN_TYPE::RETURN
    }

    else{
        return TOKEN_TYPE::VARIABLE
    }
}




#[derive(Debug, Clone)]
pub struct Token{
    pub Token_Type: TOKEN_TYPE,
    pub Line: u64,
    pub Position: u64,
    pub Value: String
}



impl Token{

    pub fn clone(&mut self) -> Token{
        return Token{
            Token_Type: self.Token_Type,
            Line: self.Line,
            Position: self.Position,
            Value: self.Value.clone()
        }
    }
}