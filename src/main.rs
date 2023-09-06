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
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14159;
    let age = "47";
    let mut age: u32 = age.trim().parse() // shadowing age: &str
        .expect("Age wasn't assigned an number");
    age = age + 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);
}
