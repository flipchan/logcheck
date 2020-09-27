// use pledge
use pledge::pledge;
use chrono::{DateTime, Utc};
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;
use regex::Regex;
use std::collections::HashSet; // use it like python's set()
use std::process::exit;


fn printsecure(strang: String, amount: usize) {

	pledge("stdio", "stdio");
//  pledge_promises![Stdio]
//        .or_else(pledge::Error::ignore_platform)
 //       .unwrap();
	println!("{} {}", strang, amount);
}

// adding secure pledge
fn output_print(amount: u32) {
	pledge("stdio", "stdio");
//  pledge_promises![Stdio]
//        .or_else(pledge::Error::ignore_platform)
//        .unwrap();
	println!("Amount of visitors today: {}", amount);
}


fn read_n_count(filename: String) -> usize {

	let now: DateTime<Utc> = Utc::now();
	let dateprint = now.format("%Y-%m-%d");
	let this = now.format("%d/%b/%Y");
	let this = format!("{}", this);
	let mut counter = 0;
//	let promises = vec![Promise::Rpath, Promise::Flock];
//	let execpromises = vec![Promise::Stdio, Promise::Tty];
//let promises = vec![Promise::Stdio, Promise::Exec];
//https://github.com/i80and/pledge-rs
    pledge("rpath flock", None).unwrap();

//  pledge_promises![rpath, flock] // i want to be able to read files(rpath) and open files(flock)
  //      .or_else(pledge::Error::ignore_platform)
//        .unwrap();

	  let file = File::open(filename).unwrap();
	 for line in BufReader::new(file).lines() { //stream parse the file
	    let linje = line.as_ref().unwrap();
	    let mut parts = linje.to_string();
	     if parts.contains(&this) { 
		counter += 1;
	    				}

						}

	counter
		}


fn todaysstuff(filen: String) { //take path as string
	let now: DateTime<Utc> = Utc::now();
	let dateprint = now.format("%Y-%m-%d");
	let sumtag = now.format("%a %b %e %Y");
	let this = now.format("%d/%b/%Y");
	let this = format!("{}", this);
	let mut counter = 0;
	println!("Checking visitors for today: {}", sumtag);

/*
	  let file = File::open(filen).unwrap();
	 for line in BufReader::new(file).lines() { //stream parse the file
	    let linje = line.as_ref().unwrap();
	    let mut parts = linje.to_string();
	     if parts.contains(&this) { // if we find todays date in the log file
		counter += 1;
//		amount.insert(caps.get(0).unwrap().as_str().to_string());   // insert the 
	    }
			    }

*/
	let securenew = read_n_count(filen);

	output_print(counter);
	//println!("Amount of visitors today: {}", counter);

}


fn main() {
  println!("logcheck by flipchan");
  let args: Vec<String> = env::args().collect();// if len == 1 // exit 
  if args.len() != 2 {
		println!("please provide a file path");
		exit(1);
	}
   let openme = &args[1];
  let file = File::open(openme).unwrap();
    let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
    let mut books = HashSet::new();
    books.insert("testing".into());
 for line in BufReader::new(file).lines() { //stream parse the file
    let linje = line.as_ref().unwrap();
    let mut parts = linje.to_string();
    for caps in re.captures_iter(&parts.as_str()) {
        let caps = caps;
        books.insert(caps.get(0).unwrap().as_str().to_string());   // problem is here
    }
		    }
    books.remove("testing");
//  println!("Amount of unique ips in the file is {}", books.len());
	printsecure("Amount of unique ips in the file is".to_string(), books.len())

  }
