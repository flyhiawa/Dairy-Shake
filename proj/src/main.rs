use std::{
    collections::HashMap,
    env, 
    fmt::Error, 
    fs, 
    io::{
        self,
        BufRead,
        BufReader
    }, 
    path::Path, 
    process, 
    vec::Vec,
};

pub struct SymName(String);

impl SymName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.contains(' ') || name.is_empty() {
            return Err("d".into());
        }

        Ok(SymName(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

enum Types {
    Str(String),
    Sym(SymName),
    //Var(),
    I8(i8), I16(i16), I32(i32), I64(i64), I128(i128),

    Set(Vec<Box<Types>>)
}

fn main() {
    //let args: Vec<&str> = env::args().collect();
    
    //if args.len() != 2 {
        //println!("{}: The command must have exactly 1 argument;\nEnter the correct path of the .mal file", "\x1B[31mError");
        //process::exit(1);
    //}

    //let path_mal = fs::File::open(&args[1]);
    //let reader = BufReader::new(path_mal);
    parsing();
}

fn parsing() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}: The command must have exactly 1 argument;\nEnter the correct path of the . file", "\x1B[31mError");
        process::exit(1);
    }

    let path_mal = fs::File::open(&args[1]);
    let reader = BufReader::new(path_mal);
}
