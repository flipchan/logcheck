// grep -o '[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}' /var/log/hiawatha/blogaccess.log|uniq| wc -l does not scale
extern crate regex;
extern crate chrono;
use chrono::{DateTime, Utc};

use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;
use regex::Regex;
use std::collections::HashSet; // use it like python's set()
use std::process::exit;

//plan
// call it like ./logcheck /file/path/to/access/log
//regex out the ips and unique sort and count it, stream parse the 

fn todaysstuff(filen: String) { //take path as string
// different logs have different formats | openbsd httpd has domain.com ipaddress - - [03/Apr/2020:11:10:00 +0300] "GET / 
//											this i need to find
// ideally it should auto detect the log format based regex matches
	let now: DateTime<Utc> = Utc::now();
	let dateprint = now.format("%Y-%m-%d");
	let sumtag = now.format("%a %b %e %Y");
	let this = now.format("%d/%b/%Y");
	let this = format!("{}", this);
	let mut counter = 0;
	println!("Checking visitors for today: {}", sumtag);

	  let file = File::open(filen).unwrap();
	 for line in BufReader::new(file).lines() { //stream parse the file
	    let linje = line.as_ref().unwrap();
	    let mut parts = linje.to_string();
	     if parts.contains(&this) { // if we find todays date in the log file
		counter += 1;
//		amount.insert(caps.get(0).unwrap().as_str().to_string());   // insert the 
	    }
			    }

	println!("Amount of visitors today: {}", counter);

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
  println!("Amount of unique ips in the file is {}", books.len());
  }
