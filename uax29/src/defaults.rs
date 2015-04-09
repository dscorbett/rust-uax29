//! Breaking according to UAX #29
//!
//! Unicode Standard Annex #29 (Unicode Text Segmentation) specifies how
//! to split a string into graphemes, words, sentences, or other text
//! elements. This module provides an iterator over a string by text
//! elements.
//!
//! For example, this code creates an iterator over words.
//!
//! ```rust
//! let mut breaks = uax29::defaults::Breaks::new(
//!     "1.21 gigawatts.", uax29::defaults::make_word_break_tree());
//! ```
//!
//! This would produce the string slices "1.21", " ", "gigawatts", and
//! ".".
//!
//! `make_word_break_tree` creates a representation of the default
//! word-breaking specification. Clients may create their own
//! specifications using `Node`, `NextNode`, and `Boundary` (q.v.).
//! There is no explicit representation of the pseudo-property value
//! Any, nor of ignore rules. This is merely an inconvenience: every
//! specification can be rewritten to exclude them.

use breaks;

use std::collections::ring_buf;
use std::{fmt, str};

/// An iterator over the pieces of a string broken in accordance with
/// UAX #29.
pub struct Breaks<'a, Category> {
    tree: Node<Category>,
    index: usize,
    inner: BreaksInner<'a, Category>,
}

/// A tree of states.
///
/// `rules` is a vector of rules to apply, where a rule is a tuple of
/// the number of the rule (where lower numbers have higher precedence),
/// the index of the character in this rule that the boundary precedes,
/// and the boundary type.
///
/// For example, the rule X ÷ Y Z with number 25 would be `(25, 1,
/// Boundary::Break)`, and X Y × Z with number 26 would be `(26, 2,
/// Boundary::NoBeak)`.
///
/// `children` is a vector of tuples of categories and the child states
/// they lead to. For example, the state corresponding to X in X ÷ Y
/// would have a child `(Y, y_node)`, where `y_node` is the state node
/// corresponding to Y.
///
/// A node can have an edge to itself, so this is not really a tree.
/// It is, however, nearly a tree, since arbitrary loops are not
/// allowed.
#[derive(Clone, PartialEq, Show)]
pub struct Node<Category> {
    rules: Vec<(usize, usize, Boundary)>,
    children: Vec<(Category, NextNode<Category>)>,
}

/// The type of connection to a child node.
#[derive(Clone, PartialEq, Show)]
pub enum NextNode<Category> {
    /// A connection to another node (the usual case).
    Child(Node<Category>),
    /// A self-loop. Other kinds of loops are impossible.
    Loop,
}

/// The inner part of `Breaks`.
///
/// This only exists as an implementation detail. There is no principled
/// reason this shouldn't be part of `Breaks`.
// The reason it can't be part of `Breaks` is that `next` needs both
// `index` and `string` at the same time, and the borrow checker didn't
// allow it.
struct BreaksInner<'a, Category> {
    string: &'a str,
    char_indices: str::CharIndices<'a>,
    future: ring_buf::RingBuf<FutureInfo<Category>>,
}

/// Information about a future character: whether it is skipped by
/// rewrite rules (e.g. WB4), its position in the string and its break
/// property value (if known), and whether to break before it (if known).
#[derive(PartialEq, Show)]
struct FutureInfo<Category> {
    skip: bool,
    char_info: Option<CharInfo<Category>>,
    rule_info: Option<RuleInfo>,
}

/// Information about a character: its position in a string and its
/// break property value.
///
/// The fake "end of text" character gets a `char_offset` of the length
/// of the string and a `category` of None.
#[derive(PartialEq, Show)]
struct CharInfo<Category> {
    char_offset: usize,
    category: Option<Category>,
}

/// Information about the boundary before a character: what rule applies
/// and the boundary type.
#[derive(PartialEq, Show)]
struct RuleInfo {
    rule_number: usize,
    boundary: Boundary,
}

/// A boundary type.
#[derive(Clone, Copy, PartialEq, Show)]
pub enum Boundary {
    /// A break before a character.
    Break,
    /// A non-break before a character.
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

impl<'a, Category: breaks::FromChar + PartialEq + fmt::Show> Breaks<'a, Category>
{
    /// Constructs a new `Breaks`.
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
            make_future_info(0, false, ALetter, None);
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
                   Some(&make_future_info(0, false, ALetter, None)));
        inner.pop_front(&tree);
        assert_eq!(inner.future.front(),
                   Some(&make_future_info(2, false, Numeric, Some(RuleInfo {
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
        buf.push_back(make_future_info(0, false, ALetter, None));
        buf.push_back(make_future_info(2, true, Single_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(3, true, Double_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(4, true, Double_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(5, true, Single_Quote, Some(RuleInfo {
            rule_number: 0,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(6, false, Numeric, Some(RuleInfo {
            rule_number: 1,
            boundary: Boundary::NoBreak,
        })));
        buf.push_back(make_future_info(7, false, ALetter, None));
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

    fn make_future_info(char_offset: usize, skip: bool, category: Category,
        rule_info: Option<RuleInfo>) -> FutureInfo<Category>
    {
        FutureInfo {
            skip: skip,
            char_info: Some(CharInfo {
                char_offset: char_offset,
                category: Some(category),
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
        buf.push_back(make_future_info(0, false, ALetter, None));
        buf.push_back(make_future_info(2, false, ALetter, None));
        inner.get_enough_chars_for_rule(1);
        assert_eq!(inner.future, buf);
    }

/*
    #[test]
    fn test_add_rule_to_future_none() {
        let mut inner: BreaksInner<Category> = make_inner("\u{e9}bcd");
        let mut buf: ring_buf::RingBuf<FutureInfo<Category>> =
            ring_buf::RingBuf::new();
        buf.push_back(FutureInfo {
            skip: false,
            char_info: None,
            rule_info: Some(RuleInfo{
                rule_number: 123,
                boundary: Boundary::Break,
            }),
        });
        inner.add_rule_to_future(123, 0, Boundary::Break);
        assert_eq!(inner.future, buf);
    }
*/

    #[test]
    fn test_add_rule_to_future_some() {
        let mut inner: BreaksInner<Category> = make_inner("\u{e9}bcd");
        let mut buf: ring_buf::RingBuf<FutureInfo<Category>> =
            ring_buf::RingBuf::new();
        buf.push_back(make_future_info(0, false, ALetter, None));
        buf.push_back(make_future_info(2, false, ALetter, Some(RuleInfo{
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
    /// Provides a reference to the front element.
    ///
    /// If the sequence is empty, it tries to get more elements first.
    /// If there are no more elements to be gotten, returns `None`.
    fn front(&mut self, node: &Node<Category>) -> Option<&FutureInfo<Category>> {
        if self.future.is_empty() {
            self.find_breaks(node, 0, 0);
        }
        self.future.front()
    }

    /// Removes the first element and gets more elements.
    fn pop_front(&mut self, node: &Node<Category>) {
        self.future.pop_front();
        self.find_breaks(node, 0, 0);
    }

    /// Gets all elements corresponding to future characters whose
    /// breaking status might depend on the current character.
    fn find_breaks(
        &mut self, node: &Node<Category>, offset: usize, loops: usize) {
        for &(rule_number, position, boundary) in node.rules.iter() {
            self.get_enough_chars_for_rule(position + loops);
            self.add_rule_to_future(rule_number, position, boundary);
        }
        self.get_enough_chars_for_children(offset);
        self.handle_children(node, offset, loops);
    }

    /// Gets elements such that there are a given number of elements in
    /// the future buffer.
    ///
    /// May get one more character than `position`, to ensure that the
    /// rule applies.
    fn get_enough_chars_for_rule(&mut self, position: usize) {
        while self.future.len() <= position {
            match self.char_indices.next() {
                None => break,
                Some((char_offset, char)) =>
                    self.future.push_back(FutureInfo {
                        skip: false,
                        char_info: Some(CharInfo {
                            char_offset: char_offset,
                            category: Some(breaks::FromChar::from_char(char)),
                        }),
                        rule_info: None,
                    }),
            }
        }
    }

    /// Adds a rule at a given position, ignoring skipped characters.
    ///
    /// Grows the future buffer if it is too small.
    ///
    /// Does not add the rule if the character at the given position is
    /// already subject to a higher-ranked rule.
    fn add_rule_to_future(
        &mut self, rule_number: usize, mut position: usize, boundary: Boundary)
    {
        let mut real_position: usize = 0;
        position += 1;
        for future_info in self.future.iter() {
            if position == 0 {
                break;
            }
            if !future_info.skip {
                position -= 1;
            }
            real_position  += 1;
        }
        real_position -= 1;
        if self.future.get_mut(real_position).is_none() {
            self.future.push_back(FutureInfo {
                skip: false,
                char_info: Some(CharInfo {
                    char_offset: self.string.len(),
                    category: None,
                }),
                rule_info: Some(RuleInfo {
                    rule_number: rule_number,
                    boundary: boundary,
                }),
            });
        } else {
            let fut = self.future.get_mut(real_position).unwrap();
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

    /// Gets elements such that there are a given number of elements in
    /// the future buffer.
    ///
    /// May get one more character than `offset`, to ensure that the
    /// rule applies.
    fn get_enough_chars_for_children(&mut self, offset: usize) {
        while self.future.len() <= offset {
            self.future.push_back(FutureInfo {
                skip: false,
                char_info: match self.char_indices.next() {
                    None => Some(CharInfo {
                        char_offset: self.string.len(),
                        category: None,
                    }),
                    Some((char_offset, char)) => Some(CharInfo {
                        char_offset: char_offset,
                        category: Some(breaks::FromChar::from_char(char)),
                    }),
                },
                rule_info: None,
            });
        }
    }

    /// Populates the future buffer with respect to the current state
    /// node and an index into the future buffer to add rules to.
    fn handle_children(&mut self, node: &Node<Category>, offset: usize,
                       loops: usize)
    {
        for &(ref category, ref child) in node.children.iter() {
            let mut should_find_breaks = false;
            {
                match self.future[offset].char_info {
                    None => (),
                    Some(ref char_info) =>
                        should_find_breaks = match char_info.category {
                            None => false,
                            Some(ref c) => *c == *category,
                        },
                }
            }
            if should_find_breaks {
                let (node, loops) = match *child {
                    NextNode::Child(ref node) => (node, loops),
                    NextNode::Loop => {
                        self.future[offset].skip = true;
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

/// Creates a node corresponding to the default specification of UAX
/// \#29.
pub fn make_word_break_tree() -> Node<breaks::word_break::Category> {
    use breaks::word_break::Category;
    let any_node = NextNode::Child(Node {
        rules: vec![(14000, 1, Boundary::Break)],
        children: vec![
            (Category::Extend, NextNode::Loop),
            (Category::Format, NextNode::Loop),
        ],
    });
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
                    (Category::ALetter, any_node.clone()),
                    (Category::CR, any_node.clone()),
                    (Category::Double_Quote, any_node.clone()),
                    (Category::Extend, any_node.clone()),
                    (Category::ExtendNumLet, any_node.clone()),
                    (Category::Format, any_node.clone()),
                    (Category::Hebrew_Letter, any_node.clone()),
                    (Category::Katakana, any_node.clone()),
                    (Category::MidLetter, any_node.clone()),
                    (Category::MidNum, any_node.clone()),
                    (Category::MidNumLet, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Numeric, any_node.clone()),
                    (Category::Regional_Indicator, any_node.clone()),
                    (Category::Single_Quote, any_node.clone()),
                    (Category::Other, any_node.clone()),
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                    (Category::CR, any_node.clone()),
                    (Category::Double_Quote, any_node.clone()),
                    (Category::Katakana, any_node.clone()),
                    (Category::LF, any_node.clone()),
                    (Category::MidNum, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Regional_Indicator, any_node.clone()),
                    (Category::Other, any_node.clone()),
                ],
            })),
            (Category::Hebrew_Letter, NextNode::Child(Node {
                rules: vec![],
                children: vec![
                    (Category::Extend, NextNode::Loop),
                    (Category::Format, NextNode::Loop),
                    (Category::ALetter, NextNode::Child(Node {
                        rules: vec![(5002, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::Hebrew_Letter, NextNode::Child(Node {
                        rules: vec![(5003, 1, Boundary::NoBreak)],
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Numeric, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                    (Category::CR, any_node.clone()),
                    (Category::Katakana, any_node.clone()),
                    (Category::LF, any_node.clone()),
                    (Category::MidNum, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Regional_Indicator, any_node.clone()),
                    (Category::Other, any_node.clone()),
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
                                rules: vec![(11000, 2, Boundary::NoBreak),
                                            (12000, 1, Boundary::NoBreak)],
                                children: vec![],
                            })),
                            (Category::ALetter, any_node.clone()),
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Hebrew_Letter, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::ALetter, any_node.clone()),
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Hebrew_Letter, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
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
                            (Category::ALetter, any_node.clone()),
                            (Category::CR, any_node.clone()),
                            (Category::Double_Quote, any_node.clone()),
                            (Category::ExtendNumLet, any_node.clone()),
                            (Category::Hebrew_Letter, any_node.clone()),
                            (Category::Katakana, any_node.clone()),
                            (Category::LF, any_node.clone()),
                            (Category::MidLetter, any_node.clone()),
                            (Category::MidNum, any_node.clone()),
                            (Category::MidNumLet, any_node.clone()),
                            (Category::Newline, any_node.clone()),
                            (Category::Regional_Indicator, any_node.clone()),
                            (Category::Single_Quote, any_node.clone()),
                            (Category::Other, any_node.clone()),
                        ],
                    })),
                    (Category::ExtendNumLet, NextNode::Child(Node {
                        rules: vec![(13102, 1, Boundary::NoBreak)],
                        children: vec![],
                    })),
                    (Category::CR, any_node.clone()),
                    (Category::Double_Quote, any_node.clone()),
                    (Category::Katakana, any_node.clone()),
                    (Category::LF, any_node.clone()),
                    (Category::MidLetter, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Regional_Indicator, any_node.clone()),
                    (Category::Other, any_node.clone()),
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
                    (Category::ALetter, any_node.clone()),
                    (Category::CR, any_node.clone()),
                    (Category::Double_Quote, any_node.clone()),
                    (Category::Hebrew_Letter, any_node.clone()),
                    (Category::LF, any_node.clone()),
                    (Category::MidLetter, any_node.clone()),
                    (Category::MidNum, any_node.clone()),
                    (Category::MidNumLet, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Numeric, any_node.clone()),
                    (Category::Regional_Indicator, any_node.clone()),
                    (Category::Single_Quote, any_node.clone()),
                    (Category::Other, any_node.clone()),
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
                    (Category::CR, any_node.clone()),
                    (Category::Double_Quote, any_node.clone()),
                    (Category::LF, any_node.clone()),
                    (Category::MidLetter, any_node.clone()),
                    (Category::MidNum, any_node.clone()),
                    (Category::MidNumLet, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Regional_Indicator, any_node.clone()),
                    (Category::Single_Quote, any_node.clone()),
                    (Category::Other, any_node.clone()),
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
                    (Category::ALetter, any_node.clone()),
                    (Category::CR, any_node.clone()),
                    (Category::Double_Quote, any_node.clone()),
                    (Category::ExtendNumLet, any_node.clone()),
                    (Category::Hebrew_Letter, any_node.clone()),
                    (Category::Katakana, any_node.clone()),
                    (Category::LF, any_node.clone()),
                    (Category::MidLetter, any_node.clone()),
                    (Category::MidNum, any_node.clone()),
                    (Category::MidNumLet, any_node.clone()),
                    (Category::Newline, any_node.clone()),
                    (Category::Numeric, any_node.clone()),
                    (Category::Single_Quote, any_node.clone()),
                    (Category::Other, any_node.clone()),
                ],
            })),
            (Category::Double_Quote, any_node.clone()),
            (Category::Extend, any_node.clone()),
            (Category::Format, any_node.clone()),
            (Category::MidLetter, any_node.clone()),
            (Category::MidNum, any_node.clone()),
            (Category::MidNumLet, any_node.clone()),
            (Category::Single_Quote, any_node.clone()),
            (Category::Other, any_node.clone()),
        ],
    }
}
