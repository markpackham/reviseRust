#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
   let age2 = 8;
   match age2 {
    1..=18 => println!("Important birthday"),
    21 | 50 => println!("Important birthday"),
    65..=i32::MAX => println!("Important birthday"),
    _ => println!("Not an important birthday"),
   };
}
