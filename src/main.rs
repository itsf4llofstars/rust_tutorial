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
    // ternary conditional
    let mut my_age = 50;

    // no semi-colon needed for the return statements
    // semi-colon needed at last curly brace
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };

    println!("Can Vot: {}", can_vote);
}
