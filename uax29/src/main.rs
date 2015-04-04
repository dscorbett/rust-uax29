#![allow(unstable)]
#![feature(plugin)]

extern crate uax29;

use std::collections::ring_buf;
use std::str;
use uax29::breaks;

struct WordBreaks<'a> {
    tree: Node<breaks::word_break::Category>,
    index: usize,
    inner: WordBreaksInner<'a>,
}

#[derive(Show)]
struct Node<Category> {
    rules: Vec<(usize, usize, Boundary)>,
    children: Vec<(Category, Node<Category>)>,
}

struct WordBreaksInner<'a> {
    string: &'a str,
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
    ch: char,  // TODO: Remove this: it is just for debugging.
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
        let old_index: usize = self.index;
        if old_index == self.inner.string.len() {
            None
        } else {
            let mut start_of_word = true;
            while {
//                println!("\nfut: {:?}", self.inner.future);
                let (boundary, char_offset): (Boundary, usize) =
                    match self.inner.front(&self.tree) {
                        None => (Boundary::Break, self.index + 1),
                        Some(fut) => (match fut.rule_info {
                            None => Boundary::Break,
                            Some(ref rule_info) => rule_info.boundary,
                        }, match fut.char_info {
                            None => self.index + 1,
                            Some(ref char_info) => char_info.char_offset,
                        }),
                    };
                self.index = char_offset;
                boundary == Boundary::NoBreak || start_of_word
            } {
                self.inner.pop_front(&self.tree);
                start_of_word = false;
            }
            if old_index == self.index {
                None
            } else {
                Some(self.inner.string.slice(old_index, self.index))
            }
        }
    }
}

impl<'a> WordBreaks<'a> {
    fn new(s: &'a str, tree: Node<breaks::word_break::Category>) -> WordBreaks<'a> {
        WordBreaks {
            tree: tree,
            index: 0,
            inner: WordBreaksInner {
                string: s,
                char_indices: s.char_indices(),
                future: ring_buf::RingBuf::new(),
            },
        }
    }
}

impl<'a> WordBreaksInner<'a> {
    fn front(&mut self, node: &Node<breaks::word_break::Category>) -> Option<&FutureInfo<breaks::word_break::Category>> {
        if self.future.is_empty() {
            self.find_breaks(node, 0);
        }
        self.future.front()
    }

    fn pop_front(&mut self, node: &Node<breaks::word_break::Category>) {
        self.future.pop_front();
        self.find_breaks(node, 0);
    }

    fn find_breaks(
        &mut self, node: &Node<breaks::word_break::Category>, offset: usize)
    {
/*
        println!("rules: {:?}", node.rules);
        println!("kids: {:?}", node.children);
        println!("offset: {}", offset);
        println!("future: {:?}", self.future);
        println!("offset: {}", offset);
*/
        for &(rule_number, position, boundary) in node.rules.iter() {
//            println!("rule: {}\npos: {}\nbnd: {:?}", rule_number, position, boundary);
            while self.future.len() <= position {
                match self.char_indices.next() {
                    None => break,
                    Some((char_offset, char)) =>
                        self.future.push_back(FutureInfo {
                            char_info: Some(CharInfo {
                                char_offset: char_offset,
                                ch: char,
                                category: breaks::word_break::category(char),
                            }),
                            rule_info: None,
                        }),
                }
            }
            if self.future.get_mut(position).is_none() {
                self.future.push_back(FutureInfo {
                    char_info: None,
                    rule_info: Some(RuleInfo {
                        rule_number: rule_number,
                        boundary: boundary,
                    }),
                });
            } else {
                let fut = self.future.get_mut(position).unwrap();
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
        while self.future.len() <= offset {
            self.future.push_back(FutureInfo {
                char_info: match self.char_indices.next() {
                    None => {
None},
                    Some((char_offset, char)) => Some(CharInfo {
                        char_offset: char_offset,
                        ch: char,
                        category: breaks::word_break::category(char),
                    }),
                },
                rule_info: None,
            });
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
    use uax29::breaks::word_break;
    use uax29::breaks::word_break::Category::*;
    use Boundary::*;
    let mut breaks = WordBreaks::new("ab\r\ncd\n\u{e9}f\r", Node {
        rules: vec![],
        children: vec![
            (CR, Node {
                rules: vec![],
                children: vec![(LF, Node {
                    rules: vec![(300, 1, NoBreak)],
                    children: vec![],
                })],
            }),
            (Newline, Node {
                rules: vec![(310, 1, Break)],
                children: vec![],
            }),
            (CR, Node {
                rules: vec![(311, 1, Break)],
                children: vec![],
            }),
            (LF, Node {
                rules: vec![(312, 1, Break)],
                children: vec![],
            }),
            (Newline, Node {
                rules: vec![(320, 0, Break)],
                children: vec![],
            }),
            (CR, Node {
                rules: vec![(321, 0, Break)],
                children: vec![],
            }),
            (LF, Node {
                rules: vec![(322, 0, Break)],
                children: vec![],
            }),
            // TODO: WB4
            (ALetter, Node {
                rules: vec![],
                children: vec![(ALetter, Node {
                    rules: vec![(500, 1, NoBreak)],
                    children: vec![],
                })],
            }),
            (ALetter, Node {
                rules: vec![],
                children: vec![(Hebrew_Letter, Node {
                    rules: vec![(501, 1, NoBreak)],
                    children: vec![],
                })],
            }),
            (Hebrew_Letter, Node {
                rules: vec![],
                children: vec![(ALetter, Node {
                    rules: vec![(502, 1, NoBreak)],
                    children: vec![],
                })],
            }),
            (Hebrew_Letter, Node {
                rules: vec![],
                children: vec![(ALetter, Node {
                    rules: vec![(503, 1, NoBreak)],
                    children: vec![],
                })],
            }),
        ],
    });
    for word in breaks {
        println!("word: {:?}", word);
    }
}
