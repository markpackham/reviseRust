#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

fn main() {

   struct Reactangle<T, U>{
      length: T,
      height: U,
   }

   let rec = Reactangle{length:4, height:10.5};
}
