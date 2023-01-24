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
   
   pub struct Bank{
      balance: f32
   }
   fn withdraw(the_bank: &mut Bank, amt:f32){
      the_bank.balance -= amt;
   }
}
