#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
 let arr_1 = [1, 2, 3, 4, 5];
 println!("1st: {}", arr_1[0]);
 println!("Length: {}", arr_1.len());

 let arr_2 = [1,2,3,4,5,6,7,8,9,10,11];
 let mut loop_idx = 0;
 loop {
   if arr_2[loop_idx] % 2 == 0{
      loop_idx += 1;
      continue;
   }
   if arr_2[loop_idx] == 9{
      break;
   }
   println!("Val : {}", arr_2[loop_idx]);
   loop_idx +=1;
 }
}
