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
   write!(output, "Just some\nRandom words").expect("Failed to write to file");

   let input = File::open(path).unwrap();
   let buffered = BufReader::new(input);
   for line in buffered.lines(){
      println!("{}", line.unwrap());
   }

   let output2 = File::create("rand.txt");
   let output2 = match output2{
      Ok(file)=>file,
      // catch specific error
      Err(error)=>match error.kind(){
         ErrorKind::NotFound => match File::create("rand.txt"){
            Ok(fc) => fc,
            Err(e)=> panic!("Cannot create file: {:?}", error),
         },
         _other_error => panic!("Problem opening file: {:?}", error),
      },
   };
}
