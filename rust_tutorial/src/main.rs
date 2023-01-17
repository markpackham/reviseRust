#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    // the "mut" stand for mutable so changeable
    // variables in rust are immutable by default
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(buf: &mut name) Result<usize, Error>.expect(msg: "Didn't recieve input!");
    // if you see an ! at the end of a function it is a macro
    println!("Hello {}! {}", name.trim_end(), greeting);
}
