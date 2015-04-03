#![allow(unstable)]
#![feature(plugin)]

extern crate uax29;

use std::collections::ring_buf;
use std::str;
use uax29::breaks;

struct WordBreaks<'a> {
    tree: Node<breaks::word_break::Category>,
    inner: WordBreaksInner<'a>,
}

struct Node<Category> {
    rules: Vec<(usize, usize, Boundary)>,
    children: Vec<(Category, Node<Category>)>,
}

struct WordBreaksInner<'a> {
    string: &'a str,
    index: usize,
    char_indices: str::CharIndices<'a>,
    future: ring_buf::RingBuf<FutureInfo<breaks::word_break::Category>>,
}

#[derive(Show)]
struct FutureInfo<Category> {
    char_info: Option<CharInfo<Category>>,
    rule_info: Option<RuleInfo>,
}

#[derive(Show)]
struct CharInfo<Category> {
    char_offset: usize,
    category: Category,
}

#[derive(Show)]
struct RuleInfo {
    rule_number: usize,
    boundary: Boundary,
}

#[derive(Copy, PartialEq, Show)]
enum Boundary {
    Break,
    NoBreak,
}

impl<'a> Iterator for WordBreaks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        /*
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
        */
        let old_index: usize = self.inner.index;
        if old_index == self.inner.string.len() {
            None
        } else {
            while {
                self.inner.index += 1;
                self.bk() == Boundary::NoBreak &&
                    self.inner.index != self.inner.string.len()
            } {}
            Some(self.inner.string.slice_chars(old_index, self.inner.index))
        }
    }
}

impl<'a> WordBreaks<'a> {
    fn new(s: &'a str, tree: Node<breaks::word_break::Category>) -> WordBreaks<'a> {
        WordBreaks {
            tree: tree,
            inner: WordBreaksInner {
                string: s,
                index: 0,
                char_indices: s.char_indices(),
                future: ring_buf::RingBuf::new(),
            },
        }
    }

    fn bk(&mut self) -> Boundary {
        self.inner.find_breaks(&self.tree, 0);
        // TODO: These unwraps should always work, but it is possible to write
        // a bad list of rules which does not assign a boundary to a position.
        match self.inner.future.pop_front() {
            None => Boundary::Break,
            Some(fut) => match fut.rule_info {
                None => Boundary::Break,
                Some(rule_info) => rule_info.boundary,
            },
        }
    }
}

impl<'a> WordBreaksInner<'a> {
    fn find_breaks(
        &mut self, node: &Node<breaks::word_break::Category>, offset: usize)
    {
        for &(rule_number, position, boundary) in node.rules.iter() {
            while self.future.len() - offset < position {
                match self.char_indices.next() {
                    None => break,
                    Some((char_offset, char)) =>
                        self.future.push_back(FutureInfo {
                            char_info: Some(CharInfo {
                                char_offset: char_offset,
                                category: breaks::word_break::category(char),
                            }),
                            rule_info: None,
                        }),
                }
            }
            if self.future.get_mut(position + offset).is_none() {
                self.future.push_back(FutureInfo {
                    char_info: None,
                    rule_info: Some(RuleInfo {
                        rule_number: rule_number,
                        boundary: boundary,
                    }),
                });
            } else {
                let fut = self.future.get_mut(position + offset).unwrap();
                match fut.rule_info {
                    Some(ref rule_info) if rule_info.rule_number <= rule_number =>
                        (),
                    _ => fut.rule_info = Some(RuleInfo {
                        rule_number: rule_number,
                        boundary: boundary,
                    }),
                }
            }
        }
        for &(category, ref child) in node.children.iter() {
            let mut should_find_breaks = false;
            {
                match self.future[offset].char_info {
                    None => (),
                    Some(ref char_info) =>
                        should_find_breaks = char_info.category == category,
                }
            }
            if should_find_breaks {
                self.find_breaks(child, offset + 1);
                break;
            }
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
/*
    use uax29::breaks::word_break::Category::*;
    let mut breaks = WordBreaks {
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
*/
    let mut breaks = WordBreaks::new("hello world!\ncrlf:\r\nlfcr:\n\rEND", Node {
        rules: vec![(0, 2, Boundary::NoBreak)],
        children: vec![],
    });
    for word in breaks {
        println!("word: {:?}", word);
    }
}
