
extern crate serde_json;
extern crate lmdb_zero as lmdb;
extern crate regex;

use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;
use regex::Regex;
use std::collections::HashSet; // use it like python's set()


//plan
// call it like ./logcheck /file/path/to/access/log
//regex out the ips and unique sort and count it, stream parse the 

fn grepip(lineoffile: &str) ->str {
  let ipv4find = Regex::new("").unwrap(); //ipv4 regex
  let findings = ipv4find.find(&lineoffile).unwrap();
  println!("i have found {}", findings);
  return "ip here";
}

fn main() {
  println!("logcheck by flipchan");
  let mut ipcounter = HashSet::new();
     let args: Vec<String> = env::args().collect();
    let openme = &args[1];
 // let openme = "file here";
  let file = File::open(openme).unwrap();
  println!("file is open!");
  println!("starting to count it");
 for line in BufReader::new(file).lines() { //stream parse the file
    //do things
   let loot = grepip(line);
   if loot != "nope"{ //append to global uniq list 
   ipcounter.insert(loot.to_string());//insert ip address
   }
                                            }

  println!("Amount of unique ips in the file is {}", ipcounter.len());
  }
