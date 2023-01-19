#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn get_sum(x: i32, y: i32){
   println!("{} + {} = {}", x, y, x+y);
}

fn get_sum2(x: i32, y: i32) -> i32{
   //x + y
   return x + y;
}

fn main() {
   //get_sum(1,2);
   println!("{}",get_sum2(10, 11));

}
