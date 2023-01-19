#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn get_2(x: i32) -> (i32, i32) {
   return (x+1,x+2);
}


fn main() {
   let (va1_1, val_2) = get_2(10);
   println!("{} {}", va1_1, val_2);
}
