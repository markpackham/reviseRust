#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

fn main() {
   const PI: f32 = 3.141592;

   trait Shape{
      fn new(length: f32, width: f32) -> Self;
      fn area(&self) -> f32;
   }

   struct Rectangle {length: f32, width: f32};
   struct Circle {length: f32, width: f32};

   // implement (use the Interface of Shape)
   impl Shape for Rectangle{
      fn new(length: f32, width: f32) -> Rectangle{
         return Rectangle{length, width};
      }
      fn area(&self)-> f32{
         return self.length * self.width;
      }
   }

   impl Shape for Circle{
      fn new(length: f32, width: f32) -> Circle{
         return Circle{length, width};
      }
      fn area(&self)-> f32{
         return (self.length/2.0).powf(2.0) * PI;
      }
   }

}
