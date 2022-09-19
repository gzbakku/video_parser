

use std::env::current_dir;
use std::fs::File;
use std::io::{Read,Write};

#[allow(dead_code)]
pub fn cwd()-> String{
    match current_dir(){
        Ok(buf)=>{
            match buf.into_os_string().into_string(){
                Ok(s)=>{return s;},
                Err(_)=>{String::new()}
            }
        },
        Err(_)=>{String::new()}
    }
}

#[allow(dead_code)]
pub fn read_file(f:String) -> Result<Vec<u8>,&'static str>{
    match File::open(f){
        Ok(mut file)=>{
            let mut reader:Vec<u8> = Vec::new();
            match file.read_to_end(&mut reader){
                Ok(_)=>{
                    return Ok(reader);
                },
                Err(_)=>{
                    return Err("failed-file_read");
                }
            }
        },
        Err(_)=>{
            return Err("failed-file_open");
        }
    }
}

#[allow(dead_code)]
pub fn write_file(path:String,data:Vec<u8>)->Result<(),&'static str>{
    match File::create(&path){
        Ok(mut file)=>{
            match file.write_all(&data){
                Ok(_)=>{
                    return Ok(());
                },
                Err(_)=>{
                    return Err("failed-file_write-write_file");
                }
            }
        },
        Err(e)=>{
            println!("!!! {:?}",path);
            println!("!!! {:?}",e);
            return Err("failed-file_create-write_file");
        }
    }
}