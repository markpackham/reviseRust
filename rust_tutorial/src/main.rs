#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

// Generic <T> just like C# T is a placeholder, you can use any letter
// to add Generics we need to use std::ops::Add;
fn get_sum_gen<T:Add<Output = T>>(x: T, y: T)->T{
   return x + y;
}

fn main() {
   println!("5 + 4 = {}", get_sum_gen(5,4));
   println!("50000000 + 40000000 = {}", get_sum_gen(500000000,400000000));
   println!("5.1 + 4.1 = {}", get_sum_gen(5.1,4.1));
}
