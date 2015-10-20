// main.rs
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate regex;

use std::fmt::{ Display, Formatter };
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;
// https://github.com/docopt/docopt.rs
use docopt::Docopt;

// https://doc.rust-lang.org/regex/regex/index.html
use regex::Regex;

docopt!(Args derive Debug, "
Alexey Mekhanoshin

Usage:
	rus_fritz -e <engfile>  -r <rusfile> ( -o <outfile> | --stdout )
	rus_fritz (-h | --help)
	rus_fritz --version
Options:
  -h --help		Show this screen.
  --version		Show version.
");

struct EnglishName {
    name: String,
    opis: String,
}

struct RussianName {
    name: String,
    opis: String,
    using: bool,
}

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
            using: false,
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


fn open_eng_file(engvec: &mut Vec<EnglishName>, file_name: &String) {
    let path = Path::new(file_name);
    let display = path.display();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(why) => panic!("Not open file {}: {}", display, Error::description(&why) ), 
    };
    let re = Regex::new(r"(.*)(,)(.*)(\x22.*\x22)").unwrap();
    for line in BufReader::new(file).lines() {
        let s = line.unwrap();
        for cap in re.captures_iter(&s) {
            let ubs = EnglishName::new( cap.at(1).unwrap(), cap.at(4).unwrap() );
            engvec.push(ubs);
    }
    }
}

fn open_rus_file(engvec: &mut Vec<RussianName>, file_name: &String) {
    let path = Path::new(file_name);
    let display = path.display();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(why) => panic!("Not open file: {}: {}", display, Error::description(&why) ), 
    };
    let re = Regex::new(r"(.*)(,)(.*)(\x22.*\x22)").unwrap();
    for line in BufReader::new(file).lines() {
        let s = line.unwrap();
        for cap in re.captures_iter(&s) {
            let ubs = RussianName::new( cap.at(1).unwrap(), cap.at(4).unwrap() );
            engvec.push(ubs);
        }
    }
}

fn create_out_file(file_name: &String) -> BufWriter<File> {
    let path = Path::new(file_name);
    let display = path.display();
    let mut options = OpenOptions::new();
    let file = match options.create(true).write(true).open(path) {
	    Ok(file) => file,
	    Err(why) => panic!("Not create/open file {}: {}", display, Error::description(&why)),
	};
    let writer = BufWriter::new(file);
    return writer;
}


fn main() {

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    if args.flag_stdout {
        println!("{:?}", args);
        if args.flag_h { println!("Print Help coming soon ..." ); return; }
        if args.flag_version { println!("Print Version coming soon ..." ); return; }
    }
     
    let mut engvec: Vec<EnglishName> = Vec::new();
    let mut rusvec: Vec<RussianName> = Vec::new();

    open_eng_file(&mut engvec, &args.arg_engfile.to_string());
    assert!( !engvec.is_empty() );
    open_rus_file(&mut rusvec, &args.arg_rusfile.to_string());
    assert!( !rusvec.is_empty() );

	//  Подготовка закончилась, начинаем работу.

    if args.flag_stdout {
        println!("/*\n\tRecreated by RusFritz project\n*/");	
    } else {
		// Write to file
    }

    let mut i=0;
    let mut found = false;
    let mut outstr = String::new();
	
    for e in &mut engvec {
        i = i+1;
        for r in &mut rusvec {
            if e.name == r.name {
                e.opis = r.opis.clone();
                found = true;
                r.using = true;
                if args.flag_stdout {
                    println!("{},\t\t{};", r.name, r.opis);
                } else {
                    let s = String::from( format!("{},\t\t{};\n", r.name, r.opis) );
                    outstr.push_str(&s);
                }
            }
        }
        if !found {
            //println!("ERR: {} - не найдено соответствие", e.name );
            if args.flag_stdout { 
                println!("{},\t\t{};",e.name, e.opis);
            } else {
                // Wite output to file
            }
        }
        found = false;
    }
    // Print if not using from Russian Names
    println!("------ Not using Found from Russian File --------");
    for r in rusvec {
        if r.using == false {
            println!("{},\t\t{}",r.name,r.opis);
        }
    }

    // File write, if needed
    if args.flag_stdout == false {
        let mut wr = create_out_file(&args.arg_outfile.to_string());
        if outstr.len() > 0 {
            match wr.write_all(&outstr.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to{}: {}", 
                        &args.arg_outfile.to_string(),
                        Error::description(&why)) 
                },
                Ok(_) => println!("Good write file"),
            };
            wr.flush();	
        }
    }
    //println!("Обработано {} строк из {}",i, engvec.len());
    //let writer = std::io::file_writer(&Path(std::os::args()[2]), [io::Append, io::Create]).unwrap();
}
