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
    // floating point precision

    let num_1: f32 = 1.111111111111111; // 15 ones
    println!("f32: {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111; // 15 ones
    println!("f64: {}", num_2 + 0.111111111111111);

    // math operators
    let num_3: u32 = 50;
    let num_4: u32 = 4;
    println!("50 + 4 = {}", num_3 + num_4);
    println!("50 - 4 = {}", num_3 - num_4);
    println!("50 * 4 = {}", num_3 * num_4);
    println!("50 / 4 = {}", num_3 / num_4);
    println!("50 % 4 = {}", num_3 % num_4);

    // increment
    let mut inc_num: u32 = 100;
    println!("inc_num = {}", inc_num);
    inc_num += 1;
    println!("inc_num = {}", inc_num);
}
