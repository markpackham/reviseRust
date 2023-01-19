#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn sum_list(list: &[i32]) -> i32{
   let mut sum = 0;
   for &val in list.iter(){
      sum += &val;
   } 
   sum
   // also works if you love return and semi colons
   // return sum;
}


fn main() {
   let num_list = vec![1,2,3,4,5];
   println!("Sum of list = {}", sum_list(&num_list));
}
