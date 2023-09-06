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
    // random numbers
    let random_num = rand::thread_rng().gen_range(1..101); // non-inclusive
    println!("Random: {}", random_num);
}
