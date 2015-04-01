#![allow(unstable)]
#![feature(plugin)]

#[plugin]
#[no_link]
extern crate uax29;

use std::{io, str};

struct Breaks<'a> {
    string: &'a str,
    index: usize,
    bk: fn(&str, usize) -> bool,
}

impl<'a> Iterator for Breaks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.string.len() == self.index {
            None
        } else {
            let old_index: usize = self.index;
            while {
                self.index += 1;
                self.string.len() != self.index &&
                    !(self.bk)(self.string, self.index)
            } {}
            Some(self.string.slice_chars(old_index, self.index))
        }
    }
}

fn main() {
    let hello_world = brainfuck!(
+ + + + + + + + [ > + + + + [ > + + > + + + > + + + > + < < < < - ] > + > + > - > > + [ < ] < - ] > > . > - - - . + + + + + + + . . + + + . > > . < - . < . + + + . - - - - - - . - - - - - - - - . > > + . > + + .
    );
    println!("{}", str::from_utf8(hello_world(&mut io::stdin(), &mut io::stdout()).unwrap().as_slice()).unwrap());
    let mut breaks = Breaks {
        string: "hello world",
        index: 0us,
        bk: {
            fn bk(string: &str, index: usize) -> bool {
                index % 3 == 0
            }
            bk
        }
    };
    for word in breaks {
        println!("word: <{}>", word);
    }
}
