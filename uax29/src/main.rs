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
    let mut breaks = WordBreaks::new("x:y", Node {
        rules: vec![],
        children: vec![
            (CR, Node {
                rules: vec![],
                children: vec![(LF, Node {
                    rules: vec![(3000, 1, NoBreak)],
                    children: vec![],
                })],
            }),
            (Newline, Node {
                rules: vec![(3100, 1, Break)],
                children: vec![],
            }),
            (CR, Node {
                rules: vec![(3101, 1, Break)],
                children: vec![],
            }),
            (LF, Node {
                rules: vec![(3102, 1, Break)],
                children: vec![],
            }),
            (Newline, Node {
                rules: vec![(3200, 0, Break)],
                children: vec![],
            }),
            (CR, Node {
                rules: vec![(3201, 0, Break)],
                children: vec![],
            }),
            (LF, Node {
                rules: vec![(3202, 0, Break)],
                children: vec![],
            }),
            // TODO: WB4
            (ALetter, Node {
                rules: vec![],
                children: vec![
                    (ALetter, Node {
                        rules: vec![(5000, 1, NoBreak)],
                        children: vec![],
                    }),
                    (Hebrew_Letter, Node {
                        rules: vec![(5001, 1, NoBreak)],
                        children: vec![],
                    }),
                    (MidLetter, Node {
                        rules: vec![],
                        children: vec![
                            (ALetter, Node {
                                rules: vec![(6000, 1, NoBreak),
                                            (7000, 2, NoBreak)],
                                children: vec![],
                            }),
                            (Hebrew_Letter, Node {
                                rules: vec![(6001, 1, NoBreak),
                                            (7001, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (MidNumLet, Node {
                        rules: vec![],
                        children: vec![
                            (ALetter, Node {
                                rules: vec![(6002, 1, NoBreak),
                                            (7002, 2, NoBreak)],
                                children: vec![],
                            }),
                            (Hebrew_Letter, Node {
                                rules: vec![(6003, 1, NoBreak),
                                            (7003, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (Single_Quote, Node {
                        rules: vec![],
                        children: vec![
                            (ALetter, Node {
                                rules: vec![(6004, 1, NoBreak),
                                            (7004, 2, NoBreak)],
                                children: vec![],
                            }),
                            (Hebrew_Letter, Node {
                                rules: vec![(6005, 1, NoBreak),
                                            (7005, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (Numeric, Node {
                        rules: vec![(9000, 1, NoBreak)],
                        children: vec![],
                    }),
                    (ExtendNumLet, Node {
                        rules: vec![(13100, 1, NoBreak)],
                        children: vec![],
                    }),
                ],
            }),
            (Hebrew_Letter, Node {
                rules: vec![],
                children: vec![
                    (ALetter, Node {
                        rules: vec![(502, 1, NoBreak)],
                        children: vec![],
                    }),
                    (ALetter, Node {
                        rules: vec![(503, 1, NoBreak)],
                        children: vec![],
                    }),
                    (MidLetter, Node {
                        rules: vec![],
                        children: vec![
                            (ALetter, Node {
                                rules: vec![(6006, 1, NoBreak),
                                            (7006, 2, NoBreak)],
                                children: vec![],
                            }),
                            (Hebrew_Letter, Node {
                                rules: vec![(6007, 1, NoBreak),
                                            (7007, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (MidNumLet, Node {
                        rules: vec![],
                        children: vec![
                            (ALetter, Node {
                                rules: vec![(6008, 1, NoBreak),
                                            (7008, 2, NoBreak)],
                                children: vec![],
                            }),
                            (Hebrew_Letter, Node {
                                rules: vec![(6009, 1, NoBreak),
                                            (7009, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (Single_Quote, Node {
                        rules: vec![(7100, 1, NoBreak)],
                        children: vec![
                            (ALetter, Node {
                                rules: vec![(6010, 1, NoBreak),
                                            (7010, 2, NoBreak)],
                                children: vec![],
                            }),
                            (Hebrew_Letter, Node {
                                rules: vec![(6011, 1, NoBreak),
                                            (7011, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (Double_Quote, Node {
                        rules: vec![],
                        children: vec![
                            (Hebrew_Letter, Node {
                                rules: vec![(7200, 1, NoBreak),
                                            (7300, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (Numeric, Node {
                        rules: vec![(9001, 1, NoBreak)],
                        children: vec![],
                    }),
                    (ExtendNumLet, Node {
                        rules: vec![(13101, 1, NoBreak)],
                        children: vec![],
                    }),
                ],
            }),
            (Numeric, Node {
                rules: vec![],
                children: vec![
                    (Numeric, Node {
                        rules: vec![(8000, 1, NoBreak)],
                        children: vec![],
                    }),
                    (ALetter, Node {
                        rules: vec![(10000, 1, NoBreak)],
                        children: vec![],
                    }),
                    (Hebrew_Letter, Node {
                        rules: vec![(10001, 1, NoBreak)],
                        children: vec![],
                    }),
                    (MidNum, Node {
                        rules: vec![],
                        children: vec![
                            (Numeric, Node {
                                rules: vec![(11000, 1, NoBreak),
                                            (12000, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (MidNumLet, Node {
                        rules: vec![],
                        children: vec![
                            (Numeric, Node {
                                rules: vec![(11001, 1, NoBreak),
                                            (12001, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (Single_Quote, Node {
                        rules: vec![],
                        children: vec![
                            (Numeric, Node {
                                rules: vec![(11003, 1, NoBreak),
                                            (12003, 2, NoBreak)],
                                children: vec![],
                            }),
                        ],
                    }),
                    (ExtendNumLet, Node {
                        rules: vec![(13102, 1, NoBreak)],
                        children: vec![],
                    }),
                ],
            }),
            (Katakana, Node {
                rules: vec![],
                children: vec![
                    (Katakana, Node {
                        rules: vec![(13000, 1, NoBreak)],
                        children: vec![],
                    }),
                    (ExtendNumLet, Node {
                        rules: vec![(13103, 1, NoBreak)],
                        children: vec![],
                    }),
                ],
            }),
            (ExtendNumLet, Node {
                rules: vec![],
                children: vec![
                    (ExtendNumLet, Node {
                        rules: vec![(13104, 1, NoBreak)],
                        children: vec![],
                    }),
                    (ALetter, Node {
                        rules: vec![(13200, 1, NoBreak)],
                        children: vec![],
                    }),
                    (Hebrew_Letter, Node {
                        rules: vec![(13201, 1, NoBreak)],
                        children: vec![],
                    }),
                    (Numeric, Node {
                        rules: vec![(13202, 1, NoBreak)],
                        children: vec![],
                    }),
                    (Katakana, Node {
                        rules: vec![(13203, 1, NoBreak)],
                        children: vec![],
                    }),
                ],
            }),
            (Regional_Indicator, Node {
                rules: vec![],
                children: vec![
                    (Regional_Indicator, Node {
                        rules: vec![(13300, 1, NoBreak)],
                        children: vec![],
                    })
                ],
            }),
            // WB14 is implicit.
        ],
    });
    for word in breaks {
        println!("word: {:?}", word);
    }
}
