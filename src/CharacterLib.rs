#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]





// Import Modules





// Use Statements Here
use std::char;
use std::string::ToString;









#[derive(Debug)]
pub struct Character{
    __Char_Code: u8
}



impl Character{
    
    pub fn new(Char_Code: u8) -> Self{
        return Character{
            __Char_Code: Char_Code
        }
    }
}



impl Character{

    pub fn is_digit(&self) -> bool{
        return self.__Char_Code > 47 && self.__Char_Code < 58
    }


    pub fn is_alpha(&self) -> bool{
        return self.__Char_Code > 64 && self.__Char_Code < 91 || self.__Char_Code > 96 && self.__Char_Code < 123
    }


    pub fn is_space(&self) -> bool{
        return self.__Char_Code == 32 || self.__Char_Code == 9 || self.__Char_Code == 13
    }

    pub fn is_newline(&self) -> bool{
        return self.__Char_Code == 10
    }

    pub fn is_eof(&self) -> bool{
        return self.__Char_Code == 0
    }


    pub fn To_String(&self) -> String{
        if self.__Char_Code == 0{
            return String::from("\0")
        }
        else{
            return String::from(
                char::from_u32(self.__Char_Code.into()).unwrap().to_string()
            )
        }
    }
}
