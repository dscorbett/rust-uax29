#![allow(unstable)]
#![feature(plugin)]

#[plugin]
#[no_link]
extern crate uax29;

use std::{io, str};

fn main() {
    println!("macro:");
    let hello_world = brainfuck!(
+ + + + + + + + [ > + + + + [ > + + > + + + > + + + > + < < < < - ] > + > + > - > > + [ < ] < - ] > > . > - - - . + + + + + + + . . + + + . > > . < - . < . + + + . - - - - - - . - - - - - - - - . > > + . > + + .
    );
    println!("call:");
    println!("{}", str::from_utf8(hello_world(&mut io::stdin(), &mut io::stdout()).unwrap().as_slice()).unwrap());
}
