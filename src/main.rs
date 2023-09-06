#![allow(unused)]

// use standard input/output library
use std::io;
// generate random values
use rand::Rng;
/** Nested importing of libraries
 * these are to be covered later in the
 * lecture
 */
 use std::io::{Write, BufReader, BufRead, ErrorKind};
 use std::fs::File;
 use std::cmp::Ordering;
 

fn main() {
    println!("What is your name?");
    let mut name = String::new(); // mutable string var
    let greeting = "Nice to meet you"; // string var
    io::stdin().read_line(&mut name) // get input from user
        .expect("Didn't Receive Input");

    println!("Hello {}! {}", name.trim_end(), greeting); // print result
}
