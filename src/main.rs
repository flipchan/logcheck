
extern crate serde_json;
extern crate lmdb_zero as lmdb;

use std::io::{BufReader,BufRead};
use std::fs::File;



fn main() {
  println!("starting!")
  let openme = "file here";
  let file = File::open(openme).unwrap();
  println!("file is open!");
 for line in BufReader::new(file).lines() {
    //do things
                                            }
}
