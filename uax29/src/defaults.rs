use breaks;

use std::collections::ring_buf;
use std::{fmt, str};

pub struct Breaks<'a, Category> {
    tree: Node<Category>,
    index: usize,
    inner: BreaksInner<'a, Category>,
}

#[derive(PartialEq, Show)]
enum NextNode<Category> {
    Child(Node<Category>),
    Loop,
}

#[derive(PartialEq, Show)]
pub struct Node<Category> {
    rules: Vec<(usize, usize, Boundary)>,
    children: Vec<(Category, NextNode<Category>)>,
}

struct BreaksInner<'a, Category> {
    string: &'a str,
    char_indices: str::CharIndices<'a>,
    future: ring_buf::RingBuf<FutureInfo<Category>>,
}

#[derive(PartialEq, Show)]
struct FutureInfo<Category> {
    char_info: Option<CharInfo<Category>>,
    rule_info: Option<RuleInfo>,
}

#[derive(PartialEq, Show)]
struct CharInfo<Category> {
    char_offset: usize,
    ch: char,  // TODO: Remove this: it is just for debugging.
    category: Category,
}

#[derive(PartialEq, Show)]
struct RuleInfo {
    rule_number: usize,
    boundary: Boundary,
}

#[derive(Copy, PartialEq, Show)]
enum Boundary {
    Break,
    NoBreak,
}

#[cfg(test)]
mod test_iterator_for_word_breaks {
    use super::{Boundary, NextNode, Node, Breaks};
    use breaks::word_break::Category::*;

    #[test]
    fn test_next() {
        let tree = Node {
            rules: vec![],
            children: vec![
                (ALetter, NextNode::Child(Node {
                    rules: vec![],
                    children: vec![
                        (Numeric, NextNode::Child(Node {
                            rules: vec![(1, 1, Boundary::NoBreak)],
                            children: vec![],
                        })),
                    ], 
                }),
            )],
        };
        let mut breaks = Breaks::new("a1bc2", tree);
        assert_eq!(breaks.next(), Some("a1"));
        assert_eq!(breaks.next(), Some("b"));
        assert_eq!(breaks.next(), Some("c2"));
        assert_eq!(breaks.next(), None);
    }
}

impl<'a, Category: breaks::FromChar + PartialEq + fmt::Show> Iterator
    for Breaks<'a, Category>
{
    type Item = &'a str;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let old_index: usize = self.index;
        if old_index == self.inner.string.len() {
            None
        } else {
            let mut start_of_segment = true;
            while {
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
                (boundary == Boundary::NoBreak || start_of_segment) &&
                    char_offset != self.inner.string.len()
            } {
                self.inner.pop_front(&self.tree);
                start_of_segment = false;
            }
            if old_index == self.index {
                None
            } else {
                Some(self.inner.string.slice(old_index, self.index))
            }
        }
    }
}

impl<'a, Category: breaks::FromChar + PartialEq + fmt::Show> Breaks<'a, Category> {
    pub fn new(s: &'a str, tree: Node<Category>) -> Breaks<'a, Category> {
        Breaks {
            tree: tree,
            index: 0,
            inner: BreaksInner {
                string: s,
                char_indices: s.char_indices(),
                future: ring_buf::RingBuf::new(),
            },
        }
    }
}

#[cfg(test)]
mod test_word_breaks_inner {
    use std::collections::ring_buf;
    use super::{Boundary, BreaksInner, CharInfo, FutureInfo, NextNode, Node,
                RuleInfo};
    use breaks::word_break::Category;
    use breaks::word_break::Category::*;

    #[test]
    fn test_front() {
        let s: &str = "\u{e9}1bc2";
        let mut inner: BreaksInner<Category> = make_inner(s);
        let future_info: FutureInfo<Category> =
            make_future_info(0, '\u{e9}', ALetter, None);
        let tree: Node<Category> = make_tree();
        assert_eq!(inner.front(&tree), Some(&future_info));
        assert_eq!(inner.front(&tree), Some(&future_info));
    }

    #[test]
    fn test_pop_front() {
        let s: &str = "\u{e9}1bc2";
        let mut inner: BreaksInner<Category> = make_inner(s);
        let tree: Node<Category> = make_tree();
        inner.pop_front(&tree);
        assert_eq!(inner.future.front(),
                   Some(&make_future_info(0, '\u{e9}', ALetter, None)));
        inner.pop_front(&tree);
        assert_eq!(inner.future.front(),
                   Some(&make_future_info(2, '1', Numeric, Some(RuleInfo {
                        rule_number: 1, boundary: Boundary::NoBreak,
                   }))));
    }

    #[test]
    fn test_find_breaks() {
        let s: &str = "\u{e9}'\"\"'1bc2";
        let mut inner: BreaksInner<Category> = make_inner(s);
        let tree: Node<Category> = make_tree();
        let mut buf: ring_buf::RingBuf<FutureInfo<Category>> =
            ring_buf::RingBuf::new();
        buf.push_back(make_future_info(0, '\u{e9}', ALetter, None));
        buf.push_back(make_future_info(2, '\'', Single_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(3, '"', Double_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(4, '"', Double_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(5, '\'', Single_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(6, '1', Numeric, Some(RuleInfo {
            rule_number: 1,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(7, 'b', ALetter, None));
        inner.find_breaks(&tree, 0, 0);
        assert_eq!(inner.future, buf);
    }

    fn make_inner(s: &str) -> BreaksInner<Category> {
        BreaksInner {
            string: s,
            char_indices: s.char_indices(),
            future: ring_buf::RingBuf::new(),
        }
    }

    fn make_future_info(char_offset: usize, ch: char, category: Category,
        rule_info: Option<RuleInfo>)
        -> FutureInfo<Category>
    {
        FutureInfo {
            char_info: Some(CharInfo {
                char_offset: char_offset,
                ch: ch,
                category: category,
            }),
            rule_info: rule_info,
        }
    }

    fn make_tree() -> Node<Category> {
        Node {
            rules: vec![],
            children: vec![
                (ALetter, NextNode::Child(Node {
                    rules: vec![],
                    children: vec![
                        (Single_Quote, NextNode::Loop),
                        (Double_Quote, NextNode::Loop),
                        (Numeric, NextNode::Child(Node {
                            rules: vec![(1, 1, Boundary::NoBreak)],
                            children: vec![],
                        })),
                    ],
                })),
            ],
        }
    }

    #[test]
    fn test_get_enough_chars_for_rule() {
        let mut inner: BreaksInner<Category> = make_inner("\u{e9}bcd");
        let mut buf: ring_buf::RingBuf<FutureInfo<Category>> =
            ring_buf::RingBuf::new();
        buf.push_back(make_future_info(0, '\u{e9}', ALetter, None));
        buf.push_back(make_future_info(2, 'b', ALetter, None));
        inner.get_enough_chars_for_rule(1);
        assert_eq!(inner.future, buf);
    }

    #[test]
    fn test_add_rule_to_future_none() {
        let mut inner: BreaksInner<Category> = make_inner("\u{e9}bcd");
        let mut buf: ring_buf::RingBuf<FutureInfo<Category>> =
            ring_buf::RingBuf::new();
        buf.push_back(FutureInfo {
            char_info: None,
            rule_info: Some(RuleInfo{
                rule_number: 123,
                boundary: Boundary::Break,
            }),
        });
        inner.add_rule_to_future(123, 0, Boundary::Break);
        assert_eq!(inner.future, buf);
    }

    #[test]
    fn test_add_rule_to_future_some() {
        let mut inner: BreaksInner<Category> = make_inner("\u{e9}bcd");
        let mut buf: ring_buf::RingBuf<FutureInfo<Category>> =
            ring_buf::RingBuf::new();
        buf.push_back(make_future_info(0, '\u{e9}', ALetter, None));
        buf.push_back(make_future_info(2, 'b', ALetter, Some(RuleInfo{
            rule_number: 123,
            boundary: Boundary::Break,
        })));
        inner.get_enough_chars_for_rule(1);
        inner.add_rule_to_future(123, 1, Boundary::Break);
        assert_eq!(inner.future, buf);
    }
}

impl<'a, Category: breaks::FromChar + PartialEq + fmt::Show>
    BreaksInner<'a, Category>
{
    fn front(&mut self, node: &Node<Category>) -> Option<&FutureInfo<Category>> {
        if self.future.is_empty() {
            self.find_breaks(node, 0, 0);
        }
        self.future.front()
    }

    fn pop_front(&mut self, node: &Node<Category>) {
        self.future.pop_front();
        self.find_breaks(node, 0, 0);
    }

    fn find_breaks(
        &mut self, node: &Node<Category>, offset: usize, loops: usize) {
/*
        println!("rules: {:?}", node.rules);
        println!("kids: {:?}", node.children);
        println!("offset: {}", offset);
        println!("future: {:?}", self.future);
        println!("loops: {}", loops);
*/
        for &(rule_number, position, boundary) in node.rules.iter() {
            self.get_enough_chars_for_rule(position + loops);
            self.add_rule_to_future(rule_number, position + loops, boundary);
        }
        self.get_enough_chars_for_children(offset);
        self.handle_children(node, offset, loops);
    }

    fn get_enough_chars_for_rule(&mut self, position: usize) {
        while self.future.len() <= position {
            match self.char_indices.next() {
                None => break,
                Some((char_offset, char)) =>
                    self.future.push_back(FutureInfo {
                        char_info: Some(CharInfo {
                            char_offset: char_offset,
                            ch: char,
                            category: breaks::FromChar::from_char(char),
                        }),
                        rule_info: None,
                    }),
            }
        }
    }

    fn get_enough_chars_for_children(&mut self, offset: usize) {
        while self.future.len() <= offset {
            self.future.push_back(FutureInfo {
                char_info: match self.char_indices.next() {
                    None => Some(CharInfo {
                        char_offset: self.string.len(),
                        // These will not be used and don't matter:
                        ch: 'x',
                        category: breaks::FromChar::from_char('x'),
                    }),
                    Some((char_offset, char)) => Some(CharInfo {
                        char_offset: char_offset,
                        ch: char,
                        category: breaks::FromChar::from_char(char),
                    }),
                },
                rule_info: None,
            });
        }
    }

    fn add_rule_to_future(
        &mut self, rule_number: usize, position: usize, boundary: Boundary)
    {
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

    fn handle_children(&mut self, node: &Node<Category>, offset: usize,
                       loops: usize)
    {
        for &(ref category, ref child) in node.children.iter() {
            let mut should_find_breaks = false;
            {
                match self.future[offset].char_info {
                    None => (),
                    Some(ref char_info) =>
                        should_find_breaks = char_info.category == *category,
                }
            }
            if should_find_breaks {
                let (node, loops) = match *child {
                    NextNode::Child(ref node) => (node, loops),
                    NextNode::Loop => {
                        self.future[offset].rule_info = Some(RuleInfo {
                            rule_number: 0,
                            boundary: Boundary::NoBreak,
                        });
                        (node, loops + 1)
                    },
                };
                self.find_breaks(node, offset + 1, loops);
                break;
            }
        }
    }
}

#[cfg(test)]
pub fn make_word_break_tree() -> Node<breaks::word_break::Category> {
    use breaks::word_break::Category;
    Node {
        rules: vec![],
        children: vec![
            (Category::CR, NextNode::Child(Node {
                rules: vec![(3101, 1, Boundary::Break),
                            (3201, 0, Boundary::Break)],
                children: vec![
                    (Category::LF, NextNode::Child(Node {
                        rules: vec![(3000, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
            (Category::Newline, NextNode::Child(Node {
                rules: vec![(3100, 1, Boundary::Break),
                            (3200, 0, Boundary::Break)],
                children: vec![],
            })),
            (Category::LF, NextNode::Child(Node {
                rules: vec![(3102, 1, Boundary::Break),
                            (3202, 0, Boundary::Break)],
                children: vec![],
            })),
            (Category::ALetter, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::ALetter, NextNode::Child(Node {
                        rules: vec![(5000, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::Hebrew_Letter, NextNode::Child(Node {
                        rules: vec![(5001, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::MidLetter, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                            (Category::Extend, NextNode::Loop),
                            (Category::Format, NextNode::Loop),
                            (Category::ALetter, NextNode::Child(Node {
                                rules: vec![(6000, 1, Boundary::NoBreak),
                                            (7000, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(6001, 1, Boundary::NoBreak),
                                            (7001, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::MidNumLet, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                            (Category::Extend, NextNode::Loop),
                            (Category::Format, NextNode::Loop),
                            (Category::ALetter, NextNode::Child(Node {
                                rules: vec![(6002, 1, Boundary::NoBreak),
                                            (7002, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(6003, 1, Boundary::NoBreak),
                                            (7003, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::Single_Quote, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::ALetter, NextNode::Child(Node {
                                rules: vec![(6004, 1, Boundary::NoBreak),
                                            (7004, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(6005, 1, Boundary::NoBreak),
                                            (7005, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::Numeric, NextNode::Child(Node {
                        rules: vec![(9000, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::ExtendNumLet, NextNode::Child(Node {
                        rules: vec![(13100, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
            (Category::Hebrew_Letter, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::ALetter, NextNode::Child(Node {
                        rules: vec![(502, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::ALetter, NextNode::Child(Node {
                        rules: vec![(503, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::MidLetter, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::ALetter, NextNode::Child(Node {
                                rules: vec![(6006, 1, Boundary::NoBreak),
                                            (7006, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(6007, 1, Boundary::NoBreak),
                                            (7007, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::MidNumLet, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::ALetter, NextNode::Child(Node {
                                rules: vec![(6008, 1, Boundary::NoBreak),
                                            (7008, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(6009, 1, Boundary::NoBreak),
                                            (7009, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::Single_Quote, NextNode::Child(Node {
                        rules: vec![(7100, 1, Boundary::NoBreak)],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::ALetter, NextNode::Child(Node {
                                rules: vec![(6010, 1, Boundary::NoBreak),
                                            (7010, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(6011, 1, Boundary::NoBreak),
                                            (7011, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::Double_Quote, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::Hebrew_Letter, NextNode::Child(Node {
                                rules: vec![(7200, 1, Boundary::NoBreak),
                                            (7300, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::Numeric, NextNode::Child(Node {
                        rules: vec![(9001, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::ExtendNumLet, NextNode::Child(Node {
                        rules: vec![(13101, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
            (Category::Numeric, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::Numeric, NextNode::Child(Node {
                        rules: vec![(8000, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::ALetter, NextNode::Child(Node {
                        rules: vec![(10000, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::Hebrew_Letter, NextNode::Child(Node {
                        rules: vec![(10001, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::MidNum, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::Numeric, NextNode::Child(Node {
                                rules: vec![(11000, 1, Boundary::NoBreak),
                                            (12000, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::MidNumLet, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                        (Category::Extend, NextNode::Loop),
                        (Category::Format, NextNode::Loop),
                            (Category::Numeric, NextNode::Child(Node {
                                rules: vec![(11001, 1, Boundary::NoBreak),
                                            (12001, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::Single_Quote, NextNode::Child(Node {
                        rules: vec![],
                        children: vec![
                            (Category::Extend, NextNode::Loop),
                            (Category::Format, NextNode::Loop),
                            (Category::Numeric, NextNode::Child(Node {
                                rules: vec![(11003, 1, Boundary::NoBreak),
                                            (12003, 2, Boundary::NoBreak)],
                                children: vec![],
                            })),
                        ],
                    })),
                    (Category::ExtendNumLet, NextNode::Child(Node {
                        rules: vec![(13102, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
            (Category::Katakana, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::Katakana, NextNode::Child(Node {
                        rules: vec![(13000, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::ExtendNumLet, NextNode::Child(Node {
                        rules: vec![(13103, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
            (Category::ExtendNumLet, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::ExtendNumLet, NextNode::Child(Node {
                        rules: vec![(13104, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::ALetter, NextNode::Child(Node {
                        rules: vec![(13200, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::Hebrew_Letter, NextNode::Child(Node {
                        rules: vec![(13201, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::Numeric, NextNode::Child(Node {
                        rules: vec![(13202, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::Katakana, NextNode::Child(Node {
                        rules: vec![(13203, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
            (Category::Regional_Indicator, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::Regional_Indicator, NextNode::Child(Node {
                        rules: vec![(13300, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                ],
            })),
/*
            (Category::Other, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::Other, NextNode::Child(Node {
                        rules: vec![(14000, 1, Boundary::Break)],
                        children: vec![],
                    })),
                ],
            })),
*/
        ],
    }
}
