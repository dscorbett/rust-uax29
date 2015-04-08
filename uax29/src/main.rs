#![allow(unstable)]

extern crate uax29;

#[cfg(not(test))]
fn main() {
    let mut breaks = uax29::defaults::Breaks::new(
        "x\u{300}y", uax29::defaults::make_word_break_tree());
    for word in breaks {
        println!("word: {:?}", word);
    }
}
