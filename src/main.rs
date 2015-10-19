// main.rs
extern crate regex;

use std::fmt::{ Display, Formatter, Error };
//use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
//use std::io::{BufReader,BufRead};
use std::io::BufReader;
use std::path::Path;
use std::env;


// https://doc.rust-lang.org/regex/regex/index.html
use regex::Regex;

/*
trait EnglishTrait {
	fn ename(&self) -> String;
	fn eopis(&self) -> String;
}
*/

struct EnglishName {
	name: String,
	opis: String,
}

struct RussianName {
	name: String,
	opis: String,
}
/*
struct RussianNameIntoIterator {
    rusname: RussianName,
    index: usize,
}

impl Iterator for RussianNameIntoIterator {
	type Item = String;
	fn next(&mut self) -> Option<String> {
		let result = match self.index {
			0 => Some(self.rusname.name),
			1 => Some(self.rusname.opis),
			_ => return None,
		};
		self.index += 1;
		result
	}
}

impl IntoIterator for RussianName {
	type Item = String;
	type IntoIter = RussianNameIntoIterator;
	
	fn into_iter(self) -> Self::IntoIter {
		RussianNameIntoIterator { rusname: self, index: 0 }
	}
}
*/

/*
impl EnglishTrait for EnglishName {
	fn ename(&self) -> String {
		return format!("{}", self.name);
	}
	
	fn eopis(&self) -> String {
		return format!("{}", self.opis);
	}
}
*/

impl EnglishName {
	fn new<S: Into<String>>(name: S, opis: S) -> EnglishName {
		EnglishName {
			name: name.into(),
			opis: opis.into(),
		}
	}
}

impl RussianName {
	fn new<S: Into<String>>(name: S, opis: S) -> RussianName {
		RussianName {
			name: name.into(),
			opis: opis.into(),
		}
	}
}


impl std::fmt::Display for EnglishName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        //Ok(())
        write!(f, "Display Eng: {},\t\t\t{}", self.name, self.opis)
    }
}

impl std::fmt::Display for RussianName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        	write!(f, "Display Rus: {},\t\t\t{}", self.name, self.opis)
    }
}

// http://ru.stackoverflow.com/questions/445905/%D0%9F%D0%B5%D1%80%D0%B5%D0%B4%D0%B0%D1%82%D1%8C-%D0%B2%D0%B5%D0%BA%D1%82%D0%BE%D1%80-%D1%81%D1%82%D1%80%D1%83%D0%BA%D1%82%D1%83%D1%80-%D0%BF%D0%BE-%D1%81%D1%81%D1%8B%D0%BB%D0%BA%D0%B5-%D0%B2-%D1%84%D1%83%D0%BD%D0%BA%D1%86%D0%B8%D1%8E-%D0%B8-%D0%B2%D1%8B%D0%BF%D0%BE%D0%BB%D0%BD%D0%B8%D1%82%D1%8C-%D0%B2-%D0%BD%D1%91%D0%BC-%D0%BF%D0%BE%D0%B8%D1%81%D0%BA
/*
fn find_rus_opis( russians: &Vec<RussianName>, name_find: String) -> Option<&String> {
	for rus in russians {
		println!("{}", rus.name);
		
		for it in &rus.opis {
			
			if *it == name_find {
				return Some(&rus.opis);
			}
			
		}
		
	}
	None
}
*/

fn main() {
	
	
	let mut engvec: Vec<EnglishName> = Vec::new();
	let mut rusvec: Vec<RussianName> = Vec::new();
	
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
   
    let efile = BufReader::new(File::open(&fname_eng).unwrap());
    let rfile = BufReader::new(File::open(&fname_rus).unwrap());

	// Read file by lines 
	for eline in efile.lines().filter_map(|result| result.ok()) {
		let re = Regex::new(r"(.*)(,)(.*)(\x22.*\x22)").unwrap();
		for cap in re.captures_iter(&eline) {
			let s = EnglishName::new( cap.at(1).unwrap(), cap.at(4).unwrap() );
			engvec.push(s);
		}
    }
	//println!("English Vector len: {}", engvec.len());
	// Проверка что вектор заполнен
	assert!( !engvec.is_empty() );
	
	// Read file by lines 
	for rline in rfile.lines().filter_map(|result| result.ok()) {
		let re = Regex::new(r"(.*)(,)(.*)(\x22.*\x22)").unwrap();
		for cap in re.captures_iter(&rline) {
			let s = RussianName::new( cap.at(1).unwrap(), cap.at(4).unwrap() );
			rusvec.push(s);
		}
    }
	// Проверка что вектор заполнен
	assert!( !rusvec.is_empty() );
	
	//  Подготовка закончилась, начинаем работу.
	/*
    for sengvect in engvec.iter() {
    	println!("{}", sengvect);
    }
    */
	
	let mut i=0;
	let mut found = false;
	
    for e in &mut engvec {
    	i = i+1;
    	for r in &mut rusvec {
    		if e.name == r.name {
    			e.opis = r.opis.clone();
    			found = true;
    			println!("{}\t\t{}", r.name, r.opis);
    		}
    	}
    	if !found {
    		//println!("ERR: {} - не найдено соответствие", e.name );
    		println!("NeedModify: {}\t\t{}",e.name, e.opis);
    	}
    	found = false;
    }
    
    println!("Обработано {} строк из {}",i, engvec.len());
    //let writer = std::io::file_writer(&Path(std::os::args()[2]), [io::Append, io::Create]).unwrap();

}
