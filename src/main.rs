#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]




// Import Modules
mod CharacterLib;
mod Filelib;
mod TokenTypeLib;
mod TokenLib;
mod LexerLib;
mod ParserLib;
mod Parse;
mod SyntaxTreeLib;
mod Analyze;
mod SymanticAnalyzerLib;
mod EngineCode;
mod CodeGeneratorLib;






// Use Namespaces
use crate::ParserLib::Parser;
use crate::Parse::Parse;
use crate::SymanticAnalyzerLib::Analyzer;
use crate::Analyze::Analyze;
use crate::SymanticAnalyzerLib::Environments;
use crate::EngineCode::GenerateCode;
use crate::CodeGeneratorLib::CodeGenerator;
use std::env;
use std::io::{stdin, stdout, Write};






// Main

fn main() {

    /*let File_Name = String::from(
        "E:/My Projects/Engine Programming Language/Engine Compiler/compiler_in_rust/init.eng"
    );*/

    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        panic!("Please Enter File Path Only");
    }

    let mut _Parser = Parser::new(args[1].clone());

    Parse(&mut _Parser);

    let mut _Analyzer = Analyzer::new(
        Environments::ENVIRNMENT::MAIN
    );
    Analyze(&mut _Analyzer, &_Parser.Syntax_Tree);

    let Engine_Code_File = String::from(
        "E:/My Projects/Engine Programming Language/Engine Compiler/compiler_in_rust/init.vir"
    );

    let mut _CodeGenerator = CodeGenerator::new(
        Engine_Code_File
    );

    GenerateCode(&mut _CodeGenerator, &mut _Parser.Syntax_Tree);
}
