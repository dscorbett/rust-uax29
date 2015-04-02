#![allow(unstable)]
#![feature(plugin)]

extern crate uax29;

use std::{io, str};
use uax29::breaks;

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
/*
    let hello_world = brainfuck!(
+ + + + + + + + [ > + + + + [ > + + > + + + > + + + > + < < < < - ] > + > + > - > > + [ < ] < - ] > > . > - - - . + + + + + + + . . + + + . > > . < - . < . + + + . - - - - - - . - - - - - - - - . > > + . > + + .
    );
    println!("{}", str::from_utf8(hello_world(&mut io::stdin(), &mut io::stdout()).unwrap().as_slice()).unwrap());
*/
    use uax29::breaks::word_break;
    use uax29::breaks::word_break::Category::*;
    println!("{:?}", word_break::category('\r'));
    let mut breaks = Breaks {
        string: "hello\rworld\n\rhello\r\nworld",
        index: 0us,
        bk: {
            fn bk(string: &str, index: usize) -> bool {
                if index == 0 { return true; }
                if index == string.len() { return true; }

                let left1 = word_break::category(string.char_at(index - 1));
                let right1 = word_break::category(string.char_at(index));
                if left1 == Some(CR) && right1 == Some(LF) { return false; }
                if left1 == Some(Newline) || left1 == Some(CR) ||
                    left1 == Some(LF) { return true; }
                if right1 == Some(Newline) || right1 == Some(CR) ||
                    right1 == Some(LF) { return true; }
                // TODO: rewriting with ->
                if left1 == Some(ALetter) && right1 == Some(ALetter) {
                    return false;
                }

                // TODO: check bounds
                let left2 = word_break::category(string.char_at(index));
                if left2 == Some(Numeric) && (left1 == Some(MidNum) ||
                                              left1 == Some(MidNumLet) ||
                                              left1 == Some(Single_Quote)) &&
                    right1 == Some(Numeric) { return false; }
                true
            }
            bk
        }
    };
    for word in breaks {
        println!("word: {:?}", word);
    }
}
