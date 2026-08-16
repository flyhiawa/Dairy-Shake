#![allow(unused_imports)]

use crate::types::{Token, Types};

use std::{
	env::{Args, args}, fmt::format, fs::{File, read}, io::{BufRead, BufReader, Error}, ops::Index, path::{self, PathBuf}, vec::Vec,
};

fn tokenizer() -> Result<Vec<Token>, String> {
    let args_vec: Vec<String> = args().collect();

	let path_file = PathBuf::from(args_vec.index(1));
	let existe = path_file.try_exists().map_err(|e| {
    	format!("\x1B[31mError\x1B[0m: It was not possible to verify if the file exists on your OS. {}", e)
	})?;

	if !existe {
    	return Err(format!("\x1B[31mError\x1B[0m: File does not exist"));
	}

	let mut tokens: Vec<Token> = Vec::new();

	let file_path = File::open(path_file).map_err(|e| e.to_string())?;
	let reader= BufReader::new(file_path);

	for lines_result in reader.lines() {
		let lines: String = lines_result.map_err(|e|e.to_string())?;
		for wchars in lines.chars() {
			
		}
	}
	
	Ok(tokens)
}