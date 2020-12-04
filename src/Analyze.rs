#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]




// Use Statements Here
use crate::SyntaxTreeLib::SyntaxTree;
use crate::SyntaxTreeLib::Trees;
use crate::SymanticAnalyzerLib::Analyzer;
use crate::SymanticAnalyzerLib::Environments;
use crate::TokenTypeLib::TOKEN_TYPE;





pub fn Analyze(__Analyzer: &mut Analyzer, __Tree: &SyntaxTree){

    let mut __Analyzer = __Analyzer;

    Passes::First_Pass::Analyze(&mut __Analyzer, &__Tree);

    Passes::Second_Pass::Analyze(&mut __Analyzer, &__Tree);
}


mod Passes{

    pub mod First_Pass{

        use crate::SyntaxTreeLib::SyntaxTree;
        use crate::SyntaxTreeLib::Trees;
        use crate::SymanticAnalyzerLib::Analyzer;
        use crate::SymanticAnalyzerLib::Environments;
        use crate::TokenTypeLib::TOKEN_TYPE;


        pub fn Analyze(__Analyzer: &mut Analyzer, __Tree: &SyntaxTree){

            let mut __Analyzer = __Analyzer;

            Statements(&mut __Analyzer, &__Tree.Statements);
        }


        fn Statements(__Analyzer: &mut Analyzer, __Statements: &Trees::Statements_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            for _Statement in __Statements.Statements.iter(){
                Statement(&mut __Analyzer, &_Statement);
            }
        }


        fn Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            match _Statement.Kind{

                Trees::STATEMENT_KIND::DEFINE_FUNCTION => Define_Function(&mut __Analyzer, _Statement.Function.as_ref()),

                _ => ()
            }
        }


        fn Define_Function(__Analyzer: &mut Analyzer, _Function: Option<&Trees::Function_Statement_Node>){

            let mut __Analyzer = __Analyzer;

            let _Function = _Function.unwrap();

            if __Analyzer.Is_Function_Defined(&_Function.Function_Name){
                __Analyzer.Raise_Error(format!("Function `{}` Already Defined", _Function.Function_Name));
            }

            let mut Function = Environments::Function::new();

            Function.Name = _Function.Function_Name.clone();
            Function.Global_Return_Type = if _Function.Return_Type == None {TOKEN_TYPE::VOID} else {
                _Function.Return_Type.unwrap()
            };

            Define_Function_Difinition(&mut __Analyzer, &mut Function, &_Function);

            __Analyzer.Register_Function(Function);
        }


        fn Define_Function_Difinition(__Analyzer: &mut Analyzer, Function: &mut Environments::Function, _Function: &Trees::Function_Statement_Node){

            let mut __Analyzer = __Analyzer;
            let mut Function = Function;

            for Child in _Function.Childs.iter(){

                let _Return_Type = if Child.Return_Type == None {Function.Global_Return_Type} else {
                    Child.Return_Type.unwrap()
                };

                if Function.Is_Definition_Exists(_Return_Type, &Child.Variables){
                    __Analyzer.Raise_Error(
                        format!("Two Definitions Found in Function `{}`", Function.Name)
                    );
                }

                let mut _Function_Variables: Vec<Environments::Function_Param> = Vec::new();

                for Variable in Child.Variables.iter(){
                    let mut Param = Environments::Function_Param::new();
                    Param.Name = Variable.Name.clone();
                    Param.Type = Variable.Type;
                    _Function_Variables.push(Param);
                }

                let mut Definition = Environments::Definition::new();
                Definition.Params = _Function_Variables;
                Definition.Return_Type = _Return_Type;

                Function.Definitions.push(Definition);
            }
        }
    }





    pub mod Second_Pass{

        use crate::SyntaxTreeLib::SyntaxTree;
        use crate::SyntaxTreeLib::Trees;
        use crate::SymanticAnalyzerLib::Analyzer;
        use crate::SymanticAnalyzerLib::Environments;
        use crate::TokenTypeLib::TOKEN_TYPE;

        pub fn Analyze(__Analyzer: &mut Analyzer, __Tree: &SyntaxTree){
            
            let mut __Analyzer = __Analyzer;

            Statements(&mut __Analyzer, &__Tree.Statements);
        }


        fn Statements(__Analyzer: &mut Analyzer, __Statements: &Trees::Statements_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            for _Statement in __Statements.Statements.iter(){
                Statement(&mut __Analyzer, &_Statement);
            }
        }


        fn Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            match _Statement.Kind{

                Trees::STATEMENT_KIND::DEFINE_BOOL => Define_Bool(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_INT => Define_int(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_DOUBLE => Define_Double(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_CHAR => Define_Char(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_STRING => Define_String(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_IF => Define_If(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_SWITCH => Define_Switch(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_LOOP => Define_Loop(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::BREAK => Break_Statement(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::CONTINUE => Continue_Statement(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::CALL_FUNCTION => Call_Function(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::DEFINE_FUNCTION => Define_Function(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::RETURN => Return_Statement(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::PRINT => Print_Statement(&mut __Analyzer, &_Statement),

                Trees::STATEMENT_KIND::INPUT => Input_Statement(&mut __Analyzer, &_Statement),

                _ => ()
            }
        }


        fn Define_Bool(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            if __Analyzer.Current_Environment.Is_Variable_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!("Variable `{}` Already Defined", _Statement.Variable_Name));
            }

            let mut _Variable = Environments::Variable::new();

            _Variable.Type = Some(TOKEN_TYPE::BOOL);
            _Variable.Name = _Statement.Variable_Name.clone();
            _Variable.Bool_Value = _Statement.Bool_Value;

            __Analyzer.Current_Environment.Register_Variable(_Variable);
        }


        fn Define_int(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            if __Analyzer.Current_Environment.Is_Variable_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!("Variable `{}` Already Defined", _Statement.Variable_Name));
            }

            let mut _Variable = Environments::Variable::new();

            _Variable.Type = Some(TOKEN_TYPE::INT);
            _Variable.Name = _Statement.Variable_Name.clone();
            Check_int_Value(&mut __Analyzer, _Statement.int_Value.as_ref().unwrap());
            _Variable.int_Value = _Statement.int_Value.clone();

            __Analyzer.Current_Environment.Register_Variable(_Variable);
        }


        fn Check_int_Value(__Analyzer: &mut Analyzer, _Value: &Trees::OperationNode){

            let mut __Analyzer = __Analyzer;

            if _Value.Type == Trees::OPERATION_TYPE::PARENTHESES_EXPRESSION{
                Check_int_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::NUMBER{
                return 
            }

            else if _Value.Type == Trees::OPERATION_TYPE::VARIABLE{

                if !__Analyzer.Is_Variable_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", _Value.Value));
                }

                let _Variable = __Analyzer.Get_Variable(&_Value.Value);

                match _Variable.Type.as_ref().unwrap(){

                    TOKEN_TYPE::INT | TOKEN_TYPE::DOUBLE => (),

                    _ => __Analyzer.Raise_Error(format!("Variable `{}` is Not Of Type int Or Double", _Value.Value))
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::FUNCTION{

                if !__Analyzer.Is_Function_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Function `{}` Is Undefined", _Value.Value));
                }

                for Param in _Value.Function_Params.iter(){
                    if !__Analyzer.Is_Variable_Defined(&Param){
                        __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", Param));
                    }
                }

                if !__Analyzer.Check_Params_Count(&_Value.Value, &_Value.Function_Params, vec![TOKEN_TYPE::INT, TOKEN_TYPE::DOUBLE]){
                    __Analyzer.Raise_Error(format!(
                        "Function `{}` Has No Definition With This Params \n{:#?}", _Value.Value, _Value.Function_Params
                    ));
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::MINUS_OPERATION{
                Check_int_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::MATH_OPERATIONS{
                Check_int_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
                Check_int_Value(&mut __Analyzer, _Value.Right.as_ref().unwrap());
            }
        }


        fn Define_Double(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            if __Analyzer.Current_Environment.Is_Variable_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!("Variable `{}` Already Defined", _Statement.Variable_Name));
            }

            let mut _Variable = Environments::Variable::new();

            _Variable.Type = Some(TOKEN_TYPE::DOUBLE);
            _Variable.Name = _Statement.Variable_Name.clone();
            Check_Double_Value(&mut __Analyzer, _Statement.Double_Value.as_ref().unwrap());
            _Variable.Double_Value = _Statement.Double_Value.clone();

            __Analyzer.Current_Environment.Register_Variable(_Variable);
        }


        fn Check_Double_Value(__Analyzer: &mut Analyzer, _Value: &Trees::OperationNode){

            let mut __Analyzer = __Analyzer;

            if _Value.Type == Trees::OPERATION_TYPE::PARENTHESES_EXPRESSION{
                Check_Double_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::NUMBER{
                return 
            }

            else if _Value.Type == Trees::OPERATION_TYPE::VARIABLE{
                if !__Analyzer.Is_Variable_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", _Value.Value));
                }

                let _Variable = __Analyzer.Get_Variable(&_Value.Value);

                match _Variable.Type.as_ref().unwrap(){

                    TOKEN_TYPE::INT | TOKEN_TYPE::DOUBLE => (),

                    _ => __Analyzer.Raise_Error(format!("Variable `{}` is Not Of Type int Or Double", _Value.Value))
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::FUNCTION{
                if !__Analyzer.Is_Function_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Function `{}` Is Undefined", _Value.Value));
                }

                for Param in _Value.Function_Params.iter(){
                    if !__Analyzer.Is_Variable_Defined(&Param){
                        __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", Param));
                    }
                }

                if !__Analyzer.Check_Params_Count(&_Value.Value, &_Value.Function_Params, vec![TOKEN_TYPE::INT, TOKEN_TYPE::DOUBLE]){
                    __Analyzer.Raise_Error(format!(
                        "Function `{}` Has No Definition With This Params \n{:#?}", _Value.Value, _Value.Function_Params
                    ));
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::MINUS_OPERATION{
                Check_Double_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::MATH_OPERATIONS{
                Check_Double_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
                Check_Double_Value(&mut __Analyzer, _Value.Right.as_ref().unwrap());
            }
        }


        fn Define_Char(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            if __Analyzer.Current_Environment.Is_Variable_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!("Variable `{}` Already Defined", _Statement.Variable_Name));
            }

            let mut _Variable = Environments::Variable::new();

            _Variable.Type = Some(TOKEN_TYPE::CHAR);
            _Variable.Name = _Statement.Variable_Name.clone();
            _Variable.Char_Value = _Statement.Char_Value.clone();

            __Analyzer.Current_Environment.Register_Variable(_Variable);
        }


        fn Define_String(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){            

            let mut __Analyzer = __Analyzer;

            if __Analyzer.Current_Environment.Is_Variable_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!("Variable `{}` Already Defined", _Statement.Variable_Name));
            }

            let mut _Variable = Environments::Variable::new();

            _Variable.Type = Some(TOKEN_TYPE::STRING);
            _Variable.Name = _Statement.Variable_Name.clone();
            Check_String_Value(&mut __Analyzer, _Statement.String_Value.as_ref().unwrap());
            _Variable.String_Value = _Statement.String_Value.clone();

            __Analyzer.Current_Environment.Register_Variable(_Variable);
        }


        fn Check_String_Value(__Analyzer: &mut Analyzer, _Value: &Trees::StringOperationNode){

            let mut __Analyzer = __Analyzer;

            if _Value.Type == Trees::OPERATION_TYPE::STRING_SEQUENCE{
                
            }

            else if _Value.Type == Trees::OPERATION_TYPE::CHARACTER{
                
            }

            else if _Value.Type == Trees::OPERATION_TYPE::VARIABLE{

                if !__Analyzer.Is_Variable_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", _Value.Value));
                }

                let _Variable = __Analyzer.Get_Variable(&_Value.Value);

                match _Variable.Type.as_ref().unwrap(){

                    TOKEN_TYPE::STRING => (),

                    _ => __Analyzer.Raise_Error(format!("Variable `{}` is Not Of Type String", _Value.Value))
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::FUNCTION{
                
                if !__Analyzer.Is_Function_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Function `{}` Is Undefined", _Value.Value));
                }

                for Param in _Value.Function_Params.iter(){
                    if !__Analyzer.Is_Variable_Defined(&Param){
                        __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", Param));
                    }
                }

                if !__Analyzer.Check_Params_Count(&_Value.Value, &_Value.Function_Params, vec![TOKEN_TYPE::STRING]){
                    __Analyzer.Raise_Error(format!(
                        "Function `{}` Has No Definition With This Params \n{:#?}", _Value.Value, _Value.Function_Params
                    ));
                }
            }

            if _Value.Left != None{
                Check_String_Value(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }
        }


        fn Define_If(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            for Child in _Statement.If.as_ref().unwrap().Childs.iter(){

                if Child.Expression != None{
                    Handle_If_Expression(&mut __Analyzer, Child.Expression.as_ref().unwrap());
                }

                __Analyzer.Environment_Stack.push(__Analyzer.Current_Environment.clone());
                __Analyzer.Current_Environment = Environments::Environment::new(Environments::ENVIRNMENT::IF);

                Statements(&mut __Analyzer, &Child.Statements);

                __Analyzer.Current_Environment = __Analyzer.Environment_Stack.pop().unwrap();
            }
        }


        fn Handle_If_Expression(__Analyzer: &mut Analyzer, _Value: &Trees::OperationNode){

            let mut __Analyzer = __Analyzer;

            if _Value.Type == Trees::OPERATION_TYPE::PARENTHESES_EXPRESSION{
                Handle_If_Expression(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::NUMBER{
                return 
            }

            else if _Value.Type == Trees::OPERATION_TYPE::VARIABLE{
                if !__Analyzer.Is_Variable_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", _Value.Value));
                }

                let _Variable = __Analyzer.Get_Variable(&_Value.Value);

                match _Variable.Type.as_ref().unwrap(){

                    TOKEN_TYPE::INT | TOKEN_TYPE::DOUBLE => (),

                    _ => __Analyzer.Raise_Error(format!("Variable `{}` is Not Of Type int Or Double", _Value.Value))
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::FUNCTION{

                if !__Analyzer.Is_Function_Defined(&_Value.Value){
                    __Analyzer.Raise_Error(format!("Function `{}` Is Undefined", _Value.Value));
                }

                for Param in _Value.Function_Params.iter(){
                    if !__Analyzer.Is_Variable_Defined(&Param){
                        __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", Param));
                    }
                }

                if !__Analyzer.Check_Params_Count(&_Value.Value, &_Value.Function_Params, vec![TOKEN_TYPE::INT, TOKEN_TYPE::DOUBLE]){
                    __Analyzer.Raise_Error(format!(
                        "Function `{}` Has No Definition With This Params \n{:#?}", _Value.Value, _Value.Function_Params
                    ));
                }
            }

            else if _Value.Type == Trees::OPERATION_TYPE::MINUS_OPERATION{
                Handle_If_Expression(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::MATH_OPERATIONS{
                Handle_If_Expression(&mut __Analyzer, _Value.Left.as_ref().unwrap());
                Handle_If_Expression(&mut __Analyzer, _Value.Right.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::COMPARE_OPERATIONS{
                Handle_If_Expression(&mut __Analyzer, _Value.Left.as_ref().unwrap());
                Handle_If_Expression(&mut __Analyzer, _Value.Right.as_ref().unwrap());
            }

            else if _Value.Type == Trees::OPERATION_TYPE::COMPARE_EXPRESSION{
                Handle_If_Expression(&mut __Analyzer, _Value.Left.as_ref().unwrap());
            }
        }


        fn Define_Switch(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            if !__Analyzer.Is_Variable_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!(
                    "Switch On Variable `{}` Which is Not Defined", _Statement.Variable_Name
                ));
            }

            let mut _Variable = __Analyzer.Get_Variable(&_Statement.Variable_Name);

            if _Variable.Type != Some(TOKEN_TYPE::INT) && _Variable.Type != Some(TOKEN_TYPE::CHAR) &&
                _Variable.Type != Some(TOKEN_TYPE::STRING){
                
                __Analyzer.Raise_Error(format!(
                    "Variable `{}` is of Type `{:?}` Which Switch Statement Does not Support",
                    _Variable.Name,
                    _Variable.Type.unwrap_or(TOKEN_TYPE::NONE)
                ));
            }


            for Switch_Node in _Statement.Switch.as_ref().unwrap().Childs.iter(){
                
                for Switch_Variable in Switch_Node.Variables.iter(){
                    
                    if _Variable.Type == Some(TOKEN_TYPE::INT) && Switch_Variable.Token_Type != TOKEN_TYPE::NUMBER ||
                       _Variable.Type == Some(TOKEN_TYPE::CHAR) && Switch_Variable.Token_Type != TOKEN_TYPE::CHARACTER ||
                       _Variable.Type == Some(TOKEN_TYPE::STRING) && Switch_Variable.Token_Type != TOKEN_TYPE::STRING_SEQUENCE{
                        __Analyzer.Raise_Error(format!(
                            "Switch On Variable `{}` Of Type `{:?}` Found `{}` of Type `{:?}`",
                            _Variable.Name,
                            _Variable.Type.unwrap_or(TOKEN_TYPE::NONE),
                            Switch_Variable.Value,
                            Switch_Variable.Token_Type
                        ));
                    }
                }

                __Analyzer.Environment_Stack.push(__Analyzer.Current_Environment.clone());
                __Analyzer.Current_Environment = Environments::Environment::new(Environments::ENVIRNMENT::SWITCH);

                Statements(&mut __Analyzer, &Switch_Node.Statements);

                __Analyzer.Current_Environment = __Analyzer.Environment_Stack.pop().unwrap();

            }
        }


        
        fn Define_Loop(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            let mut _Variable = Environments::Variable::new();

            match &_Statement.Loop.as_ref().unwrap().Expression{

                None => (),

                Some(_Expression) => {
                    let _Expression = _Expression;

                    match _Expression.Loop_Type{

                        Trees::OPERATION_TYPE::FOR_LOOP => {
                            _Variable.Type = Some(TOKEN_TYPE::INT);
                            _Variable.Name = _Expression.Variable.clone();

                            Check_int_Value(&mut __Analyzer, &_Expression.First_Expression.as_ref().unwrap());

                            if _Expression.Second_Expression != None{
                                Check_int_Value(&mut __Analyzer, &_Expression.Second_Expression.as_ref().unwrap());

                                if _Expression.Third_Expression != None{
                                    Check_int_Value(&mut __Analyzer, &_Expression.Third_Expression.as_ref().unwrap());
                                }
                            }
                        },
    
                        _ => Handle_If_Expression(&mut __Analyzer, &_Expression.First_Expression.as_ref().unwrap())
                    }
                }
            }

            __Analyzer.Environment_Stack.push(__Analyzer.Current_Environment.clone());
            __Analyzer.Current_Environment = Environments::Environment::new(Environments::ENVIRNMENT::LOOP);
            __Analyzer.Current_Environment.Register_Variable(_Variable);

            Statements(&mut __Analyzer, &_Statement.Loop.as_ref().unwrap().Statements);

            __Analyzer.Current_Environment = __Analyzer.Environment_Stack.pop().unwrap();
        }



        fn Break_Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            for _Environment in __Analyzer.Environment_Stack.iter().rev(){
                if _Environment.Scope == Environments::ENVIRNMENT::LOOP{
                    return
                }
            }

            if __Analyzer.Current_Environment.Scope == Environments::ENVIRNMENT::LOOP{
                return
            }

            __Analyzer.Raise_Error(format!(
                "Break Statement Outside Loop Statement"
            ));
        }



        fn Continue_Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            for _Environment in __Analyzer.Environment_Stack.iter().rev(){
                if _Environment.Scope == Environments::ENVIRNMENT::LOOP{
                    return
                }
            }

            
            if __Analyzer.Current_Environment.Scope == Environments::ENVIRNMENT::LOOP{
                return
            }

            __Analyzer.Raise_Error(format!(
                "Continue Statement Outside Loop Statement"
            ));
        }



        fn Call_Function(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            if !__Analyzer.Is_Function_Defined(&_Statement.Variable_Name){
                __Analyzer.Raise_Error(format!("Function `{}` Is Undefined", _Statement.Variable_Name));
            }

            for Param in _Statement.Function_Params.as_ref().unwrap().iter(){
                if !__Analyzer.Is_Variable_Defined(Param){
                    __Analyzer.Raise_Error(format!("Variable `{}` Is Undefined", Param));
                }
            }

            if !__Analyzer.Check_Params_Count(&_Statement.Variable_Name, &_Statement.Function_Params.as_ref().unwrap(), vec![
                TOKEN_TYPE::INT, TOKEN_TYPE::DOUBLE, TOKEN_TYPE::BOOL, TOKEN_TYPE::CHAR, TOKEN_TYPE::STRING, TOKEN_TYPE::VOID
            ]){
                __Analyzer.Raise_Error(format!(
                    "Function `{}` Has No Definition With This Params \n{:#?}", _Statement.Variable_Name, _Statement.Function_Params
                ));
            }
        }


        fn Define_Function(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            let mut __Analyzer = __Analyzer;
            let _Function = __Analyzer.Functions.get(&_Statement.Function.as_ref().unwrap().Function_Name).clone();
            let _Function = _Function.unwrap().Clone();

            for (i, _Definition) in _Statement.Function.as_ref().unwrap().Childs.iter().enumerate(){

                __Analyzer.Environment_Stack.push(__Analyzer.Current_Environment.clone());
                let __Function = _Function.clone();
                __Analyzer.Current_Environment = Environments::Environment::new(Environments::ENVIRNMENT::FUNCTION(
                    __Function, i
                ));

                for _Variable in _Function.Definitions[i].Params.iter(){
                    let mut Variable = Environments::Variable::new();
                    Variable.Name = _Variable.Name.clone();
                    Variable.Type = Some(_Variable.Type.clone());
                    __Analyzer.Current_Environment.Register_Variable(Variable);
                }

                Statements(&mut __Analyzer, &_Definition.Statements);

                __Analyzer.Current_Environment = __Analyzer.Environment_Stack.pop().unwrap();
            }
        }



        fn Return_Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            match &__Analyzer.Current_Environment.Scope{

                Environments::ENVIRNMENT::FUNCTION(_Function, i) => {

                    let _Definition = _Function.Definitions.get(i.clone()).unwrap();

                    if _Statement.Variable_Name.eq(""){

                        if _Definition.Return_Type == TOKEN_TYPE::VOID{
                            return
                        }

                        else{
                            __Analyzer.Raise_Error(format!(
                                "return Variable On Function `{}` Definition That return VOID",
                                _Function.Name
                            ));
                        }
                    }

                    else{
                        if !__Analyzer.Is_Variable_Defined(&_Statement.Variable_Name){
                            __Analyzer.Raise_Error(format!(
                                "Variable `{}` is Undefined", &_Statement.Variable_Name
                            ));
                        }

                        let mut _Variable = __Analyzer.Get_Variable(&_Statement.Variable_Name);

                        if Some(_Definition.Return_Type) != _Variable.Type{
                            __Analyzer.Raise_Error(format!(
                                "Function `{}` Definition Return Type Is `{:?}` And The Variable `{}` Return Type Is `{:?}`",
                                _Function.Name,
                                _Definition.Return_Type,
                                _Variable.Name,
                                _Variable.Type.unwrap_or(TOKEN_TYPE::VOID)
                            ));
                        }
                    }

                    return 
                },

                _ => ()
            }

            for (i, _Environment) in __Analyzer.Environment_Stack.iter().rev().enumerate(){

                match &__Analyzer.Current_Environment.Scope{

                    Environments::ENVIRNMENT::FUNCTION(_Function, i) => {
    
                        let _Definition = _Function.Definitions.get(i.clone()).unwrap();
    
                        if _Statement.Variable_Name.eq(""){
    
                            if _Definition.Return_Type == TOKEN_TYPE::VOID{
                                return
                            }
    
                            else{
                                __Analyzer.Raise_Error(format!(
                                    "return Variable On Function `{}` Definition That return VOID",
                                    _Function.Name
                                ));
                            }
                        }
    
                        else{
                            if !__Analyzer.Is_Variable_Defined(&_Statement.Variable_Name){
                                __Analyzer.Raise_Error(format!(
                                    "Variable `{}` is Undefined", &_Statement.Variable_Name
                                ));
                            }
    
                            let mut _Variable = __Analyzer.Get_Variable(&_Statement.Variable_Name);
    
                            if Some(_Definition.Return_Type) != _Variable.Type{
                                __Analyzer.Raise_Error(format!(
                                    "Function `{}` Definition Return Type Is `{:?}` And The Variable `{}` Return Type Is `{:?}`",
                                    _Function.Name,
                                    _Definition.Return_Type,
                                    _Variable.Name,
                                    _Variable.Type.unwrap_or(TOKEN_TYPE::VOID)
                                ));
                            }
                        }
                    },
    
                    _ => ()
                }
            }

            __Analyzer.Raise_Error(format!(
                "Return Statement Outside Loop Statement"
            ));
        }



        fn Print_Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            

            let mut __Analyzer = __Analyzer;

            if !__Analyzer.Is_Variable_Defined(&_Statement.Variable_Name.clone()){
                __Analyzer.Raise_Error(format!(
                    "Variable `{}` is Undefined", _Statement.Variable_Name
                ));
            }
        }


        fn Input_Statement(__Analyzer: &mut Analyzer, _Statement: &Trees::Statement_Syntax_Node){

            let mut __Analyzer = __Analyzer;

            if !__Analyzer.Is_Variable_Defined(&_Statement.Variable_Name.clone()){
                __Analyzer.Raise_Error(format!(
                    "Variable `{}` is Undefined", _Statement.Variable_Name
                ));
            }
        }
    }
}
