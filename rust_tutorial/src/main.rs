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
   // order_food();

   let path = "lines.txt";
   let output = File::create(path);
   let mut output = match output {
      Ok(file) => file,
      Err(error) => {
         panic!("Problem creating file {:?}", error);
      }
   };
}
