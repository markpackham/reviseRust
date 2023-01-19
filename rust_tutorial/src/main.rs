#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
     enum Day {
      Monday,
      Tuesday,
      Wednesday,
      Thursday,
      Friday,
      Saturday,
      Sunday
  }

  // can define functions for the enum I want to work with
  impl Day {
      fn is_weekend(&self) -> bool {
         // in Rust you will use match as much as you use If statements
          match self {
              Day::Saturday | Day::Sunday => true,
              _ => false
          }
      }
  }

  let today:Day = Day::Monday;

  match today {
   Day::Monday => println!("Everyone hates Monday"),
   Day::Tuesday => println!("Pancake day"),
   Day::Wednesday => println!("Odin day"),
   Day::Thursday => println!("Thor's day"),
   Day::Friday => println!("Almost Weekend"),
   Day::Saturday => println!("Weekend!!!"),
   Day::Sunday => println!("Weekend!!!"),
}

println!("Is today the weekend {}", today.is_weekend());

}
