#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]
#![allow(unreachable_code)]
#![allow(unused_must_use)]



// Use Statements Here
use std::collections::HashMap;
use crate::TokenTypeLib::TOKEN_TYPE;





#[derive(Debug)]
pub struct Analyzer{
    pub Current_Environment: Environments::Environment,
    pub Environment_Stack: Vec<Environments::Environment>,
    
    pub Functions: HashMap<String, Environments::Function>,

}

impl Analyzer{

    pub fn new(Scope: Environments::ENVIRNMENT) -> Self{
        return Analyzer{
            Current_Environment: Environments::Environment::new(Scope),
            Environment_Stack: Vec::new(),
            Functions: HashMap::new()
        }
    }
}

impl Analyzer{

    pub fn Is_Function_Defined(&self, Function_Name: &String) -> bool{
        return self.Functions.contains_key(Function_Name)
    }

    
    pub fn Register_Function(&mut self, _Function: Environments::Function){
        self.Functions.insert(_Function.Name.clone(), _Function);
    }


    pub fn Is_Variable_Defined(&self, Variable_Name: &String) -> bool{
        
        for _Environment in self.Environment_Stack.iter().rev(){

            if _Environment.Is_Variable_Defined(&Variable_Name){
                return true
            }
        }

        return self.Current_Environment.Is_Variable_Defined(&Variable_Name)
    }


    pub fn Get_Variable(&self, Variable_Name: &String) -> Environments::Variable{

        let mut __Variable = Environments::Variable::new();

        match self.Current_Environment.Variables.get(Variable_Name){
            None => (),
            Some(_Variable) => {
                __Variable = _Variable.clone();
            }
        }

        for _Environment in self.Environment_Stack.iter().rev(){

            match _Environment.Variables.get(Variable_Name){
                None => (),
                Some(_Variable) => {
                    __Variable = _Variable.clone();
                    break
                }
            }
        }

        return __Variable
    }


    
    pub fn Check_Params_Count(&mut self, Function_Name: &String, Params: &Vec<String>, Return_Types: Vec<TOKEN_TYPE>) -> bool{
        
        let mut _Function = self.Functions.get(Function_Name).unwrap();
        let mut _Function = _Function.Clone();

        for _Definition in _Function.Definitions.iter(){
                    
            if Return_Types.contains(&_Definition.Return_Type){
    
                if _Definition.Params.len() == Params.len(){

                    let mut Exact = true;

                    for (i, Param) in Params.iter().enumerate(){

                        let mut _Variable = self.Get_Variable(&Param);

                        if _Definition.Params[i].Type != _Variable.Type.unwrap(){
                            Exact = false;
                            break
                        }
                    }

                    if Exact{
                        return true
                    }
                }
            }
        }

        return false
    }

    
    pub fn Raise_Error(&self, Error: String){
        panic!("Symantic Analyzer Error: {}", Error);
    }
}





pub mod Environments{

    use crate::TokenTypeLib::TOKEN_TYPE;
    use crate::SyntaxTreeLib::Trees;
    use std::collections::HashMap;



    #[derive(Debug, Clone, PartialEq)]
    pub enum ENVIRNMENT{
        MAIN,
        IF,
        SWITCH,
        LOOP,
        FUNCTION(Function, usize)
    }




    
    #[derive(Debug, Clone)]
    pub struct Environment{
        pub Variables: HashMap<String, Variable>,
        pub Scope: ENVIRNMENT
    }

    impl Environment{

        pub fn new(Scope: ENVIRNMENT) -> Self{
            return Environment{
                Variables: HashMap::new(),
                Scope: Scope
            }
        }
    }

    impl Environment{


        pub fn Is_Variable_Defined(&self, Variable_Name: &String) -> bool{
            return self.Variables.contains_key(Variable_Name)
        }

        
        pub fn Register_Variable(&mut self, _Variable: Variable){
            self.Variables.insert(_Variable.Name.clone(), _Variable);
        }
    }




    #[derive(Debug, Clone, PartialEq)]
    pub struct Function{
        pub Name: String,

        pub Global_Return_Type: TOKEN_TYPE,

        pub Definitions: Vec<Definition>
    }

    impl Function{

        pub fn new() -> Self{
            return Function{
                Name: String::new(),

                Global_Return_Type: TOKEN_TYPE::VOID,

                Definitions: Vec::new()
            }
        }
    }

    impl Function{

        pub fn Is_Definition_Exists(&self, _Return_Type: TOKEN_TYPE, Params: &Vec<Trees::Function_Variable_Node>) -> bool{
            
            for _Definition in self.Definitions.iter(){
            
                if _Definition.Return_Type == _Return_Type{
            
                    if _Definition.Params.len() == Params.len(){

                        let mut Exact = true;
    
                        for i in 0..Params.len(){
                            if _Definition.Params[i].Type != Params[i].Type{
                                Exact = false;
                                break
                            }
                        }
    
                        if Exact{
                            return true
                        }
                    }
                }
            }

            return false
        }


        pub fn Clone(&self) -> Self{
            return Function{
                Name: self.Name.clone(),

                Global_Return_Type: self.Global_Return_Type,

                Definitions: self.Definitions.clone()
            }
        }
    }






    #[derive(Debug, Clone, PartialEq)]
    pub struct Definition{
        pub Return_Type: TOKEN_TYPE,
        pub Params: Vec<Function_Param>
    }

    impl Definition{

        pub fn new() -> Self{
            return Definition{
                Return_Type: TOKEN_TYPE::VOID,
                Params: Vec::new()
            }
        }
    }






    #[derive(Debug, Clone, PartialEq)]
    pub struct Function_Param{
        pub Name: String,
        pub Type: TOKEN_TYPE
    }

    impl Function_Param{

        pub fn new() -> Self{
            return Function_Param{
                Name: String::new(),
                Type: TOKEN_TYPE::VOID
            }
        }
    }
    






    #[derive(Debug, Clone)]
    pub struct Variable{
        pub Type: Option<TOKEN_TYPE>,

        pub Name: String,

        pub Bool_Value: Option<bool>,
        pub int_Value: Option<Trees::OperationNode>,
        pub Double_Value: Option<Trees::OperationNode>,
        pub Char_Value: Option<String>,
        pub String_Value: Option<Trees::StringOperationNode>
    }

    impl Variable{
        
        pub fn new() -> Self{
            return Variable{
                Type: None,

                Name: String::new(),

                Bool_Value: None,
                int_Value: None,
                Double_Value: None,
                Char_Value: None,
                String_Value: None
            }
        }
    }
}
