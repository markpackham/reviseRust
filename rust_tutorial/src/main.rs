#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
   
thread::spawn(move || {
   for i in 1..15{
      println!("Spawned thread: {}", i);
      thread::sleep(Duration::from_millis(1));
   }
});

for i in 1..5{
   println!("Main thread: {}", i);
   thread::sleep(Duration::from_millis(1));
}


}
