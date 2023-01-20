#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;


fn print_str(x: String){
   println!("A string {}", x);
}

fn print_return_str(x: String)-> String{
   println!("A string {}", x);
   return x;
}

fn change_string(name: &mut String){
   name.push_str(" is happy");
   println!("Message : {}", name);
}

fn main() {
   let mut str1 = String::from("Edward");
   change_string(&mut str1);
}
