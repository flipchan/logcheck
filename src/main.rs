
extern crate serde_json;
extern crate lmdb_zero as lmdb;

use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;

//plan
// call it like ./logcheck /file/path/to/access/log
//regex out the ips and unique sort and count it, stream parse the 

fn grepip(lineoffile: &str) str{
  return "ip here";
}

fn main() {
  println!("logcheck by flipchan")
     let args: Vec<String> = env::args().collect();
    let openme = &args[1];
 // let openme = "file here";
  let file = File::open(openme).unwrap();
  println!("file is open!");
 for line in BufReader::new(file).lines() { //stream parse the file
    //do things
                                            }
}
