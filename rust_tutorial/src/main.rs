#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

// we need to access the restaurant directory
// mod restaurant;
// use crate::restaurant::order_food;

fn main() {

   let mut arr_it = [1,2,3,4];
   // you cannot change values with an iterator, it only borrows them from memory
   for val in arr_it.iter(){
      println!("{}", val);
   }

   // here you can consume the collection but not use it
   arr_it.into_iter();
}
