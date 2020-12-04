#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_mut)]




// Use Statements Here
use crate::CharacterLib::Character;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Write;









#[derive(Debug)]
pub struct File{
    
    __Position: u64,
    __File_Length: u64,
    __File: std::fs::File,
}


impl File{

    pub fn new(File_Name: String) -> Self{

        match std::fs::File::open(File_Name){

            Err(_) => panic!("File Exception: Can't Open File"),
            
            Ok(Opened_File) => {

                return File{
                    __Position: 0,
                    __File_Length: 0,
                    __File: Opened_File
                }
            }
        }
    }

    pub fn create_new(File_Name: String) -> Self{

        match std::fs::File::create(File_Name.clone()){

            Err(_) => panic!("File Exception: Can't Create File: `{}`", File_Name),

            Ok(Opened_File) => {
                
                return File{
                    __Position: 0,
                    __File_Length: 0,
                    __File: Opened_File
                }
            }
        }
    }
}


impl File{

    pub fn Open(&mut self){
        self.__File_Length = self.__File.metadata().expect("Error").len();
    }


    pub fn Peek(&mut self, index: u64) -> Character{
        use std::io::Read;
        use std::char;

        if index >= self.__File_Length{
            return Character::new(0)
        }

        let _index = index + self.__Position;

        match self.__File.seek(SeekFrom::Start(_index)){
            Err(error) => panic!("File Exception: Error Reading Character"),

            Ok(result) => {
                let mut _Char = [0; 1];
        
                match self.__File.read(&mut _Char) {
                    Err(error) => panic!("File Exception: Error in Reading Character"),

                    Ok(Char) => Char
                };

                match self.__File.seek(SeekFrom::Start(self.__Position)){
                    Err(error) => panic!("File Exception: Error in Reading Character"),

                    Ok(result) => return Character::new(_Char[0])
                };
            }
        };
    }

    pub fn Read(&mut self) -> Character{

        use std::io::Read;
        use std::char;

        if self.__Position >= self.__File_Length{
            return Character::new(0)
        }

        let mut _Char = [0; 1];
        
        match self.__File.read(&mut _Char) {
            Ok(Char) => Char,
            Err(_) => panic!("File Exception: Error in Reading Character"),
        };
        self.__Position += 1;

        return Character::new(_Char[0])
    }

    pub fn Read_Line(&mut self) -> String{

        let mut Line = String::new();

        loop{
            let Char = self.Read();
            if Char.is_eof() || Char.is_newline(){
                break
            }
            Line += &Char.To_String();
        }

        return Line
    }

    pub fn Write(&mut self, Data: String){
        match self.__File.write_all(Data.as_bytes()){
            Err(_) => panic!("File Exception: Error in Writing Data Into File"),

            Ok(Re) => ()
        }
    }
}


impl File{

    pub fn Delete_File(File_Path: String){

        match std::fs::remove_file(File_Path){
            Err(_) => (),

            Ok(Re) => ()
        }
    }
}
