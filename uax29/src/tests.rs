use defaults;

#[test]
fn test_0000() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0001() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0002() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0003() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0004() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0005() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0006() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0007() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0008() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0009() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0010() {
    let mut breaks = defaults::Breaks::new("\u{1}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0011() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0012() {
    let mut breaks = defaults::Breaks::new("\u{1}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0013() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0014() {
    let mut breaks = defaults::Breaks::new("\u{1},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0015() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0016() {
    let mut breaks = defaults::Breaks::new("\u{1}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0017() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0018() {
    let mut breaks = defaults::Breaks::new("\u{1}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0019() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0020() {
    let mut breaks = defaults::Breaks::new("\u{1}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0021() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0022() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0023() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0024() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0025() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0026() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0027() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0028() {
    let mut breaks = defaults::Breaks::new("\u{1}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0029() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0030() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0031() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0032() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0033() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0034() {
    let mut breaks = defaults::Breaks::new("\u{1}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0035() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0036() {
    let mut breaks = defaults::Breaks::new("\u{1}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0037() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0038() {
    let mut breaks = defaults::Breaks::new("\u{1}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0039() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0040() {
    let mut breaks = defaults::Breaks::new("\u{1}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0041() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0042() {
    let mut breaks = defaults::Breaks::new("\u{1}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0043() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0044() {
    let mut breaks = defaults::Breaks::new("\u{1}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0045() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0046() {
    let mut breaks = defaults::Breaks::new("\u{1}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0047() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0048() {
    let mut breaks = defaults::Breaks::new("\u{1}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0049() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0050() {
    let mut breaks = defaults::Breaks::new("\u{1}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0051() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0052() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0053() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0054() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0055() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0056() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0057() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0058() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0059() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0060() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0061() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0062() {
    let mut breaks = defaults::Breaks::new("\u{d}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0063() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0064() {
    let mut breaks = defaults::Breaks::new("\u{d}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0065() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0066() {
    let mut breaks = defaults::Breaks::new("\u{d},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0067() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0068() {
    let mut breaks = defaults::Breaks::new("\u{d}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0069() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0070() {
    let mut breaks = defaults::Breaks::new("\u{d}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0071() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0072() {
    let mut breaks = defaults::Breaks::new("\u{d}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0073() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0074() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0075() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0076() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0077() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0078() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0079() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0080() {
    let mut breaks = defaults::Breaks::new("\u{d}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0081() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0082() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0083() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0084() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0085() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0086() {
    let mut breaks = defaults::Breaks::new("\u{d}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0087() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0088() {
    let mut breaks = defaults::Breaks::new("\u{d}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0089() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0090() {
    let mut breaks = defaults::Breaks::new("\u{d}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0091() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0092() {
    let mut breaks = defaults::Breaks::new("\u{d}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0093() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0094() {
    let mut breaks = defaults::Breaks::new("\u{d}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0095() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0096() {
    let mut breaks = defaults::Breaks::new("\u{d}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0097() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0098() {
    let mut breaks = defaults::Breaks::new("\u{d}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0099() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0100() {
    let mut breaks = defaults::Breaks::new("\u{d}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0101() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0102() {
    let mut breaks = defaults::Breaks::new("\u{d}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0103() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0104() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0105() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0106() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0107() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0108() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0109() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0110() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0111() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0112() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0113() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0114() {
    let mut breaks = defaults::Breaks::new("\u{a}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0115() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0116() {
    let mut breaks = defaults::Breaks::new("\u{a}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0117() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0118() {
    let mut breaks = defaults::Breaks::new("\u{a},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0119() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0120() {
    let mut breaks = defaults::Breaks::new("\u{a}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0121() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0122() {
    let mut breaks = defaults::Breaks::new("\u{a}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0123() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0124() {
    let mut breaks = defaults::Breaks::new("\u{a}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0125() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0126() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0127() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0128() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0129() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0130() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0131() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0132() {
    let mut breaks = defaults::Breaks::new("\u{a}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0133() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0134() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0135() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0136() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0137() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0138() {
    let mut breaks = defaults::Breaks::new("\u{a}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0139() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0140() {
    let mut breaks = defaults::Breaks::new("\u{a}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0141() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0142() {
    let mut breaks = defaults::Breaks::new("\u{a}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0143() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0144() {
    let mut breaks = defaults::Breaks::new("\u{a}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0145() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0146() {
    let mut breaks = defaults::Breaks::new("\u{a}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0147() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0148() {
    let mut breaks = defaults::Breaks::new("\u{a}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0149() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0150() {
    let mut breaks = defaults::Breaks::new("\u{a}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0151() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0152() {
    let mut breaks = defaults::Breaks::new("\u{a}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0153() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0154() {
    let mut breaks = defaults::Breaks::new("\u{a}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0155() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0156() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0157() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0158() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0159() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0160() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0161() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0162() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0163() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0164() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0165() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0166() {
    let mut breaks = defaults::Breaks::new("\u{b}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0167() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0168() {
    let mut breaks = defaults::Breaks::new("\u{b}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0169() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0170() {
    let mut breaks = defaults::Breaks::new("\u{b},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0171() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0172() {
    let mut breaks = defaults::Breaks::new("\u{b}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0173() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0174() {
    let mut breaks = defaults::Breaks::new("\u{b}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0175() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0176() {
    let mut breaks = defaults::Breaks::new("\u{b}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0177() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0178() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0179() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0180() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0181() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0182() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0183() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0184() {
    let mut breaks = defaults::Breaks::new("\u{b}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0185() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0186() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0187() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0188() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0189() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0190() {
    let mut breaks = defaults::Breaks::new("\u{b}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0191() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0192() {
    let mut breaks = defaults::Breaks::new("\u{b}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0193() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0194() {
    let mut breaks = defaults::Breaks::new("\u{b}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0195() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0196() {
    let mut breaks = defaults::Breaks::new("\u{b}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0197() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0198() {
    let mut breaks = defaults::Breaks::new("\u{b}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0199() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0200() {
    let mut breaks = defaults::Breaks::new("\u{b}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0201() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0202() {
    let mut breaks = defaults::Breaks::new("\u{b}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0203() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0204() {
    let mut breaks = defaults::Breaks::new("\u{b}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0205() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0206() {
    let mut breaks = defaults::Breaks::new("\u{b}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0207() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0208() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0209() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0210() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0211() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0212() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0213() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0214() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0215() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0216() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0217() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0218() {
    let mut breaks = defaults::Breaks::new("\u{3031}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0219() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0220() {
    let mut breaks = defaults::Breaks::new("\u{3031}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0221() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0222() {
    let mut breaks = defaults::Breaks::new("\u{3031},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0223() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0224() {
    let mut breaks = defaults::Breaks::new("\u{3031}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0225() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0226() {
    let mut breaks = defaults::Breaks::new("\u{3031}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0227() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0228() {
    let mut breaks = defaults::Breaks::new("\u{3031}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0229() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0230() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0231() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0232() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0233() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0234() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0235() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0236() {
    let mut breaks = defaults::Breaks::new("\u{3031}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0237() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0238() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0239() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0240() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0241() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0242() {
    let mut breaks = defaults::Breaks::new("\u{3031}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0243() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0244() {
    let mut breaks = defaults::Breaks::new("\u{3031}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0245() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0246() {
    let mut breaks = defaults::Breaks::new("\u{3031}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0247() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0248() {
    let mut breaks = defaults::Breaks::new("\u{3031}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0249() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0250() {
    let mut breaks = defaults::Breaks::new("\u{3031}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0251() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0252() {
    let mut breaks = defaults::Breaks::new("\u{3031}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0253() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0254() {
    let mut breaks = defaults::Breaks::new("\u{3031}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0255() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0256() {
    let mut breaks = defaults::Breaks::new("\u{3031}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0257() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0258() {
    let mut breaks = defaults::Breaks::new("\u{3031}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0259() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0260() {
    let mut breaks = defaults::Breaks::new("A\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0261() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0262() {
    let mut breaks = defaults::Breaks::new("A\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0263() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0264() {
    let mut breaks = defaults::Breaks::new("A\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0265() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0266() {
    let mut breaks = defaults::Breaks::new("A\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0267() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0268() {
    let mut breaks = defaults::Breaks::new("A\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0269() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0270() {
    let mut breaks = defaults::Breaks::new("AA",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("AA"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0271() {
    let mut breaks = defaults::Breaks::new("A\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0272() {
    let mut breaks = defaults::Breaks::new("A:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0273() {
    let mut breaks = defaults::Breaks::new("A\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0274() {
    let mut breaks = defaults::Breaks::new("A,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0275() {
    let mut breaks = defaults::Breaks::new("A\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0276() {
    let mut breaks = defaults::Breaks::new("A.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0277() {
    let mut breaks = defaults::Breaks::new("A\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0278() {
    let mut breaks = defaults::Breaks::new("A0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0279() {
    let mut breaks = defaults::Breaks::new("A\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0280() {
    let mut breaks = defaults::Breaks::new("A_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0281() {
    let mut breaks = defaults::Breaks::new("A\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0282() {
    let mut breaks = defaults::Breaks::new("A\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0283() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0284() {
    let mut breaks = defaults::Breaks::new("A\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0285() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0286() {
    let mut breaks = defaults::Breaks::new("A\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0287() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0288() {
    let mut breaks = defaults::Breaks::new("A'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0289() {
    let mut breaks = defaults::Breaks::new("A\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0290() {
    let mut breaks = defaults::Breaks::new("A\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0291() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0292() {
    let mut breaks = defaults::Breaks::new("A\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0293() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0294() {
    let mut breaks = defaults::Breaks::new("Aa\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0295() {
    let mut breaks = defaults::Breaks::new("A\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0296() {
    let mut breaks = defaults::Breaks::new("Aa:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0297() {
    let mut breaks = defaults::Breaks::new("A\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0298() {
    let mut breaks = defaults::Breaks::new("Aa'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0299() {
    let mut breaks = defaults::Breaks::new("A\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0300() {
    let mut breaks = defaults::Breaks::new("Aa'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0301() {
    let mut breaks = defaults::Breaks::new("A\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0302() {
    let mut breaks = defaults::Breaks::new("Aa,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0303() {
    let mut breaks = defaults::Breaks::new("A\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0304() {
    let mut breaks = defaults::Breaks::new("A1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0305() {
    let mut breaks = defaults::Breaks::new("A\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0306() {
    let mut breaks = defaults::Breaks::new("A1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0307() {
    let mut breaks = defaults::Breaks::new("A\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0308() {
    let mut breaks = defaults::Breaks::new("A1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0309() {
    let mut breaks = defaults::Breaks::new("A\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0310() {
    let mut breaks = defaults::Breaks::new("A1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0311() {
    let mut breaks = defaults::Breaks::new("A\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0312() {
    let mut breaks = defaults::Breaks::new(":\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0313() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0314() {
    let mut breaks = defaults::Breaks::new(":\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0315() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0316() {
    let mut breaks = defaults::Breaks::new(":\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0317() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0318() {
    let mut breaks = defaults::Breaks::new(":\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0319() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0320() {
    let mut breaks = defaults::Breaks::new(":\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0321() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0322() {
    let mut breaks = defaults::Breaks::new(":A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0323() {
    let mut breaks = defaults::Breaks::new(":\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0324() {
    let mut breaks = defaults::Breaks::new("::",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0325() {
    let mut breaks = defaults::Breaks::new(":\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0326() {
    let mut breaks = defaults::Breaks::new(":,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0327() {
    let mut breaks = defaults::Breaks::new(":\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0328() {
    let mut breaks = defaults::Breaks::new(":.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0329() {
    let mut breaks = defaults::Breaks::new(":\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0330() {
    let mut breaks = defaults::Breaks::new(":0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0331() {
    let mut breaks = defaults::Breaks::new(":\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0332() {
    let mut breaks = defaults::Breaks::new(":_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0333() {
    let mut breaks = defaults::Breaks::new(":\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0334() {
    let mut breaks = defaults::Breaks::new(":\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0335() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0336() {
    let mut breaks = defaults::Breaks::new(":\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0337() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0338() {
    let mut breaks = defaults::Breaks::new(":\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0339() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0340() {
    let mut breaks = defaults::Breaks::new(":'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0341() {
    let mut breaks = defaults::Breaks::new(":\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0342() {
    let mut breaks = defaults::Breaks::new(":\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0343() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0344() {
    let mut breaks = defaults::Breaks::new(":\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0345() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0346() {
    let mut breaks = defaults::Breaks::new(":a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0347() {
    let mut breaks = defaults::Breaks::new(":\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0348() {
    let mut breaks = defaults::Breaks::new(":a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0349() {
    let mut breaks = defaults::Breaks::new(":\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0350() {
    let mut breaks = defaults::Breaks::new(":a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0351() {
    let mut breaks = defaults::Breaks::new(":\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0352() {
    let mut breaks = defaults::Breaks::new(":a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0353() {
    let mut breaks = defaults::Breaks::new(":\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0354() {
    let mut breaks = defaults::Breaks::new(":a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0355() {
    let mut breaks = defaults::Breaks::new(":\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0356() {
    let mut breaks = defaults::Breaks::new(":1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0357() {
    let mut breaks = defaults::Breaks::new(":\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0358() {
    let mut breaks = defaults::Breaks::new(":1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0359() {
    let mut breaks = defaults::Breaks::new(":\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0360() {
    let mut breaks = defaults::Breaks::new(":1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0361() {
    let mut breaks = defaults::Breaks::new(":\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0362() {
    let mut breaks = defaults::Breaks::new(":1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0363() {
    let mut breaks = defaults::Breaks::new(":\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0364() {
    let mut breaks = defaults::Breaks::new(",\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0365() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0366() {
    let mut breaks = defaults::Breaks::new(",\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0367() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0368() {
    let mut breaks = defaults::Breaks::new(",\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0369() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0370() {
    let mut breaks = defaults::Breaks::new(",\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0371() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0372() {
    let mut breaks = defaults::Breaks::new(",\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0373() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0374() {
    let mut breaks = defaults::Breaks::new(",A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0375() {
    let mut breaks = defaults::Breaks::new(",\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0376() {
    let mut breaks = defaults::Breaks::new(",:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0377() {
    let mut breaks = defaults::Breaks::new(",\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0378() {
    let mut breaks = defaults::Breaks::new(",,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0379() {
    let mut breaks = defaults::Breaks::new(",\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0380() {
    let mut breaks = defaults::Breaks::new(",.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0381() {
    let mut breaks = defaults::Breaks::new(",\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0382() {
    let mut breaks = defaults::Breaks::new(",0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0383() {
    let mut breaks = defaults::Breaks::new(",\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0384() {
    let mut breaks = defaults::Breaks::new(",_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0385() {
    let mut breaks = defaults::Breaks::new(",\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0386() {
    let mut breaks = defaults::Breaks::new(",\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0387() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0388() {
    let mut breaks = defaults::Breaks::new(",\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0389() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0390() {
    let mut breaks = defaults::Breaks::new(",\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0391() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0392() {
    let mut breaks = defaults::Breaks::new(",'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0393() {
    let mut breaks = defaults::Breaks::new(",\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0394() {
    let mut breaks = defaults::Breaks::new(",\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0395() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0396() {
    let mut breaks = defaults::Breaks::new(",\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0397() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0398() {
    let mut breaks = defaults::Breaks::new(",a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0399() {
    let mut breaks = defaults::Breaks::new(",\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0400() {
    let mut breaks = defaults::Breaks::new(",a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0401() {
    let mut breaks = defaults::Breaks::new(",\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0402() {
    let mut breaks = defaults::Breaks::new(",a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0403() {
    let mut breaks = defaults::Breaks::new(",\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0404() {
    let mut breaks = defaults::Breaks::new(",a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0405() {
    let mut breaks = defaults::Breaks::new(",\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0406() {
    let mut breaks = defaults::Breaks::new(",a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0407() {
    let mut breaks = defaults::Breaks::new(",\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0408() {
    let mut breaks = defaults::Breaks::new(",1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0409() {
    let mut breaks = defaults::Breaks::new(",\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0410() {
    let mut breaks = defaults::Breaks::new(",1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0411() {
    let mut breaks = defaults::Breaks::new(",\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0412() {
    let mut breaks = defaults::Breaks::new(",1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0413() {
    let mut breaks = defaults::Breaks::new(",\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0414() {
    let mut breaks = defaults::Breaks::new(",1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0415() {
    let mut breaks = defaults::Breaks::new(",\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0416() {
    let mut breaks = defaults::Breaks::new(".\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0417() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0418() {
    let mut breaks = defaults::Breaks::new(".\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0419() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0420() {
    let mut breaks = defaults::Breaks::new(".\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0421() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0422() {
    let mut breaks = defaults::Breaks::new(".\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0423() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0424() {
    let mut breaks = defaults::Breaks::new(".\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0425() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0426() {
    let mut breaks = defaults::Breaks::new(".A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0427() {
    let mut breaks = defaults::Breaks::new(".\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0428() {
    let mut breaks = defaults::Breaks::new(".:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0429() {
    let mut breaks = defaults::Breaks::new(".\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0430() {
    let mut breaks = defaults::Breaks::new(".,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0431() {
    let mut breaks = defaults::Breaks::new(".\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0432() {
    let mut breaks = defaults::Breaks::new("..",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0433() {
    let mut breaks = defaults::Breaks::new(".\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0434() {
    let mut breaks = defaults::Breaks::new(".0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0435() {
    let mut breaks = defaults::Breaks::new(".\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0436() {
    let mut breaks = defaults::Breaks::new("._",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0437() {
    let mut breaks = defaults::Breaks::new(".\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0438() {
    let mut breaks = defaults::Breaks::new(".\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0439() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0440() {
    let mut breaks = defaults::Breaks::new(".\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0441() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0442() {
    let mut breaks = defaults::Breaks::new(".\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0443() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0444() {
    let mut breaks = defaults::Breaks::new(".'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0445() {
    let mut breaks = defaults::Breaks::new(".\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0446() {
    let mut breaks = defaults::Breaks::new(".\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0447() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0448() {
    let mut breaks = defaults::Breaks::new(".\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0449() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0450() {
    let mut breaks = defaults::Breaks::new(".a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0451() {
    let mut breaks = defaults::Breaks::new(".\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0452() {
    let mut breaks = defaults::Breaks::new(".a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0453() {
    let mut breaks = defaults::Breaks::new(".\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0454() {
    let mut breaks = defaults::Breaks::new(".a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0455() {
    let mut breaks = defaults::Breaks::new(".\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0456() {
    let mut breaks = defaults::Breaks::new(".a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0457() {
    let mut breaks = defaults::Breaks::new(".\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0458() {
    let mut breaks = defaults::Breaks::new(".a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0459() {
    let mut breaks = defaults::Breaks::new(".\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0460() {
    let mut breaks = defaults::Breaks::new(".1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0461() {
    let mut breaks = defaults::Breaks::new(".\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0462() {
    let mut breaks = defaults::Breaks::new(".1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0463() {
    let mut breaks = defaults::Breaks::new(".\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0464() {
    let mut breaks = defaults::Breaks::new(".1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0465() {
    let mut breaks = defaults::Breaks::new(".\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0466() {
    let mut breaks = defaults::Breaks::new(".1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0467() {
    let mut breaks = defaults::Breaks::new(".\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0468() {
    let mut breaks = defaults::Breaks::new("0\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0469() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0470() {
    let mut breaks = defaults::Breaks::new("0\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0471() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0472() {
    let mut breaks = defaults::Breaks::new("0\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0473() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0474() {
    let mut breaks = defaults::Breaks::new("0\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0475() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0476() {
    let mut breaks = defaults::Breaks::new("0\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0477() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0478() {
    let mut breaks = defaults::Breaks::new("0A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0479() {
    let mut breaks = defaults::Breaks::new("0\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0480() {
    let mut breaks = defaults::Breaks::new("0:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0481() {
    let mut breaks = defaults::Breaks::new("0\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0482() {
    let mut breaks = defaults::Breaks::new("0,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0483() {
    let mut breaks = defaults::Breaks::new("0\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0484() {
    let mut breaks = defaults::Breaks::new("0.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0485() {
    let mut breaks = defaults::Breaks::new("0\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0486() {
    let mut breaks = defaults::Breaks::new("00",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("00"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0487() {
    let mut breaks = defaults::Breaks::new("0\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0488() {
    let mut breaks = defaults::Breaks::new("0_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0489() {
    let mut breaks = defaults::Breaks::new("0\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0490() {
    let mut breaks = defaults::Breaks::new("0\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0491() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0492() {
    let mut breaks = defaults::Breaks::new("0\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0493() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0494() {
    let mut breaks = defaults::Breaks::new("0\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0495() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0496() {
    let mut breaks = defaults::Breaks::new("0'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0497() {
    let mut breaks = defaults::Breaks::new("0\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0498() {
    let mut breaks = defaults::Breaks::new("0\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0499() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0500() {
    let mut breaks = defaults::Breaks::new("0\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0501() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0502() {
    let mut breaks = defaults::Breaks::new("0a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0503() {
    let mut breaks = defaults::Breaks::new("0\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0504() {
    let mut breaks = defaults::Breaks::new("0a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0505() {
    let mut breaks = defaults::Breaks::new("0\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0506() {
    let mut breaks = defaults::Breaks::new("0a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0507() {
    let mut breaks = defaults::Breaks::new("0\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0508() {
    let mut breaks = defaults::Breaks::new("0a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0509() {
    let mut breaks = defaults::Breaks::new("0\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0510() {
    let mut breaks = defaults::Breaks::new("0a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0511() {
    let mut breaks = defaults::Breaks::new("0\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0512() {
    let mut breaks = defaults::Breaks::new("01:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0513() {
    let mut breaks = defaults::Breaks::new("0\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0514() {
    let mut breaks = defaults::Breaks::new("01'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0515() {
    let mut breaks = defaults::Breaks::new("0\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0516() {
    let mut breaks = defaults::Breaks::new("01,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0517() {
    let mut breaks = defaults::Breaks::new("0\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0518() {
    let mut breaks = defaults::Breaks::new("01.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0519() {
    let mut breaks = defaults::Breaks::new("0\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0520() {
    let mut breaks = defaults::Breaks::new("_\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0521() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0522() {
    let mut breaks = defaults::Breaks::new("_\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0523() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0524() {
    let mut breaks = defaults::Breaks::new("_\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0525() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0526() {
    let mut breaks = defaults::Breaks::new("_\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0527() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0528() {
    let mut breaks = defaults::Breaks::new("_\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0529() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0530() {
    let mut breaks = defaults::Breaks::new("_A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0531() {
    let mut breaks = defaults::Breaks::new("_\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0532() {
    let mut breaks = defaults::Breaks::new("_:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0533() {
    let mut breaks = defaults::Breaks::new("_\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0534() {
    let mut breaks = defaults::Breaks::new("_,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0535() {
    let mut breaks = defaults::Breaks::new("_\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0536() {
    let mut breaks = defaults::Breaks::new("_.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0537() {
    let mut breaks = defaults::Breaks::new("_\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0538() {
    let mut breaks = defaults::Breaks::new("_0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0539() {
    let mut breaks = defaults::Breaks::new("_\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0540() {
    let mut breaks = defaults::Breaks::new("__",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("__"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0541() {
    let mut breaks = defaults::Breaks::new("_\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0542() {
    let mut breaks = defaults::Breaks::new("_\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0543() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0544() {
    let mut breaks = defaults::Breaks::new("_\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0545() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0546() {
    let mut breaks = defaults::Breaks::new("_\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0547() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0548() {
    let mut breaks = defaults::Breaks::new("_'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0549() {
    let mut breaks = defaults::Breaks::new("_\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0550() {
    let mut breaks = defaults::Breaks::new("_\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0551() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0552() {
    let mut breaks = defaults::Breaks::new("_\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0553() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0554() {
    let mut breaks = defaults::Breaks::new("_a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0555() {
    let mut breaks = defaults::Breaks::new("_\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0556() {
    let mut breaks = defaults::Breaks::new("_a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0557() {
    let mut breaks = defaults::Breaks::new("_\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0558() {
    let mut breaks = defaults::Breaks::new("_a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0559() {
    let mut breaks = defaults::Breaks::new("_\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0560() {
    let mut breaks = defaults::Breaks::new("_a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0561() {
    let mut breaks = defaults::Breaks::new("_\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0562() {
    let mut breaks = defaults::Breaks::new("_a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0563() {
    let mut breaks = defaults::Breaks::new("_\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0564() {
    let mut breaks = defaults::Breaks::new("_1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0565() {
    let mut breaks = defaults::Breaks::new("_\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0566() {
    let mut breaks = defaults::Breaks::new("_1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0567() {
    let mut breaks = defaults::Breaks::new("_\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0568() {
    let mut breaks = defaults::Breaks::new("_1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0569() {
    let mut breaks = defaults::Breaks::new("_\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0570() {
    let mut breaks = defaults::Breaks::new("_1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0571() {
    let mut breaks = defaults::Breaks::new("_\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0572() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0573() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0574() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0575() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0576() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0577() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0578() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0579() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0580() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0581() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0582() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0583() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0584() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0585() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0586() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0587() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0588() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0589() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0590() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0591() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0592() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0593() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0594() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0595() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0596() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0597() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0598() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0599() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0600() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0601() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0602() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0603() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0604() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0605() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0606() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0607() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0608() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0609() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0610() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0611() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0612() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0613() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0614() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0615() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0616() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0617() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0618() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0619() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0620() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0621() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0622() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0623() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0624() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0625() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0626() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0627() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0628() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0629() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0630() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0631() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0632() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0633() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0634() {
    let mut breaks = defaults::Breaks::new("\u{5d0}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0635() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0636() {
    let mut breaks = defaults::Breaks::new("\u{5d0}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0637() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0638() {
    let mut breaks = defaults::Breaks::new("\u{5d0},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0639() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0640() {
    let mut breaks = defaults::Breaks::new("\u{5d0}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0641() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0642() {
    let mut breaks = defaults::Breaks::new("\u{5d0}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0643() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0644() {
    let mut breaks = defaults::Breaks::new("\u{5d0}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0645() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0646() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0647() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0648() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0649() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0650() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0651() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0652() {
    let mut breaks = defaults::Breaks::new("\u{5d0}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0653() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0654() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0655() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0656() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0657() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0658() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0659() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0660() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0661() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0662() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0663() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0664() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0665() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0666() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0667() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0668() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0669() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0670() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0671() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0672() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0673() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0674() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0675() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0676() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0677() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0678() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0679() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0680() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0681() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0682() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0683() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0684() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0685() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0686() {
    let mut breaks = defaults::Breaks::new("\u{22}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0687() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0688() {
    let mut breaks = defaults::Breaks::new("\u{22}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0689() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0690() {
    let mut breaks = defaults::Breaks::new("\u{22},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0691() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0692() {
    let mut breaks = defaults::Breaks::new("\u{22}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0693() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0694() {
    let mut breaks = defaults::Breaks::new("\u{22}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0695() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0696() {
    let mut breaks = defaults::Breaks::new("\u{22}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0697() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0698() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0699() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0700() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0701() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0702() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0703() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0704() {
    let mut breaks = defaults::Breaks::new("\u{22}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0705() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0706() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0707() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0708() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0709() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0710() {
    let mut breaks = defaults::Breaks::new("\u{22}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0711() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0712() {
    let mut breaks = defaults::Breaks::new("\u{22}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0713() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0714() {
    let mut breaks = defaults::Breaks::new("\u{22}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0715() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0716() {
    let mut breaks = defaults::Breaks::new("\u{22}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0717() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0718() {
    let mut breaks = defaults::Breaks::new("\u{22}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0719() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0720() {
    let mut breaks = defaults::Breaks::new("\u{22}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0721() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0722() {
    let mut breaks = defaults::Breaks::new("\u{22}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0723() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0724() {
    let mut breaks = defaults::Breaks::new("\u{22}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0725() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0726() {
    let mut breaks = defaults::Breaks::new("\u{22}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0727() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0728() {
    let mut breaks = defaults::Breaks::new("'\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0729() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0730() {
    let mut breaks = defaults::Breaks::new("'\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0731() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0732() {
    let mut breaks = defaults::Breaks::new("'\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0733() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0734() {
    let mut breaks = defaults::Breaks::new("'\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0735() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0736() {
    let mut breaks = defaults::Breaks::new("'\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0737() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0738() {
    let mut breaks = defaults::Breaks::new("'A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0739() {
    let mut breaks = defaults::Breaks::new("'\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0740() {
    let mut breaks = defaults::Breaks::new("':",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0741() {
    let mut breaks = defaults::Breaks::new("'\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0742() {
    let mut breaks = defaults::Breaks::new("',",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0743() {
    let mut breaks = defaults::Breaks::new("'\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0744() {
    let mut breaks = defaults::Breaks::new("'.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0745() {
    let mut breaks = defaults::Breaks::new("'\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0746() {
    let mut breaks = defaults::Breaks::new("'0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0747() {
    let mut breaks = defaults::Breaks::new("'\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0748() {
    let mut breaks = defaults::Breaks::new("'_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0749() {
    let mut breaks = defaults::Breaks::new("'\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0750() {
    let mut breaks = defaults::Breaks::new("'\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0751() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0752() {
    let mut breaks = defaults::Breaks::new("'\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0753() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0754() {
    let mut breaks = defaults::Breaks::new("'\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0755() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0756() {
    let mut breaks = defaults::Breaks::new("''",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0757() {
    let mut breaks = defaults::Breaks::new("'\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0758() {
    let mut breaks = defaults::Breaks::new("'\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0759() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0760() {
    let mut breaks = defaults::Breaks::new("'\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0761() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0762() {
    let mut breaks = defaults::Breaks::new("'a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0763() {
    let mut breaks = defaults::Breaks::new("'\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0764() {
    let mut breaks = defaults::Breaks::new("'a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0765() {
    let mut breaks = defaults::Breaks::new("'\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0766() {
    let mut breaks = defaults::Breaks::new("'a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0767() {
    let mut breaks = defaults::Breaks::new("'\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0768() {
    let mut breaks = defaults::Breaks::new("'a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0769() {
    let mut breaks = defaults::Breaks::new("'\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0770() {
    let mut breaks = defaults::Breaks::new("'a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0771() {
    let mut breaks = defaults::Breaks::new("'\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0772() {
    let mut breaks = defaults::Breaks::new("'1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0773() {
    let mut breaks = defaults::Breaks::new("'\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0774() {
    let mut breaks = defaults::Breaks::new("'1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0775() {
    let mut breaks = defaults::Breaks::new("'\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0776() {
    let mut breaks = defaults::Breaks::new("'1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0777() {
    let mut breaks = defaults::Breaks::new("'\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0778() {
    let mut breaks = defaults::Breaks::new("'1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0779() {
    let mut breaks = defaults::Breaks::new("'\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0780() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0781() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0782() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0783() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0784() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0785() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0786() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0787() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0788() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0789() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0790() {
    let mut breaks = defaults::Breaks::new("\u{ad}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0791() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0792() {
    let mut breaks = defaults::Breaks::new("\u{ad}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0793() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0794() {
    let mut breaks = defaults::Breaks::new("\u{ad},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0795() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0796() {
    let mut breaks = defaults::Breaks::new("\u{ad}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0797() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0798() {
    let mut breaks = defaults::Breaks::new("\u{ad}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0799() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0800() {
    let mut breaks = defaults::Breaks::new("\u{ad}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0801() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0802() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0803() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0804() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0805() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0806() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0807() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0808() {
    let mut breaks = defaults::Breaks::new("\u{ad}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0809() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0810() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0811() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0812() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0813() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0814() {
    let mut breaks = defaults::Breaks::new("\u{ad}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0815() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0816() {
    let mut breaks = defaults::Breaks::new("\u{ad}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0817() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0818() {
    let mut breaks = defaults::Breaks::new("\u{ad}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0819() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0820() {
    let mut breaks = defaults::Breaks::new("\u{ad}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0821() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0822() {
    let mut breaks = defaults::Breaks::new("\u{ad}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0823() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0824() {
    let mut breaks = defaults::Breaks::new("\u{ad}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0825() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0826() {
    let mut breaks = defaults::Breaks::new("\u{ad}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0827() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0828() {
    let mut breaks = defaults::Breaks::new("\u{ad}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0829() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0830() {
    let mut breaks = defaults::Breaks::new("\u{ad}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0831() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0832() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0833() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0834() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0835() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0836() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0837() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0838() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0839() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0840() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0841() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0842() {
    let mut breaks = defaults::Breaks::new("\u{300}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0843() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0844() {
    let mut breaks = defaults::Breaks::new("\u{300}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0845() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0846() {
    let mut breaks = defaults::Breaks::new("\u{300},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0847() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0848() {
    let mut breaks = defaults::Breaks::new("\u{300}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0849() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0850() {
    let mut breaks = defaults::Breaks::new("\u{300}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0851() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0852() {
    let mut breaks = defaults::Breaks::new("\u{300}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0853() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0854() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0855() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0856() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0857() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0858() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0859() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0860() {
    let mut breaks = defaults::Breaks::new("\u{300}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0861() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0862() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0863() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0864() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0865() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0866() {
    let mut breaks = defaults::Breaks::new("\u{300}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0867() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0868() {
    let mut breaks = defaults::Breaks::new("\u{300}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0869() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0870() {
    let mut breaks = defaults::Breaks::new("\u{300}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0871() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0872() {
    let mut breaks = defaults::Breaks::new("\u{300}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0873() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0874() {
    let mut breaks = defaults::Breaks::new("\u{300}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0875() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0876() {
    let mut breaks = defaults::Breaks::new("\u{300}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0877() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0878() {
    let mut breaks = defaults::Breaks::new("\u{300}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0879() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0880() {
    let mut breaks = defaults::Breaks::new("\u{300}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0881() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0882() {
    let mut breaks = defaults::Breaks::new("\u{300}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0883() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0884() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0885() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0886() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0887() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0888() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0889() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0890() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0891() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0892() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0893() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0894() {
    let mut breaks = defaults::Breaks::new("a\u{2060}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0895() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0896() {
    let mut breaks = defaults::Breaks::new("a\u{2060}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0897() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0898() {
    let mut breaks = defaults::Breaks::new("a\u{2060},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0899() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0900() {
    let mut breaks = defaults::Breaks::new("a\u{2060}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0901() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0902() {
    let mut breaks = defaults::Breaks::new("a\u{2060}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0903() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0904() {
    let mut breaks = defaults::Breaks::new("a\u{2060}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0905() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0906() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0907() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0908() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0909() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0910() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0911() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0912() {
    let mut breaks = defaults::Breaks::new("a\u{2060}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0913() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0914() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0915() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0916() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0917() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0918() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0919() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0920() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0921() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0922() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0923() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0924() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0925() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0926() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0927() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0928() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0929() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0930() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0931() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0932() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0933() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0934() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0935() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0936() {
    let mut breaks = defaults::Breaks::new("a:\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0937() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0938() {
    let mut breaks = defaults::Breaks::new("a:\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0939() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0940() {
    let mut breaks = defaults::Breaks::new("a:\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0941() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0942() {
    let mut breaks = defaults::Breaks::new("a:\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0943() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0944() {
    let mut breaks = defaults::Breaks::new("a:\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0945() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0946() {
    let mut breaks = defaults::Breaks::new("a:A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0947() {
    let mut breaks = defaults::Breaks::new("a:\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0948() {
    let mut breaks = defaults::Breaks::new("a::",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0949() {
    let mut breaks = defaults::Breaks::new("a:\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0950() {
    let mut breaks = defaults::Breaks::new("a:,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0951() {
    let mut breaks = defaults::Breaks::new("a:\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0952() {
    let mut breaks = defaults::Breaks::new("a:.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0953() {
    let mut breaks = defaults::Breaks::new("a:\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0954() {
    let mut breaks = defaults::Breaks::new("a:0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0955() {
    let mut breaks = defaults::Breaks::new("a:\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0956() {
    let mut breaks = defaults::Breaks::new("a:_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0957() {
    let mut breaks = defaults::Breaks::new("a:\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0958() {
    let mut breaks = defaults::Breaks::new("a:\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0959() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0960() {
    let mut breaks = defaults::Breaks::new("a:\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0961() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0962() {
    let mut breaks = defaults::Breaks::new("a:\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0963() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0964() {
    let mut breaks = defaults::Breaks::new("a:'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0965() {
    let mut breaks = defaults::Breaks::new("a:\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0966() {
    let mut breaks = defaults::Breaks::new("a:\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0967() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0968() {
    let mut breaks = defaults::Breaks::new("a:\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0969() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0970() {
    let mut breaks = defaults::Breaks::new("a:a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0971() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0972() {
    let mut breaks = defaults::Breaks::new("a:a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0973() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0974() {
    let mut breaks = defaults::Breaks::new("a:a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0975() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0976() {
    let mut breaks = defaults::Breaks::new("a:a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0977() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0978() {
    let mut breaks = defaults::Breaks::new("a:a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0979() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0980() {
    let mut breaks = defaults::Breaks::new("a:1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0981() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0982() {
    let mut breaks = defaults::Breaks::new("a:1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0983() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0984() {
    let mut breaks = defaults::Breaks::new("a:1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0985() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0986() {
    let mut breaks = defaults::Breaks::new("a:1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0987() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0988() {
    let mut breaks = defaults::Breaks::new("a'\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0989() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0990() {
    let mut breaks = defaults::Breaks::new("a'\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0991() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0992() {
    let mut breaks = defaults::Breaks::new("a'\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0993() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0994() {
    let mut breaks = defaults::Breaks::new("a'\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0995() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0996() {
    let mut breaks = defaults::Breaks::new("a'\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0997() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0998() {
    let mut breaks = defaults::Breaks::new("a'A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_0999() {
    let mut breaks = defaults::Breaks::new("a'\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1000() {
    let mut breaks = defaults::Breaks::new("a':",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1001() {
    let mut breaks = defaults::Breaks::new("a'\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1002() {
    let mut breaks = defaults::Breaks::new("a',",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1003() {
    let mut breaks = defaults::Breaks::new("a'\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1004() {
    let mut breaks = defaults::Breaks::new("a'.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1005() {
    let mut breaks = defaults::Breaks::new("a'\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1006() {
    let mut breaks = defaults::Breaks::new("a'0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1007() {
    let mut breaks = defaults::Breaks::new("a'\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1008() {
    let mut breaks = defaults::Breaks::new("a'_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1009() {
    let mut breaks = defaults::Breaks::new("a'\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1010() {
    let mut breaks = defaults::Breaks::new("a'\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1011() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1012() {
    let mut breaks = defaults::Breaks::new("a'\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1013() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1014() {
    let mut breaks = defaults::Breaks::new("a'\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1015() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1016() {
    let mut breaks = defaults::Breaks::new("a''",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1017() {
    let mut breaks = defaults::Breaks::new("a'\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1018() {
    let mut breaks = defaults::Breaks::new("a'\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1019() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1020() {
    let mut breaks = defaults::Breaks::new("a'\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1021() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1022() {
    let mut breaks = defaults::Breaks::new("a'a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1023() {
    let mut breaks = defaults::Breaks::new("a'\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1024() {
    let mut breaks = defaults::Breaks::new("a'a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1025() {
    let mut breaks = defaults::Breaks::new("a'\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1026() {
    let mut breaks = defaults::Breaks::new("a'a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1027() {
    let mut breaks = defaults::Breaks::new("a'\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1028() {
    let mut breaks = defaults::Breaks::new("a'a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1029() {
    let mut breaks = defaults::Breaks::new("a'\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1030() {
    let mut breaks = defaults::Breaks::new("a'a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1031() {
    let mut breaks = defaults::Breaks::new("a'\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1032() {
    let mut breaks = defaults::Breaks::new("a'1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1033() {
    let mut breaks = defaults::Breaks::new("a'\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1034() {
    let mut breaks = defaults::Breaks::new("a'1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1035() {
    let mut breaks = defaults::Breaks::new("a'\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1036() {
    let mut breaks = defaults::Breaks::new("a'1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1037() {
    let mut breaks = defaults::Breaks::new("a'\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1038() {
    let mut breaks = defaults::Breaks::new("a'1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1039() {
    let mut breaks = defaults::Breaks::new("a'\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1040() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1041() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1042() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1043() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1044() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1045() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1046() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1047() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1048() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1049() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1050() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1051() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1052() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1053() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1054() {
    let mut breaks = defaults::Breaks::new("a'\u{2060},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1055() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1056() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1057() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1058() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1059() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1060() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1061() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1062() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1063() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1064() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1065() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1066() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1067() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1068() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1069() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1070() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1071() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1072() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1073() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1074() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1075() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1076() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1077() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1078() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1079() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1080() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1081() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1082() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1083() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1084() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1085() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1086() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1087() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1088() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1089() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1090() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1091() {
    let mut breaks = defaults::Breaks::new("a'\u{2060}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1092() {
    let mut breaks = defaults::Breaks::new("a,\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1093() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1094() {
    let mut breaks = defaults::Breaks::new("a,\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1095() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1096() {
    let mut breaks = defaults::Breaks::new("a,\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1097() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1098() {
    let mut breaks = defaults::Breaks::new("a,\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1099() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1100() {
    let mut breaks = defaults::Breaks::new("a,\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1101() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1102() {
    let mut breaks = defaults::Breaks::new("a,A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1103() {
    let mut breaks = defaults::Breaks::new("a,\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1104() {
    let mut breaks = defaults::Breaks::new("a,:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1105() {
    let mut breaks = defaults::Breaks::new("a,\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1106() {
    let mut breaks = defaults::Breaks::new("a,,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1107() {
    let mut breaks = defaults::Breaks::new("a,\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1108() {
    let mut breaks = defaults::Breaks::new("a,.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1109() {
    let mut breaks = defaults::Breaks::new("a,\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1110() {
    let mut breaks = defaults::Breaks::new("a,0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1111() {
    let mut breaks = defaults::Breaks::new("a,\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1112() {
    let mut breaks = defaults::Breaks::new("a,_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1113() {
    let mut breaks = defaults::Breaks::new("a,\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1114() {
    let mut breaks = defaults::Breaks::new("a,\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1115() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1116() {
    let mut breaks = defaults::Breaks::new("a,\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1117() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1118() {
    let mut breaks = defaults::Breaks::new("a,\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1119() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1120() {
    let mut breaks = defaults::Breaks::new("a,'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1121() {
    let mut breaks = defaults::Breaks::new("a,\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1122() {
    let mut breaks = defaults::Breaks::new("a,\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1123() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1124() {
    let mut breaks = defaults::Breaks::new("a,\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1125() {
    let mut breaks = defaults::Breaks::new("a,\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1126() {
    let mut breaks = defaults::Breaks::new("a,a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1127() {
    let mut breaks = defaults::Breaks::new("a,\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1128() {
    let mut breaks = defaults::Breaks::new("a,a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1129() {
    let mut breaks = defaults::Breaks::new("a,\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1130() {
    let mut breaks = defaults::Breaks::new("a,a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1131() {
    let mut breaks = defaults::Breaks::new("a,\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1132() {
    let mut breaks = defaults::Breaks::new("a,a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1133() {
    let mut breaks = defaults::Breaks::new("a,\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1134() {
    let mut breaks = defaults::Breaks::new("a,a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1135() {
    let mut breaks = defaults::Breaks::new("a,\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1136() {
    let mut breaks = defaults::Breaks::new("a,1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1137() {
    let mut breaks = defaults::Breaks::new("a,\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1138() {
    let mut breaks = defaults::Breaks::new("a,1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1139() {
    let mut breaks = defaults::Breaks::new("a,\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1140() {
    let mut breaks = defaults::Breaks::new("a,1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1141() {
    let mut breaks = defaults::Breaks::new("a,\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1142() {
    let mut breaks = defaults::Breaks::new("a,1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1143() {
    let mut breaks = defaults::Breaks::new("a,\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1144() {
    let mut breaks = defaults::Breaks::new("1:\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1145() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1146() {
    let mut breaks = defaults::Breaks::new("1:\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1147() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1148() {
    let mut breaks = defaults::Breaks::new("1:\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1149() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1150() {
    let mut breaks = defaults::Breaks::new("1:\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1151() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1152() {
    let mut breaks = defaults::Breaks::new("1:\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1153() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1154() {
    let mut breaks = defaults::Breaks::new("1:A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1155() {
    let mut breaks = defaults::Breaks::new("1:\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1156() {
    let mut breaks = defaults::Breaks::new("1::",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1157() {
    let mut breaks = defaults::Breaks::new("1:\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1158() {
    let mut breaks = defaults::Breaks::new("1:,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1159() {
    let mut breaks = defaults::Breaks::new("1:\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1160() {
    let mut breaks = defaults::Breaks::new("1:.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1161() {
    let mut breaks = defaults::Breaks::new("1:\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1162() {
    let mut breaks = defaults::Breaks::new("1:0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1163() {
    let mut breaks = defaults::Breaks::new("1:\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1164() {
    let mut breaks = defaults::Breaks::new("1:_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1165() {
    let mut breaks = defaults::Breaks::new("1:\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1166() {
    let mut breaks = defaults::Breaks::new("1:\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1167() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1168() {
    let mut breaks = defaults::Breaks::new("1:\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1169() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1170() {
    let mut breaks = defaults::Breaks::new("1:\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1171() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1172() {
    let mut breaks = defaults::Breaks::new("1:'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1173() {
    let mut breaks = defaults::Breaks::new("1:\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1174() {
    let mut breaks = defaults::Breaks::new("1:\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1175() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1176() {
    let mut breaks = defaults::Breaks::new("1:\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1177() {
    let mut breaks = defaults::Breaks::new("1:\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1178() {
    let mut breaks = defaults::Breaks::new("1:a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1179() {
    let mut breaks = defaults::Breaks::new("1:\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1180() {
    let mut breaks = defaults::Breaks::new("1:a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1181() {
    let mut breaks = defaults::Breaks::new("1:\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1182() {
    let mut breaks = defaults::Breaks::new("1:a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1183() {
    let mut breaks = defaults::Breaks::new("1:\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1184() {
    let mut breaks = defaults::Breaks::new("1:a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1185() {
    let mut breaks = defaults::Breaks::new("1:\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1186() {
    let mut breaks = defaults::Breaks::new("1:a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1187() {
    let mut breaks = defaults::Breaks::new("1:\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1188() {
    let mut breaks = defaults::Breaks::new("1:1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1189() {
    let mut breaks = defaults::Breaks::new("1:\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1190() {
    let mut breaks = defaults::Breaks::new("1:1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1191() {
    let mut breaks = defaults::Breaks::new("1:\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1192() {
    let mut breaks = defaults::Breaks::new("1:1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1193() {
    let mut breaks = defaults::Breaks::new("1:\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1194() {
    let mut breaks = defaults::Breaks::new("1:1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1195() {
    let mut breaks = defaults::Breaks::new("1:\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1196() {
    let mut breaks = defaults::Breaks::new("1'\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1197() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1198() {
    let mut breaks = defaults::Breaks::new("1'\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1199() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1200() {
    let mut breaks = defaults::Breaks::new("1'\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1201() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1202() {
    let mut breaks = defaults::Breaks::new("1'\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1203() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1204() {
    let mut breaks = defaults::Breaks::new("1'\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1205() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1206() {
    let mut breaks = defaults::Breaks::new("1'A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1207() {
    let mut breaks = defaults::Breaks::new("1'\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1208() {
    let mut breaks = defaults::Breaks::new("1':",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1209() {
    let mut breaks = defaults::Breaks::new("1'\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1210() {
    let mut breaks = defaults::Breaks::new("1',",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1211() {
    let mut breaks = defaults::Breaks::new("1'\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1212() {
    let mut breaks = defaults::Breaks::new("1'.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1213() {
    let mut breaks = defaults::Breaks::new("1'\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1214() {
    let mut breaks = defaults::Breaks::new("1'0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1215() {
    let mut breaks = defaults::Breaks::new("1'\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1216() {
    let mut breaks = defaults::Breaks::new("1'_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1217() {
    let mut breaks = defaults::Breaks::new("1'\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1218() {
    let mut breaks = defaults::Breaks::new("1'\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1219() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1220() {
    let mut breaks = defaults::Breaks::new("1'\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1221() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1222() {
    let mut breaks = defaults::Breaks::new("1'\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1223() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1224() {
    let mut breaks = defaults::Breaks::new("1''",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1225() {
    let mut breaks = defaults::Breaks::new("1'\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1226() {
    let mut breaks = defaults::Breaks::new("1'\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1227() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1228() {
    let mut breaks = defaults::Breaks::new("1'\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1229() {
    let mut breaks = defaults::Breaks::new("1'\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1230() {
    let mut breaks = defaults::Breaks::new("1'a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1231() {
    let mut breaks = defaults::Breaks::new("1'\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1232() {
    let mut breaks = defaults::Breaks::new("1'a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1233() {
    let mut breaks = defaults::Breaks::new("1'\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1234() {
    let mut breaks = defaults::Breaks::new("1'a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1235() {
    let mut breaks = defaults::Breaks::new("1'\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1236() {
    let mut breaks = defaults::Breaks::new("1'a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1237() {
    let mut breaks = defaults::Breaks::new("1'\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1238() {
    let mut breaks = defaults::Breaks::new("1'a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1239() {
    let mut breaks = defaults::Breaks::new("1'\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1240() {
    let mut breaks = defaults::Breaks::new("1'1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1241() {
    let mut breaks = defaults::Breaks::new("1'\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1242() {
    let mut breaks = defaults::Breaks::new("1'1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1243() {
    let mut breaks = defaults::Breaks::new("1'\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1244() {
    let mut breaks = defaults::Breaks::new("1'1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1245() {
    let mut breaks = defaults::Breaks::new("1'\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1246() {
    let mut breaks = defaults::Breaks::new("1'1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1247() {
    let mut breaks = defaults::Breaks::new("1'\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1'\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1248() {
    let mut breaks = defaults::Breaks::new("1,\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1249() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1250() {
    let mut breaks = defaults::Breaks::new("1,\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1251() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1252() {
    let mut breaks = defaults::Breaks::new("1,\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1253() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1254() {
    let mut breaks = defaults::Breaks::new("1,\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1255() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1256() {
    let mut breaks = defaults::Breaks::new("1,\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1257() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1258() {
    let mut breaks = defaults::Breaks::new("1,A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1259() {
    let mut breaks = defaults::Breaks::new("1,\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1260() {
    let mut breaks = defaults::Breaks::new("1,:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1261() {
    let mut breaks = defaults::Breaks::new("1,\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1262() {
    let mut breaks = defaults::Breaks::new("1,,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1263() {
    let mut breaks = defaults::Breaks::new("1,\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1264() {
    let mut breaks = defaults::Breaks::new("1,.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1265() {
    let mut breaks = defaults::Breaks::new("1,\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1266() {
    let mut breaks = defaults::Breaks::new("1,0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1267() {
    let mut breaks = defaults::Breaks::new("1,\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1268() {
    let mut breaks = defaults::Breaks::new("1,_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1269() {
    let mut breaks = defaults::Breaks::new("1,\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1270() {
    let mut breaks = defaults::Breaks::new("1,\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1271() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1272() {
    let mut breaks = defaults::Breaks::new("1,\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1273() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1274() {
    let mut breaks = defaults::Breaks::new("1,\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1275() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1276() {
    let mut breaks = defaults::Breaks::new("1,'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1277() {
    let mut breaks = defaults::Breaks::new("1,\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1278() {
    let mut breaks = defaults::Breaks::new("1,\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1279() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1280() {
    let mut breaks = defaults::Breaks::new("1,\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1281() {
    let mut breaks = defaults::Breaks::new("1,\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1282() {
    let mut breaks = defaults::Breaks::new("1,a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1283() {
    let mut breaks = defaults::Breaks::new("1,\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1284() {
    let mut breaks = defaults::Breaks::new("1,a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1285() {
    let mut breaks = defaults::Breaks::new("1,\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1286() {
    let mut breaks = defaults::Breaks::new("1,a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1287() {
    let mut breaks = defaults::Breaks::new("1,\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1288() {
    let mut breaks = defaults::Breaks::new("1,a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1289() {
    let mut breaks = defaults::Breaks::new("1,\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1290() {
    let mut breaks = defaults::Breaks::new("1,a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1291() {
    let mut breaks = defaults::Breaks::new("1,\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1292() {
    let mut breaks = defaults::Breaks::new("1,1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1293() {
    let mut breaks = defaults::Breaks::new("1,\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1294() {
    let mut breaks = defaults::Breaks::new("1,1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1295() {
    let mut breaks = defaults::Breaks::new("1,\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1296() {
    let mut breaks = defaults::Breaks::new("1,1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1297() {
    let mut breaks = defaults::Breaks::new("1,\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1298() {
    let mut breaks = defaults::Breaks::new("1,1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1299() {
    let mut breaks = defaults::Breaks::new("1,\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1,\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1300() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1301() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1302() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1303() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1304() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1305() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1306() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1307() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1308() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1309() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1310() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1311() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1312() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1313() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1314() {
    let mut breaks = defaults::Breaks::new("1.\u{2060},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1315() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1316() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1317() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1318() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1319() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1320() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1321() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1322() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1323() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1324() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1325() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1326() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1327() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1328() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1329() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1330() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1331() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1332() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1333() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1334() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1335() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1336() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1337() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1338() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1339() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1340() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1341() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1342() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1343() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1344() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1345() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1346() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1347() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1348() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1349() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1350() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1351() {
    let mut breaks = defaults::Breaks::new("1.\u{2060}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("1.\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1352() {
    let mut breaks = defaults::Breaks::new("can't",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("can't"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1353() {
    let mut breaks = defaults::Breaks::new("can\u{2019}t",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("can\u{2019}t"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1354() {
    let mut breaks = defaults::Breaks::new("ab\u{ad}by",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("ab\u{ad}by"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1355() {
    let mut breaks = defaults::Breaks::new("a$-34,567.14%b",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("$"));
    assert_eq!(breaks.next(), Some("-"));
    assert_eq!(breaks.next(), Some("34,567.14"));
    assert_eq!(breaks.next(), Some("%"));
    assert_eq!(breaks.next(), Some("b"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1356() {
    let mut breaks = defaults::Breaks::new("3a",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("3a"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1357() {
    let mut breaks = defaults::Breaks::new("\u{2060}c\u{2060}a\u{2060}n\u{2060}'\u{2060}t\u{2060}\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{2060}"));
    assert_eq!(breaks.next(), Some("c\u{2060}a\u{2060}n\u{2060}'\u{2060}t\u{2060}\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1358() {
    let mut breaks = defaults::Breaks::new("\u{2060}c\u{2060}a\u{2060}n\u{2060}\u{2019}\u{2060}t\u{2060}\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{2060}"));
    assert_eq!(breaks.next(), Some("c\u{2060}a\u{2060}n\u{2060}\u{2019}\u{2060}t\u{2060}\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1359() {
    let mut breaks = defaults::Breaks::new("\u{2060}a\u{2060}b\u{2060}\u{ad}\u{2060}b\u{2060}y\u{2060}\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{2060}"));
    assert_eq!(breaks.next(), Some("a\u{2060}b\u{2060}\u{ad}\u{2060}b\u{2060}y\u{2060}\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1360() {
    let mut breaks = defaults::Breaks::new("\u{2060}a\u{2060}$\u{2060}-\u{2060}3\u{2060}4\u{2060},\u{2060}5\u{2060}6\u{2060}7\u{2060}.\u{2060}1\u{2060}4\u{2060}%\u{2060}b\u{2060}\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{2060}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("$\u{2060}"));
    assert_eq!(breaks.next(), Some("-\u{2060}"));
    assert_eq!(breaks.next(), Some("3\u{2060}4\u{2060},\u{2060}5\u{2060}6\u{2060}7\u{2060}.\u{2060}1\u{2060}4\u{2060}"));
    assert_eq!(breaks.next(), Some("%\u{2060}"));
    assert_eq!(breaks.next(), Some("b\u{2060}\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1361() {
    let mut breaks = defaults::Breaks::new("\u{2060}3\u{2060}a\u{2060}\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{2060}"));
    assert_eq!(breaks.next(), Some("3\u{2060}a\u{2060}\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1362() {
    let mut breaks = defaults::Breaks::new("a\u{1f1e6}b",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("b"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1363() {
    let mut breaks = defaults::Breaks::new("\u{1f1f7}\u{1f1fa}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1f7}\u{1f1fa}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1364() {
    let mut breaks = defaults::Breaks::new("\u{1f1f7}\u{1f1fa}\u{1f1f8}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1f7}\u{1f1fa}\u{1f1f8}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1365() {
    let mut breaks = defaults::Breaks::new("\u{1f1f7}\u{1f1fa}\u{1f1f8}\u{1f1ea}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1f7}\u{1f1fa}\u{1f1f8}\u{1f1ea}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1366() {
    let mut breaks = defaults::Breaks::new("\u{1f1f7}\u{1f1fa}\u{200b}\u{1f1f8}\u{1f1ea}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1f7}\u{1f1fa}"));
    assert_eq!(breaks.next(), Some("\u{200b}"));
    assert_eq!(breaks.next(), Some("\u{1f1f8}\u{1f1ea}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1367() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{1f1e7}\u{1f1e8}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{1f1e7}\u{1f1e8}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1368() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{200d}\u{1f1e7}\u{1f1e8}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{200d}\u{1f1e7}\u{1f1e8}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1369() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{1f1e7}\u{200d}\u{1f1e8}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{1f1e7}\u{200d}\u{1f1e8}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1370() {
    let mut breaks = defaults::Breaks::new(" \u{200d}\u{646}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(" \u{200d}"));
    assert_eq!(breaks.next(), Some("\u{646}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1371() {
    let mut breaks = defaults::Breaks::new("\u{646}\u{200d} ",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{646}\u{200d}"));
    assert_eq!(breaks.next(), Some(" "));
    assert_eq!(breaks.next(), None);
}

