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
    /** use let _is_true (underscore pre-pended)
     * to prevent rust compiler from erroring on
     * unused variables when we are not using
     * #![allow(unused)]
     */
    let is_true = true; // false

    let my_grade = 'A'; // char
}
