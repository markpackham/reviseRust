#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;


fn main() {
   let str1 = String::from("World");
   // clone() duplicates str1's value
   // if I used the assignment operator rather than clone()
   // then str1 would lose it's value in memory
   let str2: String = str1.clone();
   // str1 would be empty without clone since str2 would steal it's value
   println!("Hello {}",str1);
}
