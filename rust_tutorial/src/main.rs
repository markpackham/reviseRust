#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

fn main() {

    // A box is used when you have a large amount of data stored
    // on the heap and then you pass pointers to it on the stack.

    let box_int1 = Box::new(10);
    println!("b_int1 = {}", box_int1);
}
