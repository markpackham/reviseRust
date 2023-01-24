#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

fn main() {

    struct TreeNode<T> {
      pub left: TreeNode<T>,
      pub right: TreeNode<T>,
      pub key: T,
    }
}
