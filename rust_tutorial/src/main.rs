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

   let mut bank = Bank{balance: 100.0};
   withdraw(&mut bank, 5.00);
   println!("Balance : {}", bank.balance);

   // demo cocurrency via customers withdrawing money
   // use threads to show money being drawn in more realistic times
   fn customer(the_bank: &mut Bank){
      withdraw(the_bank, 5.00);
   }

   thread::spawn(||{
      customer(&mut bank)
   }).join().unwrap();

   // In Rust a closure must not outlive the current function
}
