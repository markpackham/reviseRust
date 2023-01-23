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

   // a Closure is a function without a name, often stored in a variable

   // Basic closure structure
   // let var_name = |parameters|-> return_type {BODY}

   let can_vote = |age: i32|{
      age >= 18
   };

   println!("Can vote : {}", can_vote(120));

}
