#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]
#![allow(unused_assignments)]





// Use Statemments Here
use crate::SyntaxTreeLib::SyntaxTree;
use crate::SyntaxTreeLib::Trees;
use crate::CodeGeneratorLib::CodeGenerator;
use crate::TokenTypeLib::TOKEN_TYPE;
use crate::CodeGeneratorLib::Environments;







pub fn GenerateCode(_CodeGenerator: &mut CodeGenerator, __Tree: &mut SyntaxTree){
    
    let mut _CodeGenerator = _CodeGenerator;
    let mut __Tree = __Tree;

    Function_Statements(&mut _CodeGenerator, &mut __Tree.Statements);

    Statements(&mut _CodeGenerator, &mut __Tree.Statements);
}



fn Function_Statements(_CodeGenerator: &mut CodeGenerator, _Statements: &mut Trees::Statements_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;
    let mut _Statements = _Statements;

    for _Statement in _Statements.Statements.iter_mut(){
        let mut _Statement = _Statement;

        match _Statement.Kind{
            Trees::STATEMENT_KIND::DEFINE_FUNCTION => Define_Function(&mut _CodeGenerator, &mut _Statement),
    
            _ => ()
        }
    }
}



fn Statements(_CodeGenerator: &mut CodeGenerator, _Statements: &mut Trees::Statements_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;
    let mut _Statements = _Statements;

    for _Statement in _Statements.Statements.iter_mut(){
        let mut _Statement = _Statement;
        Statement(&mut _CodeGenerator, &mut _Statement);
    }
}



fn Statement(_CodeGenerator: &mut CodeGenerator, _Statement: &mut Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;
    let mut _Statement = _Statement;

    match _Statement.Kind{
        
        Trees::STATEMENT_KIND::DEFINE_BOOL => Define_Bool(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::DEFINE_INT => Define_int(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::DEFINE_DOUBLE => Define_Double(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::DEFINE_CHAR => Define_Char(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::DEFINE_STRING => Define_String(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::DEFINE_IF => Define_If(&mut _CodeGenerator, &mut _Statement),

        Trees::STATEMENT_KIND::DEFINE_SWITCH => Define_Switch(&mut _CodeGenerator, &mut _Statement),

        Trees::STATEMENT_KIND::DEFINE_LOOP => Define_Loop(&mut _CodeGenerator, &mut _Statement),

        Trees::STATEMENT_KIND::CONTINUE => {
            _CodeGenerator.Writeln(format!("con:_"));
        },

        Trees::STATEMENT_KIND::BREAK => {
            _CodeGenerator.Writeln(format!("br:_"));
        },

        Trees::STATEMENT_KIND::RETURN => Return_Statement(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::INPUT => Input(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::PRINT => Print(&mut _CodeGenerator, &_Statement),

        Trees::STATEMENT_KIND::CALL_FUNCTION => Call_Function_Statement(&mut _CodeGenerator, &_Statement),

        _ => ()
    }
}



fn Define_Function(_CodeGenerator: &mut CodeGenerator, _Statement: &mut Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut _Function: &mut Trees::Function_Statement_Node = _Statement.Function.as_mut().unwrap();

    if _Function.Return_Type == None{
        _Function.Return_Type = Some(TOKEN_TYPE::VOID);
    }

    let mut Function = Environments::Function::new(
        _Function.Function_Name.clone(),
        format!("_{}", _CodeGenerator.Functions.len()),
        _Function.Return_Type.unwrap()
    );

    _CodeGenerator.Writeln(format!("fn:{}:{}", Function.Code_Name, _Function.Childs.len()));

    for Child in _Function.Childs.iter_mut(){

        if Child.Return_Type == None{
            Child.Return_Type = _Function.Return_Type.clone();
        }

        let mut Definition = Environments::Definition::new(format!("_{}", Function.Definitions.len()), Child.Return_Type.unwrap());
        
        _CodeGenerator.Writeln(format!("de:{}:{:?}:{}", Definition.Name, Child.Return_Type.unwrap(), Child.Variables.len()));

        for Variable in Child.Variables.iter(){

            let mut _Variable = Environments::Variable::new();
            _Variable.Name = Variable.Name.clone();
            _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
            _Variable.Type = Variable.Type;

            let mut _Param = Environments::Param::new();
            _Param.Name = Variable.Name.clone();
            _Param.Code_Name = _Variable.Code_Name.clone();
            _Param.Type = Variable.Type;

            _CodeGenerator.Writeln(format!("para:{}:{:?}", _Variable.Code_Name, _Variable.Type));

            _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);
            Definition.Params.push(_Param);
        }

        _CodeGenerator.Writeln(format!("de_stmts:("));
        Statements(&mut _CodeGenerator, &mut Child.Statements);
        _CodeGenerator.Writeln(format!("end_de_stmts:)"));

        Function.Definitions.insert(Definition.Name.clone(), Definition);
    }

    _CodeGenerator.Functions.insert(Function.Name.clone(), Function);
}



fn Define_Bool(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut _Variable = Environments::Variable::new();
    _Variable.Name = _Statement.Variable_Name.clone();
    _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
    _Variable.Type = TOKEN_TYPE::BOOL;

    _CodeGenerator.Writeln(format!("var:BOOL:{}:{}", _Variable.Code_Name, if _Statement.Bool_Value.unwrap() == true{
        "1"
    } else {"0"}));

    _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);
}


fn Define_int(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut _Variable = Environments::Variable::new();
    _Variable.Name = _Statement.Variable_Name.clone();
    _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
    _Variable.Type = TOKEN_TYPE::INT;

    _CodeGenerator.Writeln(format!("var:INT:{}:0", _Variable.Code_Name));

    let Code_Name = _Variable.Code_Name.clone();

    _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

    let Name = int_Value(&mut _CodeGenerator, &_Statement.int_Value.as_ref().unwrap());

    _CodeGenerator.Writeln(format!("add_v_v:{}:{}:{}", Code_Name, Code_Name, Name));
}



fn Call_Function(
    _CodeGenerator: &mut CodeGenerator,
    Definition_Name: String,
    Definition_Return_Type: TOKEN_TYPE,
    Function_Code_Name: String,
    _Value: &Trees::OperationNode
) -> String{

    let mut _CodeGenerator = _CodeGenerator;
    
    let mut _Variable = Environments::Variable::new();
    _Variable.Code_Name = format!("t_{}", _CodeGenerator.Variables.len());
    _Variable.Name = _Variable.Code_Name.clone();
    _Variable.Type = Definition_Return_Type.clone();

    let Name = _Variable.Code_Name.clone();
    let Var_Value = if _Variable.Type == TOKEN_TYPE::BOOL || _Variable.Type == TOKEN_TYPE::INT || _Variable.Type == TOKEN_TYPE::DOUBLE{
        "0"
    }else{
        "''"
    };

    _CodeGenerator.Writeln(format!(
        "var:{:?}:{}:{}",
        _Variable.Type,
        _Variable.Code_Name,
        Var_Value
    ));

    let Call = format!("call_v:{}:{}:{}:{}", _Variable.Code_Name, Function_Code_Name, Definition_Name, _Value.Function_Params.len());
    _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

    let mut Params = String::new();

    for Param in _Value.Function_Params.iter(){
        let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();
        Params += &":";
        Params += &_Variable.Code_Name;
    }

    _CodeGenerator.Writeln(format!("{}{}", Call, Params));
    return Name;
}



fn int_Value(_CodeGenerator: &mut CodeGenerator, _Value: &Trees::OperationNode) -> String{

    let mut _CodeGenerator = _CodeGenerator;

    if _Value.Type == Trees::OPERATION_TYPE::PARENTHESES_EXPRESSION{
        return int_Value(&mut _CodeGenerator, _Value.Left.as_ref().unwrap());
    }

    else if _Value.Type == Trees::OPERATION_TYPE::NUMBER{

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::DOUBLE;

        _CodeGenerator.Writeln(format!("var:DOUBLE:{}:{}", _Variable.Code_Name, _Value.Value));

        let Name = _Variable.Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);
        return Name
    }

    else if _Value.Type == Trees::OPERATION_TYPE::VARIABLE{

        let _Variable_Name = _CodeGenerator.Variables.get(&_Value.Value).unwrap().Code_Name.clone();

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::DOUBLE;
        
        _CodeGenerator.Writeln(format!("var:DOUBLE:{}:0", _Variable.Code_Name ));
        _CodeGenerator.Writeln(format!("add_v_v:{}:{}:{}", _Variable.Code_Name, _Variable.Code_Name, _Variable_Name));
        
        let Name = _Variable.Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);
        return Name
    }

    else if _Value.Type == Trees::OPERATION_TYPE::FUNCTION{

        let Function = _CodeGenerator.Functions.get(&_Value.Value).unwrap();
        let mut Found_Definition_Name = String::new();
        let mut Definition_Return_Type = TOKEN_TYPE::VOID;

        for (Definition_Name, Definition) in Function.Definitions.iter(){

            if vec![TOKEN_TYPE::INT, TOKEN_TYPE::DOUBLE].contains(&Definition.Return_Type){

                if Definition.Params.len() == _Value.Function_Params.len(){

                    let mut Exact = true;
    
                    for (i, Param) in _Value.Function_Params.iter().enumerate(){
    
                        let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();

                        if Definition.Params[i].Type != _Variable.Type{
                            Exact = false;
                            break
                        }
                    }
    
                    if Exact{
                        Found_Definition_Name = Definition_Name.clone();
                        Definition_Return_Type = Definition.Return_Type;
                        break
                    }
                }
            }
        }

        let Function_Name = Function.Code_Name.clone();

        return Call_Function(
            &mut _CodeGenerator,
            Found_Definition_Name,
            Definition_Return_Type,
            Function_Name,
            &_Value
        );
    }

    else if _Value.Type == Trees::OPERATION_TYPE::MINUS_OPERATION{

        let Name = int_Value(&mut _CodeGenerator, _Value.Left.as_ref().unwrap());
        _CodeGenerator.Writeln(format!("mul_v:{}:{}:{}", Name, Name, -1));
        return Name
    }

    else if _Value.Type == Trees::OPERATION_TYPE::MATH_OPERATIONS{

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::DOUBLE;

        _CodeGenerator.Writeln(format!("var:DOUBLE:{}:0", _Variable.Code_Name ));

        let Code_Name = _Variable.Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

        let Left = int_Value(&mut _CodeGenerator, _Value.Left.as_ref().unwrap());
        let Right = int_Value(&mut _CodeGenerator, _Value.Right.as_ref().unwrap());

        if _Value.Operation.unwrap() == TOKEN_TYPE::PLUS{
            _CodeGenerator.Writeln(format!("add_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Value.Operation.unwrap() == TOKEN_TYPE::MINUS{
            _CodeGenerator.Writeln(format!("sub_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Value.Operation.unwrap() == TOKEN_TYPE::MUL{
            _CodeGenerator.Writeln(format!("mul_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Value.Operation.unwrap() == TOKEN_TYPE::DIV{
            _CodeGenerator.Writeln(format!("div_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Value.Operation.unwrap() == TOKEN_TYPE::MOD{
            _CodeGenerator.Writeln(format!("mod_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        
        return Code_Name
    }

    return String::new()
}



fn Define_Double(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut _Variable = Environments::Variable::new();
    _Variable.Name = _Statement.Variable_Name.clone();
    _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
    _Variable.Type = TOKEN_TYPE::DOUBLE;

    _CodeGenerator.Writeln(format!("var:DOUBLE:{}:0", _Variable.Code_Name));

    let Code_Name = _Variable.Code_Name.clone();

    _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

    let Name = int_Value(&mut _CodeGenerator, &_Statement.Double_Value.as_ref().unwrap());

    _CodeGenerator.Writeln(format!("add_v_v:{}:{}:{}", Code_Name, Code_Name, Name));
}



fn Define_Char(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut _Variable = Environments::Variable::new();
    _Variable.Name = _Statement.Variable_Name.clone();
    _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
    _Variable.Type = TOKEN_TYPE::CHAR;

    _CodeGenerator.Writeln(format!(
        "var:CHAR:{}:'{}'",
        _Variable.Code_Name,
        &_Statement.Char_Value.as_ref().unwrap()
    ));

    _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);
}



fn Define_String(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut _Variable = Environments::Variable::new();
    _Variable.Name = _Statement.Variable_Name.clone();
    _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
    _Variable.Type = TOKEN_TYPE::STRING;

    _CodeGenerator.Writeln(format!("var:STRING:{}:''", _Variable.Code_Name));

    let Code_Name = _Variable.Code_Name.clone();

    _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

    String_Value(&mut _CodeGenerator, &Code_Name, &_Statement.String_Value.as_ref().unwrap());
}



fn String_Value(_CodeGenerator: &mut CodeGenerator, _Variable_Name: &String, _Node: &Trees::StringOperationNode){

    let mut _CodeGenerator = _CodeGenerator;

    if _Node.Type == Trees::OPERATION_TYPE::CHARACTER || _Node.Type == Trees::OPERATION_TYPE::STRING_SEQUENCE{
        _CodeGenerator.Writeln(format!("add_s_v:{}:{}:'{}'", _Variable_Name, _Variable_Name, _Node.Value));
    }

    else if _Node.Type == Trees::OPERATION_TYPE::VARIABLE{

        let _Variable_Code_Name = _CodeGenerator.Variables.get(&_Node.Value).unwrap().Code_Name.clone();

        _CodeGenerator.Writeln(format!("add_s_v_v:{}:{}:{}", _Variable_Name, _Variable_Name, _Variable_Code_Name));
    }

    else if _Node.Type == Trees::OPERATION_TYPE::FUNCTION{

        let Function = _CodeGenerator.Functions.get(&_Node.Value).unwrap();
        let mut Found_Definition_Name = String::new();
        let mut Definition_Return_Type = TOKEN_TYPE::VOID;

        for (Definition_Name, Definition) in Function.Definitions.iter(){

            if vec![TOKEN_TYPE::STRING, TOKEN_TYPE::CHAR].contains(&Definition.Return_Type){

                if Definition.Params.len() == _Node.Function_Params.len(){

                    let mut Exact = true;
    
                    for (i, Param) in _Node.Function_Params.iter().enumerate(){
    
                        let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();

                        if Definition.Params[i].Type != _Variable.Type{
                            Exact = false;
                            break
                        }
                    }
    
                    if Exact{
                        Found_Definition_Name = Definition_Name.clone();
                        Definition_Return_Type = Definition.Return_Type.clone();
                        break
                    }
                }
            }
        }

        let Function_Name = Function.Code_Name.clone();
    
        let mut _Variable = Environments::Variable::new();
        _Variable.Code_Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Name = _Variable.Code_Name.clone();
        _Variable.Type = Definition_Return_Type.clone();

        let Name = _Variable.Code_Name.clone();
        let Var_Value = if _Variable.Type == TOKEN_TYPE::BOOL || _Variable.Type == TOKEN_TYPE::INT || _Variable.Type == TOKEN_TYPE::DOUBLE{
            "0"
        }else{
            "''"
        };

        _CodeGenerator.Writeln(format!(
            "var:{:?}:{}:{}",
            _Variable.Type,
            _Variable.Code_Name,
            Var_Value
        ));

        let Temp_Variable_Name = _Variable.Code_Name.clone();

        let Call = format!("call_v:{}:{}:{}:{}", _Variable.Code_Name, Function_Name, Found_Definition_Name, _Node.Function_Params.len());
        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

        let mut Params = String::new();

        for Param in _Node.Function_Params.iter(){
            let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();
            Params += &":";
            Params += &_Variable.Code_Name;
        }

        _CodeGenerator.Writeln(format!("{}{}", Call, Params));

        _CodeGenerator.Writeln(format!("add_s_v_v:{}:{}:{}", _Variable_Name, _Variable_Name, Temp_Variable_Name));
    }

    if _Node.Left != None{
        String_Value(&mut _CodeGenerator, _Variable_Name, &_Node.Left.as_ref().unwrap());
    }
}



fn Define_If(_CodeGenerator: &mut CodeGenerator, _Statement: &mut Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let mut IF: &mut Trees::If_Statement_Syntax_Node = _Statement.If.as_mut().unwrap();

    let If_Name: String = format!("if_{}", _CodeGenerator.IF.len());
    _CodeGenerator.IF.push(If_Name.clone());

    _CodeGenerator.Writeln(format!("if:{}:(", If_Name));
    
    for Child in IF.Childs.iter_mut(){

        if Child.Expression != None{

            let _Temp_Variable = If_Expression(&mut _CodeGenerator, &Child.Expression.as_ref().unwrap());
            _CodeGenerator.Writeln(format!("if_exp:{}", _Temp_Variable));
        }
        else{
            _CodeGenerator.Writeln(format!("if_exp:true"));
        }

        let If_Definition_Name: String = format!("if_{}", _CodeGenerator.IF.len());

        _CodeGenerator.IF.push(If_Definition_Name.clone());

        _CodeGenerator.Writeln(format!("if_stmts:{}:(", If_Definition_Name));
        Statements(&mut _CodeGenerator, &mut Child.Statements);
        _CodeGenerator.Writeln(format!("end_if_stmts:{}:)", If_Definition_Name));
    }

    _CodeGenerator.Writeln(format!("end_if:{}:)", If_Name));
}



fn If_Expression(_CodeGenerator: &mut CodeGenerator, _Node: &Trees::OperationNode) -> String{

    let mut _CodeGenerator = _CodeGenerator;

    if _Node.Type == Trees::OPERATION_TYPE::PARENTHESES_EXPRESSION{
        return If_Expression(&mut _CodeGenerator, _Node.Left.as_ref().unwrap());
    }

    else if _Node.Type == Trees::OPERATION_TYPE::NUMBER{

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::DOUBLE;

        _CodeGenerator.Writeln(format!("var:DOUBLE:{}:{}", _Variable.Code_Name, _Node.Value));

        let Name = _Variable.Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

        return Name
    }

    else if _Node.Type == Trees::OPERATION_TYPE::VARIABLE{

        let _Variable_Name = _CodeGenerator.Variables.get(&_Node.Value).unwrap().Code_Name.clone();

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::DOUBLE;
        
        _CodeGenerator.Writeln(format!("var:DOUBLE:{}:0", _Variable.Code_Name ));
        _CodeGenerator.Writeln(format!("add_v_v:{}:{}:{}", _Variable.Code_Name, _Variable.Code_Name, _Variable_Name));
        
        let Name = _Variable.Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);
        return Name
    }

    else if _Node.Type == Trees::OPERATION_TYPE::FUNCTION{

        let Function = _CodeGenerator.Functions.get(&_Node.Value).unwrap();
        let mut Found_Definition_Name = String::new();
        let mut Definition_Return_Type = TOKEN_TYPE::VOID;

        for (Definition_Name, Definition) in Function.Definitions.iter(){

            if vec![TOKEN_TYPE::INT, TOKEN_TYPE::DOUBLE].contains(&Definition.Return_Type){

                if Definition.Params.len() == _Node.Function_Params.len(){

                    let mut Exact = true;
    
                    for (i, Param) in _Node.Function_Params.iter().enumerate(){
    
                        let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();

                        if Definition.Params[i].Type != _Variable.Type{
                            Exact = false;
                            break
                        }
                    }
    
                    if Exact{
                        Found_Definition_Name = Definition_Name.clone();
                        Definition_Return_Type = Definition.Return_Type;
                        break
                    }
                }
            }
        }

        let Function_Name = Function.Code_Name.clone();

        return Call_Function(
            &mut _CodeGenerator,
            Found_Definition_Name,
            Definition_Return_Type,
            Function_Name,
            &_Node
        );
    }

    else if _Node.Type == Trees::OPERATION_TYPE::MINUS_OPERATION{

        let Name = If_Expression(&mut _CodeGenerator, _Node.Left.as_ref().unwrap());
        _CodeGenerator.Writeln(format!("mul_v:{}:{}:{}", Name, Name, -1));
        return Name
    }

    else if _Node.Type == Trees::OPERATION_TYPE::MATH_OPERATIONS{

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::DOUBLE;

        let Code_Name = _Variable.Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

        _CodeGenerator.Writeln(format!("var:DOUBLE:{}:0", Code_Name ));

        let Left = If_Expression(&mut _CodeGenerator, _Node.Left.as_ref().unwrap());
        let Right = If_Expression(&mut _CodeGenerator, _Node.Right.as_ref().unwrap());

        if _Node.Operation.unwrap() == TOKEN_TYPE::PLUS{
            _CodeGenerator.Writeln(format!("add_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Node.Operation.unwrap() == TOKEN_TYPE::MINUS{
            _CodeGenerator.Writeln(format!("sub_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Node.Operation.unwrap() == TOKEN_TYPE::MUL{
            _CodeGenerator.Writeln(format!("mul_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Node.Operation.unwrap() == TOKEN_TYPE::DIV{
            _CodeGenerator.Writeln(format!("div_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        else if _Node.Operation.unwrap() == TOKEN_TYPE::MOD{
            _CodeGenerator.Writeln(format!("mod_v_v:{}:{}:{}", Code_Name, Left, Right));
        }
        
        return Code_Name
    }

    else if _Node.Type == Trees::OPERATION_TYPE::COMPARE_OPERATIONS{

        let mut _Variable = Environments::Variable::new();
        _Variable.Name = format!("t_{}", _CodeGenerator.Variables.len());
        _Variable.Code_Name = _Variable.Name.clone();
        _Variable.Type = TOKEN_TYPE::BOOL;
        
        let Code_Name = _Variable.Code_Name.clone();
        
        _CodeGenerator.Writeln(format!("var:BOOL:{}:0", _Variable.Code_Name ));

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

        let Left = If_Expression(&mut _CodeGenerator, _Node.Left.as_ref().unwrap());
        let Right = If_Expression(&mut _CodeGenerator, _Node.Right.as_ref().unwrap());


        if _Node.Operation.unwrap() == TOKEN_TYPE::GREATER_THAN{
            _CodeGenerator.Writeln(format!("gt_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::GREATER_THAN_OR_EQUAL{
            _CodeGenerator.Writeln(format!("gteq_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::LESS_THAN{
            _CodeGenerator.Writeln(format!("lt_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::LESS_THAN_OR_EQUAL{
            _CodeGenerator.Writeln(format!("lteq_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::EQUAL{
            _CodeGenerator.Writeln(format!("eq_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::NOT_EQUAL{
            _CodeGenerator.Writeln(format!("neq_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::AND{
            _CodeGenerator.Writeln(format!("and_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        else if _Node.Operation.unwrap() == TOKEN_TYPE::OR{
            _CodeGenerator.Writeln(format!("or_v_v:{}:{}:{}", Code_Name, Left, Right));
        }

        return Code_Name
    }

    else if _Node.Type == Trees::OPERATION_TYPE::COMPARE_EXPRESSION{
        return If_Expression(&mut _CodeGenerator, _Node.Left.as_ref().unwrap());
    }

    return String::new()
}



fn Define_Switch(_CodeGenerator: &mut CodeGenerator, _Statement: &mut Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    let Name = _CodeGenerator.Variables.get(&_Statement.Variable_Name).unwrap().Code_Name.clone();

    let Switch_Name: String = format!("sw_{}", _CodeGenerator.SWITCH.len());
    _CodeGenerator.SWITCH.push(Switch_Name.clone());

    _CodeGenerator.Writeln(format!("sw:{}:{}", Name, Switch_Name));

    for Child in _Statement.Switch.as_mut().unwrap().Childs.iter_mut(){

        let mut _Statements = &mut Child.Statements;

        if Child.Variables.len() > 0{
            
            for Variable in Child.Variables.iter(){
                
                if Variable.Token_Type == TOKEN_TYPE::NUMBER{
                    _CodeGenerator.Writeln(format!("sw_i:{}", Variable.Value));
                }

                else if Variable.Token_Type == TOKEN_TYPE::CHARACTER{
                    _CodeGenerator.Writeln(format!("sw_c:'{}'", Variable.Value));
                }

                else if Variable.Token_Type == TOKEN_TYPE::STRING_SEQUENCE{
                    _CodeGenerator.Writeln(format!("sw_s:'{}'", Variable.Value));
                }

                let Switch_Definition_Name = format!("sw_{}", _CodeGenerator.SWITCH.len());

                _CodeGenerator.Writeln(format!("sw_stmts:{}:(", Switch_Definition_Name));
                Statements(&mut _CodeGenerator, &mut _Statements);
                _CodeGenerator.Writeln(format!("end_sw_stmts:{}:)", Switch_Definition_Name));

                _CodeGenerator.SWITCH.push(Switch_Definition_Name);
            }
        }

        else{
            _CodeGenerator.Writeln(format!("sw_d:true"));
            
            let Switch_Definition_Name = format!("sw_{}", _CodeGenerator.SWITCH.len());

            _CodeGenerator.Writeln(format!("sw_stmts:{}:(", Switch_Definition_Name));
            Statements(&mut _CodeGenerator, &mut _Statements);
            _CodeGenerator.Writeln(format!("end_sw_stmts:{}:)", Switch_Definition_Name));

            _CodeGenerator.SWITCH.push(Switch_Definition_Name);
        }
    }

    _CodeGenerator.Writeln(format!("end_sw:{}", Switch_Name));
}



fn Define_Loop(_CodeGenerator: &mut CodeGenerator, _Statement: &mut Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;
    let mut Loop: &mut Trees::Loop_Statement_Syntax_Node = &mut _Statement.Loop.as_mut().unwrap();

    let Loop_Name = format!("loop_{}", _CodeGenerator.LOOP.len());

    _CodeGenerator.Writeln(format!("loop:{}:(", Loop_Name));

    if Loop.Expression == None{
        _CodeGenerator.Writeln(format!("loop_init:true:true"));
        _CodeGenerator.Writeln(format!("loop_exp:true:true"));
        _CodeGenerator.Writeln(format!("loop_step:true"));
    }

    else if Loop.Expression.as_ref().unwrap().Loop_Type == Trees::OPERATION_TYPE::WHILE_LOOP{
        let Name = If_Expression(
            &mut _CodeGenerator, &Loop.Expression.as_ref().unwrap().First_Expression.as_ref().unwrap()
        );

        _CodeGenerator.Writeln(format!("loop_init:true:true"));
        _CodeGenerator.Writeln(format!("loop_exp:true:{}", Name));
        _CodeGenerator.Writeln(format!("loop_step:true"));
    }

    else{
        let mut _Variable = Environments::Variable::new();
        _Variable.Name = Loop.Expression.as_ref().unwrap().Variable.clone();
        _Variable.Code_Name = format!("_{}", _CodeGenerator.Variables.len());
        _Variable.Type = TOKEN_TYPE::INT;

        _CodeGenerator.Writeln(format!("var:INT:{}:0", _Variable.Code_Name));

        let init_Name = _Variable.Code_Name.clone();

        _CodeGenerator.Variables.insert(_Variable.Name.clone(), _Variable);

        let Name = If_Expression(
            &mut _CodeGenerator, &Loop.Expression.as_ref().unwrap().First_Expression.as_ref().unwrap()
        );

        _CodeGenerator.Writeln(format!("loop_init:{}:{}", init_Name, Name));

        if Loop.Expression.as_ref().unwrap().Second_Expression != None{
            let Name = If_Expression(
                &mut _CodeGenerator, &Loop.Expression.as_ref().unwrap().Second_Expression.as_ref().unwrap()
            );
    
            _CodeGenerator.Writeln(format!("loop_exp:{}:{}", init_Name, Name));

            if Loop.Expression.as_ref().unwrap().Third_Expression != None{
                let Name = If_Expression(
                    &mut _CodeGenerator, &Loop.Expression.as_ref().unwrap().Third_Expression.as_ref().unwrap()
                );
        
                _CodeGenerator.Writeln(format!("loop_step:{}", Name));
            }
            else{
                _CodeGenerator.Writeln(format!("loop_step:true"));
            }
        }

        else{
            _CodeGenerator.Writeln(format!("loop_exp:true:true"));
            _CodeGenerator.Writeln(format!("loop_step:true"));
        }
    }

    _CodeGenerator.Writeln(format!("loop_stmts:{}:(", Loop_Name));
    Statements(&mut _CodeGenerator, &mut Loop.Statements);
    _CodeGenerator.Writeln(format!("end_loop_stmts:{}:)", Loop_Name));

    _CodeGenerator.Writeln(format!("end_loop:{}:)", Loop_Name));
    
    _CodeGenerator.LOOP.push(Loop_Name);
}


fn Return_Statement(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    if _Statement.Variable_Name.eq(""){
        _CodeGenerator.Writeln(format!("re:none"));
    }
    else{
        let Name = _CodeGenerator.Variables.get(&_Statement.Variable_Name).unwrap().Code_Name.clone();
        _CodeGenerator.Writeln(format!("re:{}", Name));
    }
}



fn Call_Function_Statement(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;
    
    let Function = _CodeGenerator.Functions.get(&_Statement.Variable_Name).unwrap();
    let mut Found_Definition_Name = String::new();
    let mut Definition_Return_Type = TOKEN_TYPE::VOID;

    for (Definition_Name, Definition) in Function.Definitions.iter(){

        if vec![
            TOKEN_TYPE::STRING,
            TOKEN_TYPE::CHAR,
            TOKEN_TYPE::DOUBLE,
            TOKEN_TYPE::INT,
            TOKEN_TYPE::BOOL
            ].contains(&Definition.Return_Type){

            if Definition.Params.len() == _Statement.Function_Params.as_ref().unwrap().len(){

                let mut Exact = true;

                for (i, Param) in _Statement.Function_Params.as_ref().unwrap().iter().enumerate(){

                    let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();

                    if Definition.Params[i].Type != _Variable.Type{
                        Exact = false;
                        break
                    }
                }

                if Exact{
                    Found_Definition_Name = Definition_Name.clone();
                    Definition_Return_Type = Definition.Return_Type.clone();
                    break
                }
            }
        }
    }

    let Function_Name = Function.Code_Name.clone();

    let Call = format!("call:{}:{}:{}", Function_Name, Found_Definition_Name, _Statement.Function_Params.as_ref().unwrap().len());

    let mut Params = String::new();

    for Param in _Statement.Function_Params.as_ref().unwrap().iter(){
        let _Variable: &Environments::Variable = _CodeGenerator.Variables.get(Param).unwrap();
        Params += &":";
        Params += &_Variable.Code_Name;
    }

    _CodeGenerator.Writeln(format!("{}{}", Call, Params));
}



fn Print(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    _CodeGenerator.Writeln(format!(
        "pr:{}",
        _CodeGenerator.Variables.get(&_Statement.Variable_Name).unwrap().Code_Name.clone()
    ));
}



fn Input(_CodeGenerator: &mut CodeGenerator, _Statement: &Trees::Statement_Syntax_Node){

    let mut _CodeGenerator = _CodeGenerator;

    _CodeGenerator.Writeln(format!(
        "in:{}",
        _CodeGenerator.Variables.get(&_Statement.Variable_Name).unwrap().Code_Name.clone()
    ));
}
