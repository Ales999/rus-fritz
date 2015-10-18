// main.rs
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
//use std::io::{BufReader,BufRead};
use std::io::BufReader;
use std::path::Path;
use std::env;
// https://doc.rust-lang.org/regex/regex/index.html
use regex::Regex;

trait EnglishTrait {
	fn ename(&self) -> String;
	fn eopis(&self) -> String;
}

struct EnglishName {
	name: String,
	opis: String,
}

impl EnglishTrait for EnglishName {
	fn ename(&self) -> String {
		return format!("{}", self.name);
	}
	
	fn eopis(&self) -> String {
		return format!("{}", self.opis);
	}
}

impl EnglishName {
	fn new(name: &str) -> EnglishName {
		EnglishName {
			name: name.to_string(),
			opis: name.to_string(),
		}
	}
}

fn print_engn<T: EnglishTrait>(engname: T) {
	println!("Name is: {}", engname.ename() );
}
/*
fn print_vec<T: EnglishName>(xs: T) {
	println!("{:?}", xs);
}
*/
fn main() {
	
	let mut engvec: Vec<EnglishName> = Vec::new();
	
	let eng_file_name = "EngFrame.strings";
	let rus_file_name = "RusFrame.strings";
	
	// let fname_eng = Path::new("/Users/amekhanoshin/Documents/workspace/RusFritz/target/debug/EngFrame.strings");
	let cur_dir = env::current_dir().unwrap();
	println!("The current directory is {}", cur_dir.display());
	let eng_name = format!("{}/{}", cur_dir.display(), eng_file_name );
	let rus_name = format!("{}/{}", cur_dir.display(), rus_file_name );

	println!("Full Path: {}", eng_name, );
	
	let fname_eng = Path::new(&eng_name);
	let fname_rus = Path::new(&rus_name);

	/*
	let display = fname_eng.display();
    let mut efile = match File::open(&fname_eng) {
    	Err(why) => panic!("Не могу открыть {}: {}", display, Error::description(&why)),
    	Ok(file) => file,
    };
    */
    
    let efile = BufReader::new(File::open(&fname_eng).unwrap());
    //let rfile = BufReader::new(File::open(&fname_rus).unwrap());

	// Read file by lines 
	for eline in efile.lines().filter_map(|result| result.ok()) {
		//println!("{}", eline);
		let s = EnglishName::new(&eline);
		//print_engn(s);
		engvec.push(s);
    }
	println!("Vector len: {}", engvec.len());
	
    
    // Version Original:
    /*
    let mut s = String::new();
    match efile.read_to_string(&mut s) {
    	Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) =>  { //print!("{} contains:\n{}", display, s),
          print!("{} contains:\n{}", display, s)
        },
    }
    */
    
}
