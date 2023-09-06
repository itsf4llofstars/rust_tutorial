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
    // Conditionals
    let age = 50;

    if (age >= 1) && (age <=18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if (age >= 65) {
        println!("Important Birthday");
    } else {
        println!("Not Important Birthday");
    }

}
