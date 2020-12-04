#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]




// Use Statements Here
use crate::ParserLib::Parser;
use crate::TokenTypeLib::TOKEN_TYPE;
use crate::SyntaxTreeLib::Trees;
use crate::TokenLib::Token;






pub fn Parse(__Parser: &mut Parser){

    let mut __Parser = __Parser;
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();

    __Statements(&mut __Parser, String::from("\t"), true);
}


fn __Statements(__Parser: &mut Parser, intent: String, Raise_Unmatched_Error: bool){

    let mut __Parser = __Parser;
    while __Statement(&mut __Parser, intent.clone(), Raise_Unmatched_Error){}
}


fn __Statement(__Parser: &mut Parser, intent: String, Raise_Unmatched_Error: bool) -> bool{

    let mut __Parser = __Parser;

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    match __Parser.Current().Token_Type {

        TOKEN_TYPE::VOID => __Function_Statement(&mut __Parser, _intent),

        TOKEN_TYPE::BOOL => {
            match __Parser.Peek(2).Token_Type {
                
                TOKEN_TYPE::OPEN_PARENTHESES => __Function_Statement(&mut __Parser, _intent),

                _ => __Define_Bool(&mut __Parser, _intent)
            }
        },

        TOKEN_TYPE::INT => {
            
            match __Parser.Peek(2).Token_Type {
                
                TOKEN_TYPE::OPEN_PARENTHESES => __Function_Statement(&mut __Parser, _intent),

                _ => __Define_int(&mut __Parser, _intent)
            }
        },

        TOKEN_TYPE::CHAR => {
            
            match __Parser.Peek(2).Token_Type {
                
                TOKEN_TYPE::OPEN_PARENTHESES => __Function_Statement(&mut __Parser, _intent),

                _ => __Define_Char(&mut __Parser, _intent)
            }
        },

        TOKEN_TYPE::STRING => {
            
            match __Parser.Peek(2).Token_Type {
                
                TOKEN_TYPE::OPEN_PARENTHESES => __Function_Statement(&mut __Parser, _intent),

                _ => __Define_String(&mut __Parser, _intent)
            }
        },

        TOKEN_TYPE::DOUBLE => {
            
            match __Parser.Peek(2).Token_Type {
                
                TOKEN_TYPE::OPEN_PARENTHESES => __Function_Statement(&mut __Parser, _intent),

                _ => __Define_Double(&mut __Parser, _intent)
            }
        },

        //TOKEN_TYPE::VAR => __Define_Var(&mut __Parser, _intent),

        //TOKEN_TYPE::CONST => __Define_Const(&mut __Parser, _intent),

        TOKEN_TYPE::IF => __If_Statement(&mut __Parser, _intent),

        TOKEN_TYPE::LOOP => __Loop_Statement(&mut __Parser, _intent),


        TOKEN_TYPE::BREAK => {
            __Parser.Match(TOKEN_TYPE::BREAK);
            __Parser.Match(TOKEN_TYPE::SEMI_COLON);

            let mut _Node = Trees::Statement_Syntax_Node::new();
            _Node.Kind = Trees::STATEMENT_KIND::BREAK;
            __Parser.Syntax_Tree.Statements.Add_Child(_Node);
        },

        TOKEN_TYPE::CONTINUE => {
            __Parser.Match(TOKEN_TYPE::CONTINUE);
            __Parser.Match(TOKEN_TYPE::SEMI_COLON);

            let mut _Node = Trees::Statement_Syntax_Node::new();
            _Node.Kind = Trees::STATEMENT_KIND::CONTINUE;
            __Parser.Syntax_Tree.Statements.Add_Child(_Node);
        },


        TOKEN_TYPE::RETURN => __Return_Statement(&mut __Parser, _intent),


        TOKEN_TYPE::PRINT => __Print_Statement(&mut __Parser, _intent),


        TOKEN_TYPE::INPUT => __Input_Statement(&mut __Parser, _intent),


        TOKEN_TYPE::VARIABLE => {

            match __Parser.Peek(1).Token_Type{

                TOKEN_TYPE::COLON => __Switch_Statement(&mut __Parser, _intent),

                TOKEN_TYPE::OPEN_PARENTHESES => __Function_Statement(&mut __Parser, _intent),

                TOKEN_TYPE::DOT => __Function_Call(&mut __Parser, _intent),

                //TOKEN_TYPE::ASSIGN => __ReAssign_Statement(&mut __Parser, _intent),

                _ => {
                    if Raise_Unmatched_Error{
                        __Parser.Raise_Error();
                    }
                    return false
                }
            }
        },


        TOKEN_TYPE::EOF => return false,

        TOKEN_TYPE::BAD_TOKEN => __Parser.Raise_Error(),

        _ => {
            if Raise_Unmatched_Error{
                __Parser.Raise_Error();
            }
            return false
        }
    }

    return true
}



fn __Define_Bool(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_BOOL;


    __Parser.Match(TOKEN_TYPE::BOOL);


    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::TRUE => {
            __Parser.Match(TOKEN_TYPE::TRUE);
            _Node.Bool_Value = Some(true);
        },
        
        _ => {
            __Parser.Match(TOKEN_TYPE::FALSE);
            _Node.Bool_Value = Some(false);
        }
    }

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);


    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __Define_int(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_INT;


    __Parser.Match(TOKEN_TYPE::INT);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);

    _Node.int_Value = Some(__Expression(&mut __Parser));

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __Expression(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    
    let mut _Node = Trees::OperationNode::new();
    _Node.Left = Some(Box::new( __Term(&mut __Parser) ));

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::PLUS => {
            _Node.Type = Trees::OPERATION_TYPE::MATH_OPERATIONS;

            _Node.Operation = Some(TOKEN_TYPE::PLUS);
            __Parser.Match(TOKEN_TYPE::PLUS);
            _Node.Right = Some(Box::new( __Expression(&mut __Parser) ));
        },
        
        TOKEN_TYPE::MINUS => {
            _Node.Type = Trees::OPERATION_TYPE::MATH_OPERATIONS;

            _Node.Operation = Some(TOKEN_TYPE::MINUS);
            __Parser.Match(TOKEN_TYPE::MINUS);
            _Node.Right = Some(Box::new( __Expression(&mut __Parser) ));
        },
        
        _ => return *_Node.Left.unwrap()
    }

    return _Node
}



fn __Term(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    let mut _Node = Trees::OperationNode::new();

    _Node.Left = Some(Box::new( __Factor(&mut __Parser) ));

    match __Parser.Current().Token_Type {

        TOKEN_TYPE::MUL => {
            _Node.Type = Trees::OPERATION_TYPE::MATH_OPERATIONS;

            _Node.Operation = Some(TOKEN_TYPE::MUL);
            __Parser.Match(TOKEN_TYPE::MUL);
            _Node.Right = Some(Box::new( __Term(&mut __Parser) ));
        },

        TOKEN_TYPE::DIV => {
            _Node.Type = Trees::OPERATION_TYPE::MATH_OPERATIONS;

            _Node.Operation = Some(TOKEN_TYPE::DIV);
            __Parser.Match(TOKEN_TYPE::DIV);
            _Node.Right = Some(Box::new( __Term(&mut __Parser) ));
        },

        TOKEN_TYPE::MOD => {
            _Node.Type = Trees::OPERATION_TYPE::MATH_OPERATIONS;

            _Node.Operation = Some(TOKEN_TYPE::MOD);
            __Parser.Match(TOKEN_TYPE::MOD);
            _Node.Right = Some(Box::new( __Term(&mut __Parser) ));
        },

        _ => return *_Node.Left.unwrap()
    }

    return _Node
}



fn __Factor(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    let mut _Node = Trees::OperationNode::new();

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::MINUS => {
            _Node.Type = Trees::OPERATION_TYPE::MINUS_OPERATION;

            __Parser.Match(TOKEN_TYPE::MINUS);
            _Node.Left = Some(Box::new( __Item(&mut __Parser) ));
            return _Node
        },

        _ => return __Item(&mut __Parser)
    }
}



fn __Item(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    let mut _Node = Trees::OperationNode::new();

    
    match __Parser.Current().Token_Type {

        TOKEN_TYPE::OPEN_PARENTHESES => {
            
            _Node.Type = Trees::OPERATION_TYPE::PARENTHESES_EXPRESSION;

            __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);
            _Node.Left = Some(Box::new( __Expression(&mut __Parser) ));
            __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);
        },

        TOKEN_TYPE::NUMBER => {

            _Node.Type = Trees::OPERATION_TYPE::NUMBER;

            _Node.Value = __Parser.Current().Value.clone();
            __Parser.Match(TOKEN_TYPE::NUMBER);

            match __Parser.Current().Token_Type{
                
                TOKEN_TYPE::DOT => {
                    __Parser.Match(TOKEN_TYPE::DOT);
                    _Node.Value.push_str(".");

                    _Node.Value += &__Parser.Current().Value.clone();
                    __Parser.Match(TOKEN_TYPE::NUMBER);
                },

                _ => ()
            }
        },

        TOKEN_TYPE::VARIABLE => {

            _Node.Type = Trees::OPERATION_TYPE::VARIABLE;

            _Node.Value = __Parser.Current().Value.clone();
            __Parser.Match(TOKEN_TYPE::VARIABLE);

            match __Parser.Current().Token_Type{

                TOKEN_TYPE::DOT => {
                    _Node.Type = Trees::OPERATION_TYPE::FUNCTION;

                    __Parser.Match(TOKEN_TYPE::DOT);
                    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

                    match __Parser.Current().Token_Type{

                        TOKEN_TYPE::CLOSE_PARENTHESES => __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES),

                        _ => {
                            _Node.Function_Params = __Call_Function_Params(&mut __Parser);
                            __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);
                        }
                    }
                },

                _ => ()
            }
        },

        _ => __Parser.Raise_Error()
    }

    return _Node
}



fn __Define_Char(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_CHAR;

    


    __Parser.Match(TOKEN_TYPE::CHAR);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);

    _Node.Char_Value = Some(__Parser.Current().Value.clone());
    __Parser.Match(TOKEN_TYPE::CHARACTER);

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}






fn __Define_String(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_STRING;

    


    __Parser.Match(TOKEN_TYPE::STRING);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);

    _Node.String_Value = Some( __String_Expression(&mut __Parser) );

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}




fn __String_Expression(__Parser: &mut Parser) -> Trees::StringOperationNode{

    let mut __Parser = __Parser;
    
    let mut _Node = __String_Term(&mut __Parser);

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::PLUS => {
            __Parser.Match(TOKEN_TYPE::PLUS);
            _Node.Left = Some(Box::new( __String_Expression(&mut __Parser) ));
        },

        _ => ()
    }

    return _Node
}




fn __String_Term(__Parser: &mut Parser) -> Trees::StringOperationNode{

    let mut __Parser = __Parser;
    let mut _Node = Trees::StringOperationNode::new();

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::STRING_SEQUENCE => {
            
            _Node.Type = Trees::OPERATION_TYPE::STRING_SEQUENCE;
            _Node.Value = __Parser.Current().Value.clone();
            
            __Parser.Match(TOKEN_TYPE::STRING_SEQUENCE);
        },

        TOKEN_TYPE::CHARACTER => {

            _Node.Type = Trees::OPERATION_TYPE::CHARACTER;
            _Node.Value = __Parser.Current().Value.clone();

            __Parser.Match(TOKEN_TYPE::CHARACTER);
        },

        TOKEN_TYPE::VARIABLE => {

            _Node.Type = Trees::OPERATION_TYPE::VARIABLE;

            _Node.Value = __Parser.Current().Value.clone();
            __Parser.Match(TOKEN_TYPE::VARIABLE);

            match __Parser.Current().Token_Type{

                TOKEN_TYPE::DOT => {

                    _Node.Type = Trees::OPERATION_TYPE::FUNCTION;

                    __Parser.Match(TOKEN_TYPE::DOT);

                    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

                    match __Parser.Current().Token_Type {
                        
                        TOKEN_TYPE::CLOSE_PARENTHESES => __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES),

                        _ => {
                            _Node.Function_Params = __Call_Function_Params(&mut __Parser);
                            __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);
                        }
                    }
                },

                _ => ()
            }
        },

        _ => __Parser.Raise_Error()
    }

    return _Node
}




fn __Define_Double(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_DOUBLE;
    
    

    __Parser.Match(TOKEN_TYPE::DOUBLE);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);

    _Node.Double_Value = Some( __Expression(&mut __Parser) );

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}


/*
fn __Define_Var(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_VAR;

    


    __Parser.Match(TOKEN_TYPE::VAR);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);


    match __Parser.Current().Token_Type{

        TOKEN_TYPE::TRUE => {
            _Node.Bool_Value = Some(true);
            __Parser.Match(TOKEN_TYPE::TRUE);
        },

        TOKEN_TYPE::FALSE => {
            _Node.Bool_Value = Some(false);
            __Parser.Match(TOKEN_TYPE::FALSE);
        },

        TOKEN_TYPE::STRING_SEQUENCE => {
            _Node.String_Value = Some(__String_Expression(&mut __Parser) );
        },

        TOKEN_TYPE::CHARACTER => {
            _Node.String_Value = Some(__String_Expression(&mut __Parser) );
        },

        _ => {
            _Node.Double_Value = Some( __Expression(&mut __Parser) );
        }
    }

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __Define_Const(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;

    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_CONST;

    


    __Parser.Match(TOKEN_TYPE::CONST);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);


    match __Parser.Current().Token_Type{

        TOKEN_TYPE::TRUE => {
            _Node.Bool_Value = Some(true);
            __Parser.Match(TOKEN_TYPE::TRUE);
        },

        TOKEN_TYPE::FALSE => {
            _Node.Bool_Value = Some(false);
            __Parser.Match(TOKEN_TYPE::FALSE);
        },

        TOKEN_TYPE::STRING_SEQUENCE => {
            _Node.String_Value = Some(__String_Expression(&mut __Parser) );
        },

        TOKEN_TYPE::CHARACTER => {
            _Node.String_Value = Some(__String_Expression(&mut __Parser) );
        },

        _ => {
            _Node.Double_Value = Some( __Expression(&mut __Parser) );
        }
    }

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}
*/



fn __If_Statement(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_IF;

    let mut _IfNodeStatement = Trees::If_Statement_Syntax_Node::new();

    

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');


    __Parser.Match(TOKEN_TYPE::IF);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let _IfNode = __First_if(&mut __Parser, _intent.clone());

    _IfNodeStatement.Childs =  __else_if(&mut __Parser, _intent);

    _IfNodeStatement.Childs.insert(0, _IfNode);

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    _Node.If = Some(_IfNodeStatement);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __First_if(__Parser: &mut Parser, intent: String) -> Trees::If_Node{

    let mut __Parser = __Parser;
    let mut _Node = Trees::If_Node::new();

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    
    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    _Node.Expression = Some(__If_Expression(&mut __Parser));

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let mut _Old_Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();

    __Statements(&mut __Parser, _intent, false);

    _Node.Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = _Old_Statements;

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    

    return _Node
}



fn __If_Expression(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    
    let mut _Node = Trees::OperationNode::new();
    _Node.Type = Trees::OPERATION_TYPE::COMPARE_OPERATIONS;
    _Node.Left = Some(Box::new( __If_Expression_Term(&mut __Parser) ));

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::OR => {
            _Node.Operation = Some(TOKEN_TYPE::OR);
            __Parser.Match(TOKEN_TYPE::OR);
            _Node.Right = Some(Box::new( __If_Expression(&mut __Parser) ));
            return _Node
        },

        _ => return *_Node.Left.unwrap()
    }
}



fn __If_Expression_Term(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    
    let mut _Node = Trees::OperationNode::new();
    _Node.Type = Trees::OPERATION_TYPE::COMPARE_OPERATIONS;
    _Node.Left = Some(Box::new( __If_Expression_Factor(&mut __Parser) ));

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::AND => {
            _Node.Operation = Some(TOKEN_TYPE::AND);
            __Parser.Match(TOKEN_TYPE::AND);
            _Node.Right = Some(Box::new( __If_Expression_Term(&mut __Parser) ));
            return _Node
        },

        _ => return *_Node.Left.unwrap()
    }
}



fn __If_Expression_Factor(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;

    match __Parser.Current().Token_Type {

        TOKEN_TYPE::OPEN_PARENTHESES => {

            let mut _Node = Trees::OperationNode::new();
            _Node.Type = Trees::OPERATION_TYPE::COMPARE_EXPRESSION;

            __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);
            _Node.Left = Some(Box::new( __If_Expression(&mut __Parser) ));
            __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

            return _Node
        },

        _ => {
            return __Compare_Expression(&mut __Parser);
        }
    }
}



fn __Compare_Expression(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    let mut _Node = Trees::OperationNode::new();
    _Node.Type = Trees::OPERATION_TYPE::COMPARE_OPERATIONS;

    _Node.Left = Some(Box::new( __Expression(&mut __Parser) ));

    match __Parser.Current().Token_Type{
        
        TOKEN_TYPE::GREATER_THAN => {
            _Node.Operation = Some(TOKEN_TYPE::GREATER_THAN);
            __Parser.Match(TOKEN_TYPE::GREATER_THAN);
        },

        TOKEN_TYPE::GREATER_THAN_OR_EQUAL => {
            _Node.Operation = Some(TOKEN_TYPE::GREATER_THAN_OR_EQUAL);
            __Parser.Match(TOKEN_TYPE::GREATER_THAN_OR_EQUAL);
        },

        TOKEN_TYPE::LESS_THAN => {
            _Node.Operation = Some(TOKEN_TYPE::LESS_THAN);
            __Parser.Match(TOKEN_TYPE::LESS_THAN);
        },

        TOKEN_TYPE::LESS_THAN_OR_EQUAL => {
            _Node.Operation = Some(TOKEN_TYPE::LESS_THAN_OR_EQUAL);
            __Parser.Match(TOKEN_TYPE::LESS_THAN_OR_EQUAL);
        },

        TOKEN_TYPE::EQUAL => {
            _Node.Operation = Some(TOKEN_TYPE::EQUAL);
            __Parser.Match(TOKEN_TYPE::EQUAL);
        },

        TOKEN_TYPE::NOT_EQUAL => {
            _Node.Operation = Some(TOKEN_TYPE::NOT_EQUAL);
            __Parser.Match(TOKEN_TYPE::NOT_EQUAL);
        },

        _ => __Parser.Raise_Error()
    }

    _Node.Right = Some(Box::new( __Expression(&mut __Parser) ));
    return _Node
}




fn __else_if(__Parser: &mut Parser, intent: String) -> Vec<Trees::If_Node>{

    let mut __Parser = __Parser;
    let mut _If_Nodes: Vec<Trees::If_Node> = Vec::new();

    loop{
        if __Parser.Current().Token_Type == TOKEN_TYPE::CLOSE_PARENTHESES{
            return _If_Nodes
        }

        else if 
            __Parser.Peek(0).Token_Type == TOKEN_TYPE::OPEN_PARENTHESES &&
            __Parser.Peek(1).Token_Type == TOKEN_TYPE::CLOSE_PARENTHESES{
            
            _If_Nodes.push( __else(&mut __Parser, intent.clone()) );
            return _If_Nodes
        }

        _If_Nodes.push( __First_if(&mut __Parser, intent.clone()) );
    }
}



fn __else(__Parser: &mut Parser, intent: String) -> Trees::If_Node{

    

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut __Parser = __Parser;
    let mut _Node = Trees::If_Node::new();
    _Node.Expression = None;

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let _Old_Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();

    __Statements(&mut __Parser, _intent, false);

    _Node.Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = _Old_Statements;

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    

    return _Node
}



fn __Switch_Statement(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_SWITCH;

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut _Switch_Node = Trees::Switch_Statement_Syntax_Node::new();

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let _First_Node = __First_Switch(&mut __Parser, _intent.clone());

    _Switch_Node.Childs = __else_Switch(&mut __Parser, _intent);
    _Switch_Node.Childs.insert(0, _First_Node);

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    _Node.Switch = Some(_Switch_Node);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);
}



fn __First_Switch(__Parser: &mut Parser, intent: String) -> Trees::Switch_Node{

    let mut __Parser = __Parser;
    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut _Node = Trees::Switch_Node::new();

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);
    _Node.Variables = __Switch_Variables(&mut __Parser);
    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let _Old_Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();
    
    __Statements(&mut __Parser, _intent, false);

    _Node.Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = _Old_Statements;
    
    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    

    return _Node
}


fn __Switch_Variables(__Parser: &mut Parser) -> Vec<Token>{

    let mut __Parser = __Parser;
    let mut _Variables: Vec<Token> = Vec::new();

    loop{

        match __Parser.Current().Token_Type{

            TOKEN_TYPE::NUMBER => {
                _Variables.push(__Parser.Current().clone());
                __Parser.Match(TOKEN_TYPE::NUMBER);
            },

            TOKEN_TYPE::CHARACTER => {
                _Variables.push(__Parser.Current().clone());
                __Parser.Match(TOKEN_TYPE::CHARACTER);
            },

            TOKEN_TYPE::STRING_SEQUENCE => {
                _Variables.push(__Parser.Current().clone());
                __Parser.Match(TOKEN_TYPE::STRING_SEQUENCE);
            },

            _ => __Parser.Raise_Error()
        }

        match __Parser.Current().Token_Type{
            
            TOKEN_TYPE::COMMA => __Parser.Match(TOKEN_TYPE::COMMA),

            _ => return _Variables
        }
    }
}



fn __else_Switch(__Parser: &mut Parser, intent: String) -> Vec<Trees::Switch_Node>{

    let mut __Parser = __Parser;
    let mut _intent = intent.clone();
    let mut _Nodes: Vec<Trees::Switch_Node> = Vec::new();

    loop {

        if __Parser.Current().Token_Type == TOKEN_TYPE::CLOSE_PARENTHESES{
            return _Nodes
        }

        else if
            __Parser.Peek(0).Token_Type == TOKEN_TYPE::OPEN_PARENTHESES &&
            __Parser.Peek(1).Token_Type == TOKEN_TYPE::CLOSE_PARENTHESES{
            
            _Nodes.push( __default_Switch(&mut __Parser, _intent.clone()) );
            return _Nodes
        }

        _Nodes.push( __First_Switch(&mut __Parser, _intent.clone()) );
    }
}



fn __default_Switch(__Parser: &mut Parser, intent: String) -> Trees::Switch_Node{

    let mut __Parser = __Parser;
    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut _Node = Trees::Switch_Node::new();
    _Node.Variables = Vec::new();

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let _Old_Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();

    __Statements(&mut __Parser, _intent, false);

    _Node.Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = _Old_Statements;

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    

    return _Node
}



fn __Loop_Statement(__Parser: &mut Parser, intent: String){

    let mut __Parser = __Parser;
    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_LOOP;

    let mut _Loop_Node = Trees::Loop_Statement_Syntax_Node::new();

    __Parser.Match(TOKEN_TYPE::LOOP);

    __Parser.Match(TOKEN_TYPE::COLON);
    _Loop_Node.Expression = __Loop_Expression(&mut __Parser);
    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let mut _Old_Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();

    __Statements(&mut __Parser, _intent, false);
    
    _Loop_Node.Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = _Old_Statements;

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    _Node.Loop = Some(_Loop_Node);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __Loop_Expression(__Parser: &mut Parser) -> Option<Trees::Loop_Expression_Node>{

    let mut __Parser = __Parser;

    if __Parser.Current().Token_Type == TOKEN_TYPE::COLON{
        return None
    }

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);
    let mut _Node = Trees::Loop_Expression_Node::new();

    if __Parser.Current().Token_Type == TOKEN_TYPE::INT{
        _Node = __For_Loop_Expression(&mut __Parser);
    }

    else{
        _Node.Loop_Type = Trees::OPERATION_TYPE::WHILE_LOOP;
        _Node.First_Expression = Some( __While_Loop_Expression(&mut __Parser) );
    }

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    return Some(_Node)
}



fn __While_Loop_Expression(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;
    
    let mut _Node = Trees::OperationNode::new();
    _Node.Type = Trees::OPERATION_TYPE::COMPARE_OPERATIONS;
    _Node.Left = Some(Box::new( __While_Loop_Factor(&mut __Parser) ));

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::OR => {
            _Node.Operation = Some( TOKEN_TYPE::OR );
            __Parser.Match(TOKEN_TYPE::OR);
            _Node.Right = Some(Box::new( __While_Loop_Expression(&mut __Parser) ));
        },

        _ => return *_Node.Left.unwrap()
    }

    return _Node
}


fn __While_Loop_Factor(__Parser: &mut Parser) -> Trees::OperationNode{
    let mut __Parser = __Parser;
    
    let mut _Node = Trees::OperationNode::new();
    _Node.Type = Trees::OPERATION_TYPE::COMPARE_OPERATIONS;
    _Node.Left = Some(Box::new( __While_Loop_Term(&mut __Parser) ));

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::AND => {
            _Node.Operation = Some( TOKEN_TYPE::AND );
            __Parser.Match(TOKEN_TYPE::AND);
            _Node.Right = Some(Box::new( __While_Loop_Expression(&mut __Parser) ));
        },

        _ => return *_Node.Left.unwrap()
    }

    return _Node
}



fn __While_Loop_Term(__Parser: &mut Parser) -> Trees::OperationNode{

    let mut __Parser = __Parser;

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::OPEN_PARENTHESES => {
            let mut _Node = Trees::OperationNode::new();
            _Node.Type = Trees::OPERATION_TYPE::COMPARE_EXPRESSION;

            __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);
            _Node.Left = Some(Box::new( __While_Loop_Expression(&mut __Parser) ));
            __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

            return _Node
        },

        _ => return __Compare_Expression(&mut __Parser)
    }
}



fn __For_Loop_Expression(__Parser: &mut Parser) -> Trees::Loop_Expression_Node{

    let mut __Parser = __Parser;
    let mut _Node = Trees::Loop_Expression_Node::new();
    _Node.Loop_Type = Trees::OPERATION_TYPE::FOR_LOOP;

    __Parser.Match(TOKEN_TYPE::INT);

    _Node.Variable = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::COLON);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    _Node.First_Expression = Some( __Expression(&mut __Parser) );
    
    if __Parser.Current().Token_Type == TOKEN_TYPE::COMMA{

        __Parser.Match(TOKEN_TYPE::COMMA);
        _Node.Second_Expression = Some( __Expression(&mut __Parser) );

        if __Parser.Current().Token_Type == TOKEN_TYPE::COMMA{

            __Parser.Match(TOKEN_TYPE::COMMA);
            _Node.Third_Expression = Some( __Expression(&mut __Parser) );
        }
    }
    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    return _Node
}



fn __Function_Statement(__Parser: &mut Parser, intent: String){

    

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::DEFINE_FUNCTION;

    let mut _Function_Node = Trees::Function_Statement_Node::new();

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::VOID => {
            _Function_Node.Return_Type = Some(TOKEN_TYPE::VOID);
            __Parser.Match(TOKEN_TYPE::VOID);
        },

        TOKEN_TYPE::BOOL => {
            _Function_Node.Return_Type = Some(TOKEN_TYPE::BOOL);
            __Parser.Match(TOKEN_TYPE::BOOL);
        },

        TOKEN_TYPE::INT => {
            _Function_Node.Return_Type = Some(TOKEN_TYPE::INT);
            __Parser.Match(TOKEN_TYPE::INT);
        },

        TOKEN_TYPE::DOUBLE => {
            _Function_Node.Return_Type = Some(TOKEN_TYPE::DOUBLE);
            __Parser.Match(TOKEN_TYPE::DOUBLE);
        },

        TOKEN_TYPE::CHAR => {
            _Function_Node.Return_Type = Some(TOKEN_TYPE::CHAR);
            __Parser.Match(TOKEN_TYPE::CHAR);
        },

        TOKEN_TYPE::STRING => {
            _Function_Node.Return_Type = Some(TOKEN_TYPE::STRING);
            __Parser.Match(TOKEN_TYPE::STRING);
        },

        _ => _Function_Node.Return_Type = None
    }

    _Function_Node.Function_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    let _First_Difinition = __First_Definition(&mut __Parser, _intent.clone());

    _Function_Node.Childs = __Another_Definition(&mut __Parser, _intent);
    _Function_Node.Childs.insert(0, _First_Difinition);
    
    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    _Node.Function = Some(_Function_Node);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __First_Definition(__Parser: &mut Parser, intent: String) -> Trees::Function_Definition_Node{

    

    let mut __Parser = __Parser;
    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut _Node = Trees::Function_Definition_Node::new();

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::VOID => {
            _Node.Return_Type = Some(TOKEN_TYPE::VOID);
            __Parser.Match(TOKEN_TYPE::VOID);
        },

        TOKEN_TYPE::BOOL => {
            _Node.Return_Type = Some(TOKEN_TYPE::BOOL);
            __Parser.Match(TOKEN_TYPE::BOOL);
        },

        TOKEN_TYPE::INT => {
            _Node.Return_Type = Some(TOKEN_TYPE::INT);
            __Parser.Match(TOKEN_TYPE::INT);
        },

        TOKEN_TYPE::DOUBLE => {
            _Node.Return_Type = Some(TOKEN_TYPE::DOUBLE);
            __Parser.Match(TOKEN_TYPE::DOUBLE);
        },

        TOKEN_TYPE::CHAR => {
            _Node.Return_Type = Some(TOKEN_TYPE::CHAR);
            __Parser.Match(TOKEN_TYPE::CHAR);
        },

        TOKEN_TYPE::STRING => {
            _Node.Return_Type = Some(TOKEN_TYPE::STRING);
            __Parser.Match(TOKEN_TYPE::STRING);
        },

        _ => _Node.Return_Type = None
    }


    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    if __Parser.Current().Token_Type != TOKEN_TYPE::CLOSE_PARENTHESES{
        _Node.Variables = __Function_Variables(&mut __Parser);
    }

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    let mut _Old_Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = Trees::Statements_Syntax_Node::new();

    if __Parser.Current().Token_Type == TOKEN_TYPE::MINUS{

        __Parser.Match(TOKEN_TYPE::MINUS);
        __Parser.Match(TOKEN_TYPE::GREATER_THAN);

        __Statement(&mut __Parser, _intent, true);
    }

    else{       

        __Parser.Match(TOKEN_TYPE::COLON);

        __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);
        __Statements(&mut __Parser, _intent, false);
        __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);
    }


    _Node.Statements = __Parser.Syntax_Tree.Statements.clone();
    __Parser.Syntax_Tree.Statements = _Old_Statements;

    

    return _Node
}



fn __Function_Variables(__Parser: &mut Parser) -> Vec<Trees::Function_Variable_Node>{

    let mut __Parser = __Parser;
    let mut _Function_Variables: Vec<Trees::Function_Variable_Node> = Vec::new();

    loop{
        let mut _Variable = Trees::Function_Variable_Node::new();

        match __Parser.Current().Token_Type{

            TOKEN_TYPE::BOOL => {
                _Variable.Type = TOKEN_TYPE::BOOL;
                __Parser.Match(TOKEN_TYPE::BOOL);
            },
    
            TOKEN_TYPE::INT => {
                _Variable.Type = TOKEN_TYPE::INT;
                __Parser.Match(TOKEN_TYPE::INT);
            },
    
            TOKEN_TYPE::DOUBLE => {
                _Variable.Type = TOKEN_TYPE::DOUBLE;
                __Parser.Match(TOKEN_TYPE::DOUBLE);
            },
    
            TOKEN_TYPE::CHAR => {
                _Variable.Type = TOKEN_TYPE::CHAR;
                __Parser.Match(TOKEN_TYPE::CHAR);
            },
    
            TOKEN_TYPE::STRING => {
                _Variable.Type = TOKEN_TYPE::STRING;
                __Parser.Match(TOKEN_TYPE::STRING);
            },
    
            _ => __Parser.Raise_Error()
        }
        
        _Variable.Name = __Parser.Current().Value.clone();
        __Parser.Match(TOKEN_TYPE::VARIABLE);

        
        _Function_Variables.push(_Variable);

        match __Parser.Current().Token_Type{
            
            TOKEN_TYPE::COMMA => __Parser.Match(TOKEN_TYPE::COMMA),

            _ => return _Function_Variables
        }
    }
}




fn __Another_Definition(__Parser: &mut Parser, intent: String) -> Vec<Trees::Function_Definition_Node>{

    let mut __Parser = __Parser;

    let mut _intent = intent.clone();
    _intent.insert(0, '\t');

    let mut _Difinitions: Vec<Trees::Function_Definition_Node> = Vec::new();

    loop{

        if __Parser.Current().Token_Type == TOKEN_TYPE::CLOSE_PARENTHESES{
            return _Difinitions
        }

        _Difinitions.push( __First_Definition(&mut __Parser, _intent.clone()) );
    }
}



fn __Return_Statement(__Parser: &mut Parser, intent: String){

    

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::RETURN;

    __Parser.Match(TOKEN_TYPE::RETURN);

    if __Parser.Current().Token_Type == TOKEN_TYPE::VARIABLE{
        _Node.Variable_Name = __Parser.Current().Value.clone();
        __Parser.Match(TOKEN_TYPE::VARIABLE);
    }

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);

    
}



fn __Function_Call(__Parser: &mut Parser, intent: String){    

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::CALL_FUNCTION;

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::DOT);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    _Node.Function_Params = Some( __Call_Function_Params(&mut __Parser) );

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);
}



fn __Call_Function_Params(__Parser: &mut Parser) -> Vec<String>{

    let mut __Parser = __Parser;
    let mut _Params = Vec::new();

    if __Parser.Current().Token_Type == TOKEN_TYPE::CLOSE_PARENTHESES{
        return _Params
    }

    loop{
        _Params.push(__Parser.Current().Value.clone());
        __Parser.Match(TOKEN_TYPE::VARIABLE);

        if __Parser.Current().Token_Type != TOKEN_TYPE::COMMA{
            return _Params
        }

        __Parser.Match(TOKEN_TYPE::COMMA);
    }
}



fn __Print_Statement(__Parser: &mut Parser, intent: String){

    

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::PRINT;

    __Parser.Match(TOKEN_TYPE::PRINT);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);
}



fn __Input_Statement(__Parser: &mut Parser, intent: String){

    

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::INPUT;

    __Parser.Match(TOKEN_TYPE::INPUT);

    __Parser.Match(TOKEN_TYPE::OPEN_PARENTHESES);

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::CLOSE_PARENTHESES);

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);
}



fn __ReAssign_Statement(__Parser: &mut Parser, intent: String){

    

    let mut __Parser = __Parser;
    let mut _Node = Trees::Statement_Syntax_Node::new();
    _Node.Kind = Trees::STATEMENT_KIND::RE_ASSIGN;

    _Node.Variable_Name = __Parser.Current().Value.clone();
    __Parser.Match(TOKEN_TYPE::VARIABLE);

    __Parser.Match(TOKEN_TYPE::ASSIGN);

    match __Parser.Current().Token_Type{

        TOKEN_TYPE::TRUE => {
            _Node.Bool_Value = Some(true);
            __Parser.Match(TOKEN_TYPE::TRUE);
        },

        TOKEN_TYPE::FALSE => {
            _Node.Bool_Value = Some(false);
            __Parser.Match(TOKEN_TYPE::FALSE);
        },

        TOKEN_TYPE::CHARACTER => _Node.String_Value = Some(__String_Expression(&mut __Parser)),

        TOKEN_TYPE::STRING_SEQUENCE => _Node.String_Value = Some( __String_Expression(&mut __Parser) ),

        TOKEN_TYPE::VARIABLE => _Node = __Variable_Expression(&mut __Parser, 0, _Node),

        _ => _Node.Double_Value = Some( __Expression(&mut __Parser) )
    }

    __Parser.Match(TOKEN_TYPE::SEMI_COLON);

    

    __Parser.Syntax_Tree.Statements.Add_Child(_Node);
}



fn __Variable_Expression(__Parser: &mut Parser, Number: usize, _Node: Trees::Statement_Syntax_Node) -> Trees::Statement_Syntax_Node{

    let mut __Parser = __Parser;
    let mut _Node = _Node;
    let mut Counter = 0;

    loop {
        let Token_Type = __Parser.Peek(Counter).Token_Type;

        if Token_Type == TOKEN_TYPE::SEMI_COLON || Token_Type == TOKEN_TYPE::EOF{
            break
        }

        else if Token_Type == TOKEN_TYPE::VARIABLE || Token_Type == TOKEN_TYPE::PLUS{
            Counter += 1;
        }

        else if Token_Type == TOKEN_TYPE::MINUS || Token_Type == TOKEN_TYPE::MOD || Token_Type == TOKEN_TYPE::MUL ||
            Token_Type == TOKEN_TYPE::DIV || Token_Type == TOKEN_TYPE::NUMBER{
            
            _Node.Double_Value = Some( __Expression(&mut __Parser) );
            return _Node
        }

        else if Token_Type == TOKEN_TYPE::STRING_SEQUENCE || Token_Type == TOKEN_TYPE::CHARACTER{
            _Node.String_Value = Some( __String_Expression(&mut __Parser) );
            return _Node
        }

        else {
            __Parser.Raise_Error();
        }
    }

    _Node.Variable_Value = Some( __Expression(&mut __Parser) );
    return _Node
}
