// main.rs
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::error::Error;


// https://github.com/docopt/docopt.rs
use docopt::Docopt;

// https://doc.rust-lang.org/regex/regex/index.html
use regex::Regex;

docopt!(Args derive Debug,"
Rus Fritz.

Usage:
    rus_fritz -e <engfile>  -r <rusfile> ( -o <outfile> | --stdout ) [ -q ] [--askme] 
    rus_fritz --help
    rus_fritz --version
Options:
  -q		Quet mode
  --askme	Ask Me for translate
  --help	Display this help and exit
  --version	Output version information and exit
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

/// Открыть файл
fn open_any_file( file_name: &str ) -> BufReader<File> {
    let path = Path::new(file_name);
    let display = path.display();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(why) => panic!("Not open file {}: {}", display, Error::description(&why) ), 
    };
    let reader = BufReader::new(file);
    return reader;
}

/// Создать и открыть файл для записи результатов
fn create_out_file( file_name: &str ) -> BufWriter<File> {
    let path = Path::new(file_name);
    let display = path.display();
    let mut options = OpenOptions::new();
    let file = match options.create(true).write(true).truncate(true).open(path) {
	    Ok(file) => file,
	    Err(why) => panic!("Not create/open file {}: {}", display, Error::description(&why)),
	};
    let writer = BufWriter::new(file);
    return writer;
}

fn ask_me_trans( opis: &str, flag_q: bool ) -> String {

    let mut in_buff = String::new();
    let mut _output = String::new(); 

    // Debug print:
    if flag_q {
        println!("{}", opis);
    } else {
        println!("Переведите: {}", opis);
    }
    
    let innum = std::io::stdin()
            .read_line( &mut in_buff )
            .ok()
            .expect("Failed to read line");
    if !flag_q { 
            println!("Ok, waiting next ..."); 
    }              
    // If User input chars              
    if innum > 1 {
        // Необходимо убрать перевод строки из введенной строки перевода
        _output = format!("{}", in_buff.trim() );
    } else { // Using English translating
        _output = opis.to_string();
    }
    return _output;
}

// ----------------------------------------------------------------------
fn main() {
    let docopt = Args::docopt();
    let args: Args = docopt.decode().unwrap_or_else(|e| e.exit());
    
    if args.flag_stdout {
        if args.flag_version { println!("Print Version coming soon ..." ); return }
    }

    let mut engvec: Vec<EnglishName> = Vec::new();
    let mut rusvec: Vec<RussianName> = Vec::new();

    let re = Regex::new(r"(.*)(,)(.*)(\x22(.*)\x22)").unwrap();

    // Create English Vector 
    for line in open_any_file(&args.arg_engfile).lines() {
        let s = line.unwrap();
        for cap in re.captures_iter(&s) {
            let ubs = EnglishName::new( cap.at(1).unwrap(), cap.at(5).unwrap() );
            engvec.push(ubs);
        }
    }
    assert!( !engvec.is_empty() );
    // Create Russian Vector
    for line in open_any_file(&args.arg_rusfile).lines() {
        let s = line.unwrap();
        for cap in re.captures_iter(&s) {
            let ubs = RussianName::new( cap.at(1).unwrap(), cap.at(5).unwrap() );
            rusvec.push(ubs);
        }
    }
    // assert!( !rusvec.is_empty() );

	//  Подготовка закончилась, начинаем работу.
    let mut found = false;
    let mut outstr  = String::new();
	let mut _transme = String::new();

    if args.flag_stdout {
        println!("/*\n\tRecreated by RusFritz project\n*/\n");	
    } else {
		// Write header to file
		outstr.push_str( &format!("/*\n\tRecreated by RusFritz project\n*/\n") );
    }

	// Main Loop
    for e in &mut engvec {
        for r in &mut rusvec {
            if e.name == r.name {
                e.opis = r.opis.clone();
                found = true;
                r.using = true;
                if args.flag_stdout {
                    println!("{},\t\t\"{}\";", r.name, r.opis);
                } else {
                    let s = String::from( format!("{},\t\t\"{}\";\n", r.name, r.opis) );
                    outstr.push_str(&s);
                }
            }
        }
        if !found {
            if  args.flag_askme {
                // Запросим перевод 
                _transme = format!("{},\t\t\"{}\";", e.name, ask_me_trans( &e.opis, args.flag_q ) );
            } else {
                _transme = format!("{},\t\t\"{}\";", e.name, e.opis);
            }
            if args.flag_stdout {
                println!("{}", _transme ); 
            } else {
               outstr.push_str( &format!("{}\n",_transme) ); 
            }
        }
        found = false;
    }
    // Print if not using from Russian Names
    if args.flag_q == false {
        println!("------ Not using Found from Russian File --------");
        for r in rusvec {
            if r.using == false {
                println!("{},\t\t\"{}\"",r.name,r.opis);
            }
        }
    }

    // File write, if needed
    if args.flag_stdout == false {
        let mut wr = create_out_file( &args.arg_outfile );
        // Check Len OutString
        if outstr.len() > 0 {
            match wr.write_all(&outstr.as_bytes()) {
                Err(why) => {
                    panic!("Couldn't write to{}: {}", 
                        &args.arg_outfile.to_string(),
                        Error::description(&why)) 
                },
                Ok(_) => (),
            };
            match wr.flush() {
                Err(why) => panic!("Error - don't flush: {}", Error::description(&why) ),
                Ok(_) => (),
            };	
        } // End check Len
    }

}
