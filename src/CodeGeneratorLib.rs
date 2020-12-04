#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]





// Use Statements Here
use crate::Filelib::File;
use std::collections::HashMap;





pub struct CodeGenerator{
    _File: File,
    pub Variables: HashMap<String, Environments::Variable>,
    pub Functions: HashMap<String, Environments::Function>,
    pub IF: Vec<String>,
    pub SWITCH: Vec<String>,
    pub LOOP: Vec<String>
}

impl CodeGenerator{

    pub fn new(Engine_Code_File: String) -> Self{
        File::Delete_File(Engine_Code_File.clone());

        return CodeGenerator{
            _File: File::create_new(Engine_Code_File.clone()),
            Variables: HashMap::new(),
            Functions: HashMap::new(),
            IF: Vec::new(),
            SWITCH: Vec::new(),
            LOOP: Vec::new()
        }
    }
}

impl CodeGenerator{

    pub fn Write(&mut self, string: String){
        self._File.Write(string);
    }


    pub fn Writeln(&mut self, string: String){
        self._File.Write(string + "\n");
    }
}



pub mod Environments{

    use std::collections::HashMap;
    use crate::TokenTypeLib::TOKEN_TYPE;


    #[derive(Debug)]
    pub struct Function{
        pub Name: String,
        pub Code_Name: String,
        pub Return_Type: TOKEN_TYPE,
        pub Definitions: HashMap<String, Definition>
    }

    impl Function{

        pub fn new(Name: String, Code_Name: String, Return_Type: TOKEN_TYPE) -> Self{
            return Function{
                Return_Type,
                Name,
                Code_Name,
                Definitions: HashMap::new()
            }
        }
    }




    #[derive(Debug)]
    pub struct Definition{
        pub Name: String,
        pub Return_Type: TOKEN_TYPE,
        pub Params: Vec<Param>
    }

    impl Definition{

        pub fn new(Name: String, Return_Type: TOKEN_TYPE) -> Self{
            return Definition{
                Name,
                Return_Type,
                Params: Vec::new()
            }
        }
    }



    #[derive(Debug)]
    pub struct Param{
        pub Name: String,
        pub Code_Name: String,
        pub Type: TOKEN_TYPE
    }

    impl Param{

        pub fn new() -> Self{
            return Param{
                Name: String::new(),
                Code_Name: String::new(),
                Type: TOKEN_TYPE::VOID
            }
        }
    }




    #[derive(Debug)]
    pub struct Variable{
        pub Name: String,
        pub Code_Name: String,
        pub Type: TOKEN_TYPE
    }

    impl Variable{

        pub fn new() -> Self{
            return Variable{
                Name: String::new(),
                Code_Name: String::new(),
                Type: TOKEN_TYPE::VOID
            }
        }
    }
}
