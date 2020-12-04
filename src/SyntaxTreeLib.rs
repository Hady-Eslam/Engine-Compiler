#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]



// Use Statements Here





#[derive(Debug)]
pub struct SyntaxTree{
    pub Statements: Trees::Statements_Syntax_Node
}




impl SyntaxTree{

    pub fn new() -> Self{
        return SyntaxTree{
            Statements: Trees::Statements_Syntax_Node::new()
        }
    }
}




impl SyntaxTree{

    pub fn Print(&self){
        println!("{:#?}", self);
    }
}




pub mod Trees{

    use crate::TokenTypeLib::TOKEN_TYPE;
    use crate::TokenLib::Token;

    #[derive(Debug, PartialEq)]
    #[derive(Clone)]
    pub enum STATEMENT_KIND{
        INITIALIZE,
        
        DEFINE_BOOL,
        DEFINE_INT,
        DEFINE_CHAR,
        DEFINE_STRING,
        DEFINE_DOUBLE,
        DEFINE_VAR,
        DEFINE_CONST,

        DEFINE_IF,
        DEFINE_SWITCH,
        DEFINE_LOOP,
        DEFINE_FUNCTION,
        
        RETURN,
        CALL_FUNCTION,
        PRINT,
        INPUT,
        RE_ASSIGN,

        BREAK,
        CONTINUE
    }



    #[derive(Debug, PartialEq)]
    #[derive(Clone)]
    pub enum OPERATION_TYPE{
        INITIALIZE,

        STRING_SEQUENCE,
        CHARACTER,
        VARIABLE,
        FUNCTION,

        MINUS_OPERATION,
        PARENTHESES_EXPRESSION,
        NUMBER,
        MATH_OPERATIONS,
        COMPARE_OPERATIONS,
        COMPARE_EXPRESSION,

        FOR_LOOP,
        WHILE_LOOP
    }



    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Statements_Syntax_Node{
        pub Statements: Vec<Statement_Syntax_Node>
    }


    impl Statements_Syntax_Node{

        pub fn new() -> Self{
            return Statements_Syntax_Node{
                Statements: Vec::new()
            }
        }
    }

    impl Statements_Syntax_Node{

        pub fn Add_Child(&mut self, Child: Statement_Syntax_Node){
            self.Statements.push(Child);
        }
    }




    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Statement_Syntax_Node{
        pub Kind: STATEMENT_KIND,
        
        pub Variable_Name: String,

        pub Bool_Value: Option<bool>,
        pub Char_Value: Option<String>,
        pub String_Value: Option<StringOperationNode>,
        pub int_Value: Option<OperationNode>,
        pub Double_Value: Option<OperationNode>,
        pub Variable_Value: Option<OperationNode>,

        pub Function_Params: Option<Vec<String>>,

        pub If: Option<If_Statement_Syntax_Node>,
        pub Switch: Option<Switch_Statement_Syntax_Node>,
        pub Loop: Option<Loop_Statement_Syntax_Node>,
        pub Function: Option<Function_Statement_Node>
    }


    impl Statement_Syntax_Node{

        pub fn new() -> Self{
            return Statement_Syntax_Node{
                Kind: STATEMENT_KIND::INITIALIZE,

                Variable_Name: String::new(),

                Bool_Value: None,
                Char_Value: None,
                String_Value: None,
                int_Value: None,
                Double_Value: None,
                Variable_Value: None,

                Function_Params: None,

                If: None,
                Switch: None,
                Loop: None,
                Function: None
            }
        }
    }





    #[derive(Debug, PartialEq)]
    #[derive(Clone)]
    pub struct StringOperationNode{
        pub Left: Option<Box<StringOperationNode>>,
        pub Right: Option<Box<StringOperationNode>>,

        pub Type: OPERATION_TYPE,
        pub Value: String,
        pub Function_Params: Vec<String>
    }


    impl StringOperationNode{

        pub fn new() -> Self{
            return StringOperationNode{
                Type: OPERATION_TYPE::INITIALIZE,
                Value: String::new(),
                Function_Params: Vec::new(),

                Left: None,
                Right: None
            }
        }
    }




    #[derive(Debug)]
    #[derive(Clone, PartialEq)]
    pub struct OperationNode{
        pub Type: OPERATION_TYPE,

        pub Value: String,

        pub Function_Params: Vec<String>,

        pub Operation: Option<TOKEN_TYPE>,
        
        pub Left: Option<Box<OperationNode>>,
        pub Right: Option<Box<OperationNode>>,
    }

    impl OperationNode{

        pub fn new() -> Self{
            return OperationNode{
                Type: OPERATION_TYPE::INITIALIZE,
                
                Value: String::new(),

                Function_Params: Vec::new(),

                Operation: None,

                Left: None,
                Right: None
            }
        }
    }




    #[derive(Debug)]
    #[derive(Clone)]
    pub struct If_Statement_Syntax_Node{
        pub Childs: Vec<If_Node>
    }


    impl If_Statement_Syntax_Node{
        pub fn new() -> Self{
            return If_Statement_Syntax_Node{
                Childs: Vec::new()
            }
        }
    }





    #[derive(Debug)]
    #[derive(Clone)]
    pub struct If_Node{
        pub Expression: Option<OperationNode>,
        pub Statements: Statements_Syntax_Node
    }


    impl If_Node{
        pub fn new() -> Self{
            return If_Node{
                Expression: None,
                Statements: Statements_Syntax_Node::new()
            }
        }
    }




    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Switch_Node{
        pub Variables: Vec<Token>,
        pub Statements: Statements_Syntax_Node
    }


    impl Switch_Node{
        pub fn new() -> Self{
            return Switch_Node{
                Variables: Vec::new(),
                Statements: Statements_Syntax_Node::new()
            }
        }
    }




    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Switch_Statement_Syntax_Node{
        pub Childs: Vec<Switch_Node>
    }


    impl Switch_Statement_Syntax_Node{
        pub fn new() -> Self{
            return Switch_Statement_Syntax_Node{
                Childs: Vec::new()
            }
        }
    }





    #[derive(Debug, Clone)]
    pub struct Loop_Statement_Syntax_Node{
        pub Expression: Option<Loop_Expression_Node>,
        pub Statements: Statements_Syntax_Node
    }

    impl Loop_Statement_Syntax_Node{
        pub fn new() -> Self{
            return Loop_Statement_Syntax_Node{
                Expression: None,
                Statements: Statements_Syntax_Node::new()
            }
        }
    }





    #[derive(Debug, Clone, PartialEq)]
    pub struct Loop_Expression_Node{
        pub Loop_Type: OPERATION_TYPE,

        pub Variable: String,

        pub First_Expression: Option<OperationNode>,
        pub Second_Expression: Option<OperationNode>,
        pub Third_Expression: Option<OperationNode>,
    }

    impl Loop_Expression_Node{
        pub fn new() -> Self{
            return Loop_Expression_Node{
                Loop_Type: OPERATION_TYPE::INITIALIZE,

                Variable: String::new(),

                First_Expression: None,
                Second_Expression: None,
                Third_Expression: None
            }
        }
    }





    #[derive(Debug, Clone)]
    pub struct Function_Statement_Node{
        pub Return_Type: Option<TOKEN_TYPE>,
        pub Function_Name: String,
        pub Childs: Vec<Function_Definition_Node>
    }


    impl Function_Statement_Node{
        pub fn new() -> Self{
            return Function_Statement_Node{
                Return_Type: None,
                Function_Name: String::new(),
                Childs: Vec::new()
            }
        }
    }




    #[derive(Debug, Clone)]
    pub struct Function_Definition_Node{
        pub Return_Type: Option<TOKEN_TYPE>,
        pub Variables: Vec<Function_Variable_Node>,
        pub Statements: Statements_Syntax_Node
    }

    impl Function_Definition_Node{
        pub fn new() -> Self{
            return Function_Definition_Node{
                Return_Type: None,
                Variables: Vec::new(),
                Statements: Statements_Syntax_Node::new()
            }
        }
    }




    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Function_Variable_Node{
        pub Type: TOKEN_TYPE,
        pub Name: String

    }

    impl Function_Variable_Node{
        pub fn new() -> Self{
            return Function_Variable_Node{
                Type: TOKEN_TYPE::VOID,
                Name: String::new()
            }
        }
    }
}
