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
    // mathc
    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote");
        Ordering::Greater => println!("Can Vote");
        Ordering::Equal => println!("You gained the right to vote");
    }
}
