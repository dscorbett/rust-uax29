use defaults;

#[test]
fn test_0() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_1() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_2() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_3() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_4() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_5() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_6() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_7() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_8() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_9() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_10() {
    let mut breaks = defaults::Breaks::new("\u{1}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_11() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_12() {
    let mut breaks = defaults::Breaks::new("\u{1}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_13() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_14() {
    let mut breaks = defaults::Breaks::new("\u{1},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_15() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_16() {
    let mut breaks = defaults::Breaks::new("\u{1}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_17() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_18() {
    let mut breaks = defaults::Breaks::new("\u{1}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_19() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_20() {
    let mut breaks = defaults::Breaks::new("\u{1}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_21() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_22() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_23() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_24() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_25() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_26() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_27() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_28() {
    let mut breaks = defaults::Breaks::new("\u{1}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_29() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_30() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_31() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_32() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_33() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_34() {
    let mut breaks = defaults::Breaks::new("\u{1}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_35() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_36() {
    let mut breaks = defaults::Breaks::new("\u{1}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_37() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_38() {
    let mut breaks = defaults::Breaks::new("\u{1}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_39() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_40() {
    let mut breaks = defaults::Breaks::new("\u{1}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_41() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_42() {
    let mut breaks = defaults::Breaks::new("\u{1}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_43() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_44() {
    let mut breaks = defaults::Breaks::new("\u{1}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_45() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_46() {
    let mut breaks = defaults::Breaks::new("\u{1}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_47() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_48() {
    let mut breaks = defaults::Breaks::new("\u{1}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_49() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_50() {
    let mut breaks = defaults::Breaks::new("\u{1}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_51() {
    let mut breaks = defaults::Breaks::new("\u{1}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_52() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_53() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_54() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_55() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_56() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_57() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_58() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_59() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_60() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_61() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_62() {
    let mut breaks = defaults::Breaks::new("\u{d}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_63() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_64() {
    let mut breaks = defaults::Breaks::new("\u{d}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_65() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_66() {
    let mut breaks = defaults::Breaks::new("\u{d},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_67() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_68() {
    let mut breaks = defaults::Breaks::new("\u{d}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_69() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_70() {
    let mut breaks = defaults::Breaks::new("\u{d}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_71() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_72() {
    let mut breaks = defaults::Breaks::new("\u{d}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_73() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_74() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_75() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_76() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_77() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_78() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_79() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_80() {
    let mut breaks = defaults::Breaks::new("\u{d}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_81() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_82() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_83() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_84() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_85() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_86() {
    let mut breaks = defaults::Breaks::new("\u{d}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_87() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_88() {
    let mut breaks = defaults::Breaks::new("\u{d}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_89() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_90() {
    let mut breaks = defaults::Breaks::new("\u{d}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_91() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_92() {
    let mut breaks = defaults::Breaks::new("\u{d}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_93() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_94() {
    let mut breaks = defaults::Breaks::new("\u{d}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_95() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_96() {
    let mut breaks = defaults::Breaks::new("\u{d}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_97() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_98() {
    let mut breaks = defaults::Breaks::new("\u{d}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_99() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_100() {
    let mut breaks = defaults::Breaks::new("\u{d}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_101() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_102() {
    let mut breaks = defaults::Breaks::new("\u{d}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_103() {
    let mut breaks = defaults::Breaks::new("\u{d}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_104() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_105() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_106() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_107() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_108() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_109() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_110() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_111() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_112() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_113() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_114() {
    let mut breaks = defaults::Breaks::new("\u{a}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_115() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_116() {
    let mut breaks = defaults::Breaks::new("\u{a}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_117() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_118() {
    let mut breaks = defaults::Breaks::new("\u{a},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_119() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_120() {
    let mut breaks = defaults::Breaks::new("\u{a}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_121() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_122() {
    let mut breaks = defaults::Breaks::new("\u{a}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_123() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_124() {
    let mut breaks = defaults::Breaks::new("\u{a}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_125() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_126() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_127() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_128() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_129() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_130() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_131() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_132() {
    let mut breaks = defaults::Breaks::new("\u{a}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_133() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_134() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_135() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_136() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_137() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_138() {
    let mut breaks = defaults::Breaks::new("\u{a}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_139() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_140() {
    let mut breaks = defaults::Breaks::new("\u{a}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_141() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_142() {
    let mut breaks = defaults::Breaks::new("\u{a}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_143() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_144() {
    let mut breaks = defaults::Breaks::new("\u{a}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_145() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_146() {
    let mut breaks = defaults::Breaks::new("\u{a}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_147() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_148() {
    let mut breaks = defaults::Breaks::new("\u{a}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_149() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_150() {
    let mut breaks = defaults::Breaks::new("\u{a}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_151() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_152() {
    let mut breaks = defaults::Breaks::new("\u{a}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_153() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_154() {
    let mut breaks = defaults::Breaks::new("\u{a}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_155() {
    let mut breaks = defaults::Breaks::new("\u{a}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_156() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_157() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_158() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_159() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_160() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_161() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_162() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_163() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_164() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_165() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_166() {
    let mut breaks = defaults::Breaks::new("\u{b}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_167() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_168() {
    let mut breaks = defaults::Breaks::new("\u{b}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_169() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_170() {
    let mut breaks = defaults::Breaks::new("\u{b},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_171() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_172() {
    let mut breaks = defaults::Breaks::new("\u{b}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_173() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_174() {
    let mut breaks = defaults::Breaks::new("\u{b}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_175() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_176() {
    let mut breaks = defaults::Breaks::new("\u{b}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_177() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_178() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_179() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_180() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_181() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_182() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_183() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_184() {
    let mut breaks = defaults::Breaks::new("\u{b}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_185() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_186() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_187() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_188() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_189() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_190() {
    let mut breaks = defaults::Breaks::new("\u{b}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_191() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_192() {
    let mut breaks = defaults::Breaks::new("\u{b}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_193() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_194() {
    let mut breaks = defaults::Breaks::new("\u{b}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_195() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_196() {
    let mut breaks = defaults::Breaks::new("\u{b}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_197() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_198() {
    let mut breaks = defaults::Breaks::new("\u{b}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_199() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_200() {
    let mut breaks = defaults::Breaks::new("\u{b}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_201() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_202() {
    let mut breaks = defaults::Breaks::new("\u{b}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_203() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_204() {
    let mut breaks = defaults::Breaks::new("\u{b}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_205() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_206() {
    let mut breaks = defaults::Breaks::new("\u{b}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_207() {
    let mut breaks = defaults::Breaks::new("\u{b}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), Some("\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_208() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_209() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_210() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_211() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_212() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_213() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_214() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_215() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_216() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_217() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_218() {
    let mut breaks = defaults::Breaks::new("\u{3031}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_219() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_220() {
    let mut breaks = defaults::Breaks::new("\u{3031}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_221() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_222() {
    let mut breaks = defaults::Breaks::new("\u{3031},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_223() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_224() {
    let mut breaks = defaults::Breaks::new("\u{3031}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_225() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_226() {
    let mut breaks = defaults::Breaks::new("\u{3031}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_227() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_228() {
    let mut breaks = defaults::Breaks::new("\u{3031}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_229() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_230() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_231() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_232() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_233() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_234() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_235() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_236() {
    let mut breaks = defaults::Breaks::new("\u{3031}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_237() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_238() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_239() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_240() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_241() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_242() {
    let mut breaks = defaults::Breaks::new("\u{3031}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_243() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_244() {
    let mut breaks = defaults::Breaks::new("\u{3031}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_245() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_246() {
    let mut breaks = defaults::Breaks::new("\u{3031}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_247() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_248() {
    let mut breaks = defaults::Breaks::new("\u{3031}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_249() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_250() {
    let mut breaks = defaults::Breaks::new("\u{3031}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_251() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_252() {
    let mut breaks = defaults::Breaks::new("\u{3031}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_253() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_254() {
    let mut breaks = defaults::Breaks::new("\u{3031}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_255() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_256() {
    let mut breaks = defaults::Breaks::new("\u{3031}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_257() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_258() {
    let mut breaks = defaults::Breaks::new("\u{3031}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_259() {
    let mut breaks = defaults::Breaks::new("\u{3031}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{3031}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_260() {
    let mut breaks = defaults::Breaks::new("A\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_261() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_262() {
    let mut breaks = defaults::Breaks::new("A\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_263() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_264() {
    let mut breaks = defaults::Breaks::new("A\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_265() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_266() {
    let mut breaks = defaults::Breaks::new("A\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_267() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_268() {
    let mut breaks = defaults::Breaks::new("A\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_269() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_270() {
    let mut breaks = defaults::Breaks::new("AA",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("AA"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_271() {
    let mut breaks = defaults::Breaks::new("A\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_272() {
    let mut breaks = defaults::Breaks::new("A:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_273() {
    let mut breaks = defaults::Breaks::new("A\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_274() {
    let mut breaks = defaults::Breaks::new("A,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_275() {
    let mut breaks = defaults::Breaks::new("A\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_276() {
    let mut breaks = defaults::Breaks::new("A.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_277() {
    let mut breaks = defaults::Breaks::new("A\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_278() {
    let mut breaks = defaults::Breaks::new("A0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_279() {
    let mut breaks = defaults::Breaks::new("A\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_280() {
    let mut breaks = defaults::Breaks::new("A_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_281() {
    let mut breaks = defaults::Breaks::new("A\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_282() {
    let mut breaks = defaults::Breaks::new("A\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_283() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_284() {
    let mut breaks = defaults::Breaks::new("A\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_285() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_286() {
    let mut breaks = defaults::Breaks::new("A\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_287() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_288() {
    let mut breaks = defaults::Breaks::new("A'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_289() {
    let mut breaks = defaults::Breaks::new("A\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_290() {
    let mut breaks = defaults::Breaks::new("A\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_291() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_292() {
    let mut breaks = defaults::Breaks::new("A\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_293() {
    let mut breaks = defaults::Breaks::new("A\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_294() {
    let mut breaks = defaults::Breaks::new("Aa\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_295() {
    let mut breaks = defaults::Breaks::new("A\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_296() {
    let mut breaks = defaults::Breaks::new("Aa:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_297() {
    let mut breaks = defaults::Breaks::new("A\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_298() {
    let mut breaks = defaults::Breaks::new("Aa'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_299() {
    let mut breaks = defaults::Breaks::new("A\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_300() {
    let mut breaks = defaults::Breaks::new("Aa'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_301() {
    let mut breaks = defaults::Breaks::new("A\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_302() {
    let mut breaks = defaults::Breaks::new("Aa,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("Aa"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_303() {
    let mut breaks = defaults::Breaks::new("A\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_304() {
    let mut breaks = defaults::Breaks::new("A1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_305() {
    let mut breaks = defaults::Breaks::new("A\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_306() {
    let mut breaks = defaults::Breaks::new("A1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_307() {
    let mut breaks = defaults::Breaks::new("A\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_308() {
    let mut breaks = defaults::Breaks::new("A1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_309() {
    let mut breaks = defaults::Breaks::new("A\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_310() {
    let mut breaks = defaults::Breaks::new("A1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_311() {
    let mut breaks = defaults::Breaks::new("A\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("A\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_312() {
    let mut breaks = defaults::Breaks::new(":\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_313() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_314() {
    let mut breaks = defaults::Breaks::new(":\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_315() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_316() {
    let mut breaks = defaults::Breaks::new(":\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_317() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_318() {
    let mut breaks = defaults::Breaks::new(":\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_319() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_320() {
    let mut breaks = defaults::Breaks::new(":\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_321() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_322() {
    let mut breaks = defaults::Breaks::new(":A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_323() {
    let mut breaks = defaults::Breaks::new(":\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_324() {
    let mut breaks = defaults::Breaks::new("::",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_325() {
    let mut breaks = defaults::Breaks::new(":\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_326() {
    let mut breaks = defaults::Breaks::new(":,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_327() {
    let mut breaks = defaults::Breaks::new(":\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_328() {
    let mut breaks = defaults::Breaks::new(":.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_329() {
    let mut breaks = defaults::Breaks::new(":\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_330() {
    let mut breaks = defaults::Breaks::new(":0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_331() {
    let mut breaks = defaults::Breaks::new(":\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_332() {
    let mut breaks = defaults::Breaks::new(":_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_333() {
    let mut breaks = defaults::Breaks::new(":\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_334() {
    let mut breaks = defaults::Breaks::new(":\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_335() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_336() {
    let mut breaks = defaults::Breaks::new(":\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_337() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_338() {
    let mut breaks = defaults::Breaks::new(":\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_339() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_340() {
    let mut breaks = defaults::Breaks::new(":'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_341() {
    let mut breaks = defaults::Breaks::new(":\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_342() {
    let mut breaks = defaults::Breaks::new(":\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_343() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_344() {
    let mut breaks = defaults::Breaks::new(":\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_345() {
    let mut breaks = defaults::Breaks::new(":\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_346() {
    let mut breaks = defaults::Breaks::new(":a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_347() {
    let mut breaks = defaults::Breaks::new(":\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_348() {
    let mut breaks = defaults::Breaks::new(":a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_349() {
    let mut breaks = defaults::Breaks::new(":\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_350() {
    let mut breaks = defaults::Breaks::new(":a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_351() {
    let mut breaks = defaults::Breaks::new(":\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_352() {
    let mut breaks = defaults::Breaks::new(":a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_353() {
    let mut breaks = defaults::Breaks::new(":\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_354() {
    let mut breaks = defaults::Breaks::new(":a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_355() {
    let mut breaks = defaults::Breaks::new(":\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_356() {
    let mut breaks = defaults::Breaks::new(":1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_357() {
    let mut breaks = defaults::Breaks::new(":\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_358() {
    let mut breaks = defaults::Breaks::new(":1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_359() {
    let mut breaks = defaults::Breaks::new(":\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_360() {
    let mut breaks = defaults::Breaks::new(":1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_361() {
    let mut breaks = defaults::Breaks::new(":\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_362() {
    let mut breaks = defaults::Breaks::new(":1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_363() {
    let mut breaks = defaults::Breaks::new(":\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_364() {
    let mut breaks = defaults::Breaks::new(",\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_365() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_366() {
    let mut breaks = defaults::Breaks::new(",\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_367() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_368() {
    let mut breaks = defaults::Breaks::new(",\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_369() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_370() {
    let mut breaks = defaults::Breaks::new(",\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_371() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_372() {
    let mut breaks = defaults::Breaks::new(",\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_373() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_374() {
    let mut breaks = defaults::Breaks::new(",A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_375() {
    let mut breaks = defaults::Breaks::new(",\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_376() {
    let mut breaks = defaults::Breaks::new(",:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_377() {
    let mut breaks = defaults::Breaks::new(",\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_378() {
    let mut breaks = defaults::Breaks::new(",,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_379() {
    let mut breaks = defaults::Breaks::new(",\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_380() {
    let mut breaks = defaults::Breaks::new(",.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_381() {
    let mut breaks = defaults::Breaks::new(",\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_382() {
    let mut breaks = defaults::Breaks::new(",0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_383() {
    let mut breaks = defaults::Breaks::new(",\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_384() {
    let mut breaks = defaults::Breaks::new(",_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_385() {
    let mut breaks = defaults::Breaks::new(",\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_386() {
    let mut breaks = defaults::Breaks::new(",\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_387() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_388() {
    let mut breaks = defaults::Breaks::new(",\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_389() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_390() {
    let mut breaks = defaults::Breaks::new(",\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_391() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_392() {
    let mut breaks = defaults::Breaks::new(",'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_393() {
    let mut breaks = defaults::Breaks::new(",\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_394() {
    let mut breaks = defaults::Breaks::new(",\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_395() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_396() {
    let mut breaks = defaults::Breaks::new(",\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_397() {
    let mut breaks = defaults::Breaks::new(",\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_398() {
    let mut breaks = defaults::Breaks::new(",a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_399() {
    let mut breaks = defaults::Breaks::new(",\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_400() {
    let mut breaks = defaults::Breaks::new(",a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_401() {
    let mut breaks = defaults::Breaks::new(",\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_402() {
    let mut breaks = defaults::Breaks::new(",a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_403() {
    let mut breaks = defaults::Breaks::new(",\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_404() {
    let mut breaks = defaults::Breaks::new(",a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_405() {
    let mut breaks = defaults::Breaks::new(",\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_406() {
    let mut breaks = defaults::Breaks::new(",a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_407() {
    let mut breaks = defaults::Breaks::new(",\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_408() {
    let mut breaks = defaults::Breaks::new(",1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_409() {
    let mut breaks = defaults::Breaks::new(",\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_410() {
    let mut breaks = defaults::Breaks::new(",1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_411() {
    let mut breaks = defaults::Breaks::new(",\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_412() {
    let mut breaks = defaults::Breaks::new(",1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_413() {
    let mut breaks = defaults::Breaks::new(",\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_414() {
    let mut breaks = defaults::Breaks::new(",1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_415() {
    let mut breaks = defaults::Breaks::new(",\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(",\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_416() {
    let mut breaks = defaults::Breaks::new(".\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_417() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_418() {
    let mut breaks = defaults::Breaks::new(".\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_419() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_420() {
    let mut breaks = defaults::Breaks::new(".\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_421() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_422() {
    let mut breaks = defaults::Breaks::new(".\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_423() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_424() {
    let mut breaks = defaults::Breaks::new(".\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_425() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_426() {
    let mut breaks = defaults::Breaks::new(".A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_427() {
    let mut breaks = defaults::Breaks::new(".\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_428() {
    let mut breaks = defaults::Breaks::new(".:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_429() {
    let mut breaks = defaults::Breaks::new(".\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_430() {
    let mut breaks = defaults::Breaks::new(".,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_431() {
    let mut breaks = defaults::Breaks::new(".\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_432() {
    let mut breaks = defaults::Breaks::new("..",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_433() {
    let mut breaks = defaults::Breaks::new(".\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_434() {
    let mut breaks = defaults::Breaks::new(".0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_435() {
    let mut breaks = defaults::Breaks::new(".\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_436() {
    let mut breaks = defaults::Breaks::new("._",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_437() {
    let mut breaks = defaults::Breaks::new(".\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_438() {
    let mut breaks = defaults::Breaks::new(".\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_439() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_440() {
    let mut breaks = defaults::Breaks::new(".\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_441() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_442() {
    let mut breaks = defaults::Breaks::new(".\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_443() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_444() {
    let mut breaks = defaults::Breaks::new(".'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_445() {
    let mut breaks = defaults::Breaks::new(".\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_446() {
    let mut breaks = defaults::Breaks::new(".\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_447() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_448() {
    let mut breaks = defaults::Breaks::new(".\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_449() {
    let mut breaks = defaults::Breaks::new(".\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_450() {
    let mut breaks = defaults::Breaks::new(".a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_451() {
    let mut breaks = defaults::Breaks::new(".\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_452() {
    let mut breaks = defaults::Breaks::new(".a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_453() {
    let mut breaks = defaults::Breaks::new(".\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_454() {
    let mut breaks = defaults::Breaks::new(".a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_455() {
    let mut breaks = defaults::Breaks::new(".\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_456() {
    let mut breaks = defaults::Breaks::new(".a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_457() {
    let mut breaks = defaults::Breaks::new(".\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_458() {
    let mut breaks = defaults::Breaks::new(".a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_459() {
    let mut breaks = defaults::Breaks::new(".\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_460() {
    let mut breaks = defaults::Breaks::new(".1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_461() {
    let mut breaks = defaults::Breaks::new(".\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_462() {
    let mut breaks = defaults::Breaks::new(".1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_463() {
    let mut breaks = defaults::Breaks::new(".\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_464() {
    let mut breaks = defaults::Breaks::new(".1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_465() {
    let mut breaks = defaults::Breaks::new(".\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_466() {
    let mut breaks = defaults::Breaks::new(".1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_467() {
    let mut breaks = defaults::Breaks::new(".\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some(".\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_468() {
    let mut breaks = defaults::Breaks::new("0\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_469() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_470() {
    let mut breaks = defaults::Breaks::new("0\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_471() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_472() {
    let mut breaks = defaults::Breaks::new("0\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_473() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_474() {
    let mut breaks = defaults::Breaks::new("0\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_475() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_476() {
    let mut breaks = defaults::Breaks::new("0\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_477() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_478() {
    let mut breaks = defaults::Breaks::new("0A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_479() {
    let mut breaks = defaults::Breaks::new("0\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_480() {
    let mut breaks = defaults::Breaks::new("0:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_481() {
    let mut breaks = defaults::Breaks::new("0\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_482() {
    let mut breaks = defaults::Breaks::new("0,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_483() {
    let mut breaks = defaults::Breaks::new("0\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_484() {
    let mut breaks = defaults::Breaks::new("0.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_485() {
    let mut breaks = defaults::Breaks::new("0\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_486() {
    let mut breaks = defaults::Breaks::new("00",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("00"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_487() {
    let mut breaks = defaults::Breaks::new("0\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_488() {
    let mut breaks = defaults::Breaks::new("0_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_489() {
    let mut breaks = defaults::Breaks::new("0\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_490() {
    let mut breaks = defaults::Breaks::new("0\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_491() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_492() {
    let mut breaks = defaults::Breaks::new("0\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_493() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_494() {
    let mut breaks = defaults::Breaks::new("0\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_495() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_496() {
    let mut breaks = defaults::Breaks::new("0'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_497() {
    let mut breaks = defaults::Breaks::new("0\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_498() {
    let mut breaks = defaults::Breaks::new("0\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_499() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_500() {
    let mut breaks = defaults::Breaks::new("0\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_501() {
    let mut breaks = defaults::Breaks::new("0\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_502() {
    let mut breaks = defaults::Breaks::new("0a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_503() {
    let mut breaks = defaults::Breaks::new("0\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_504() {
    let mut breaks = defaults::Breaks::new("0a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_505() {
    let mut breaks = defaults::Breaks::new("0\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_506() {
    let mut breaks = defaults::Breaks::new("0a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_507() {
    let mut breaks = defaults::Breaks::new("0\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_508() {
    let mut breaks = defaults::Breaks::new("0a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_509() {
    let mut breaks = defaults::Breaks::new("0\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_510() {
    let mut breaks = defaults::Breaks::new("0a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_511() {
    let mut breaks = defaults::Breaks::new("0\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_512() {
    let mut breaks = defaults::Breaks::new("01:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_513() {
    let mut breaks = defaults::Breaks::new("0\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_514() {
    let mut breaks = defaults::Breaks::new("01'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_515() {
    let mut breaks = defaults::Breaks::new("0\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_516() {
    let mut breaks = defaults::Breaks::new("01,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_517() {
    let mut breaks = defaults::Breaks::new("0\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_518() {
    let mut breaks = defaults::Breaks::new("01.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("01"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_519() {
    let mut breaks = defaults::Breaks::new("0\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("0\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_520() {
    let mut breaks = defaults::Breaks::new("_\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_521() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_522() {
    let mut breaks = defaults::Breaks::new("_\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_523() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_524() {
    let mut breaks = defaults::Breaks::new("_\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_525() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_526() {
    let mut breaks = defaults::Breaks::new("_\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_527() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_528() {
    let mut breaks = defaults::Breaks::new("_\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_529() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_530() {
    let mut breaks = defaults::Breaks::new("_A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_531() {
    let mut breaks = defaults::Breaks::new("_\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_532() {
    let mut breaks = defaults::Breaks::new("_:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_533() {
    let mut breaks = defaults::Breaks::new("_\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_534() {
    let mut breaks = defaults::Breaks::new("_,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_535() {
    let mut breaks = defaults::Breaks::new("_\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_536() {
    let mut breaks = defaults::Breaks::new("_.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_537() {
    let mut breaks = defaults::Breaks::new("_\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_538() {
    let mut breaks = defaults::Breaks::new("_0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_539() {
    let mut breaks = defaults::Breaks::new("_\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_540() {
    let mut breaks = defaults::Breaks::new("__",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("__"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_541() {
    let mut breaks = defaults::Breaks::new("_\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_542() {
    let mut breaks = defaults::Breaks::new("_\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_543() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_544() {
    let mut breaks = defaults::Breaks::new("_\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_545() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_546() {
    let mut breaks = defaults::Breaks::new("_\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_547() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_548() {
    let mut breaks = defaults::Breaks::new("_'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_549() {
    let mut breaks = defaults::Breaks::new("_\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_550() {
    let mut breaks = defaults::Breaks::new("_\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_551() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_552() {
    let mut breaks = defaults::Breaks::new("_\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_553() {
    let mut breaks = defaults::Breaks::new("_\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_554() {
    let mut breaks = defaults::Breaks::new("_a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_555() {
    let mut breaks = defaults::Breaks::new("_\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_556() {
    let mut breaks = defaults::Breaks::new("_a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_557() {
    let mut breaks = defaults::Breaks::new("_\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_558() {
    let mut breaks = defaults::Breaks::new("_a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_559() {
    let mut breaks = defaults::Breaks::new("_\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_560() {
    let mut breaks = defaults::Breaks::new("_a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_561() {
    let mut breaks = defaults::Breaks::new("_\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_562() {
    let mut breaks = defaults::Breaks::new("_a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_563() {
    let mut breaks = defaults::Breaks::new("_\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_564() {
    let mut breaks = defaults::Breaks::new("_1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_565() {
    let mut breaks = defaults::Breaks::new("_\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_566() {
    let mut breaks = defaults::Breaks::new("_1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_567() {
    let mut breaks = defaults::Breaks::new("_\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_568() {
    let mut breaks = defaults::Breaks::new("_1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_569() {
    let mut breaks = defaults::Breaks::new("_\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_570() {
    let mut breaks = defaults::Breaks::new("_1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_571() {
    let mut breaks = defaults::Breaks::new("_\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("_\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_572() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_573() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_574() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_575() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_576() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_577() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_578() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_579() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_580() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_581() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_582() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_583() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_584() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_585() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_586() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_587() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_588() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_589() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_590() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_591() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_592() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_593() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_594() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_595() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_596() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_597() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_598() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_599() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_600() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_601() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_602() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_603() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_604() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_605() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_606() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_607() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_608() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_609() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_610() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_611() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_612() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_613() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_614() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_615() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_616() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_617() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_618() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_619() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_620() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_621() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_622() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_623() {
    let mut breaks = defaults::Breaks::new("\u{1f1e6}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{1f1e6}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_624() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_625() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_626() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_627() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_628() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_629() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_630() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_631() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_632() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_633() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_634() {
    let mut breaks = defaults::Breaks::new("\u{5d0}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_635() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_636() {
    let mut breaks = defaults::Breaks::new("\u{5d0}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_637() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_638() {
    let mut breaks = defaults::Breaks::new("\u{5d0},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_639() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_640() {
    let mut breaks = defaults::Breaks::new("\u{5d0}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_641() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_642() {
    let mut breaks = defaults::Breaks::new("\u{5d0}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_643() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_644() {
    let mut breaks = defaults::Breaks::new("\u{5d0}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_645() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_646() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_647() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_648() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_649() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_650() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_651() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_652() {
    let mut breaks = defaults::Breaks::new("\u{5d0}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_653() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_654() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_655() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_656() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_657() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_658() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_659() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_660() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_661() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_662() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_663() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_664() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_665() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_666() {
    let mut breaks = defaults::Breaks::new("\u{5d0}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_667() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_668() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_669() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_670() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_671() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_672() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_673() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_674() {
    let mut breaks = defaults::Breaks::new("\u{5d0}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_675() {
    let mut breaks = defaults::Breaks::new("\u{5d0}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{5d0}\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_676() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_677() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_678() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_679() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_680() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_681() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_682() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_683() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_684() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_685() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_686() {
    let mut breaks = defaults::Breaks::new("\u{22}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_687() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_688() {
    let mut breaks = defaults::Breaks::new("\u{22}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_689() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_690() {
    let mut breaks = defaults::Breaks::new("\u{22},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_691() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_692() {
    let mut breaks = defaults::Breaks::new("\u{22}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_693() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_694() {
    let mut breaks = defaults::Breaks::new("\u{22}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_695() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_696() {
    let mut breaks = defaults::Breaks::new("\u{22}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_697() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_698() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_699() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_700() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_701() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_702() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_703() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_704() {
    let mut breaks = defaults::Breaks::new("\u{22}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_705() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_706() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_707() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_708() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_709() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_710() {
    let mut breaks = defaults::Breaks::new("\u{22}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_711() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_712() {
    let mut breaks = defaults::Breaks::new("\u{22}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_713() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_714() {
    let mut breaks = defaults::Breaks::new("\u{22}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_715() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_716() {
    let mut breaks = defaults::Breaks::new("\u{22}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_717() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_718() {
    let mut breaks = defaults::Breaks::new("\u{22}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_719() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_720() {
    let mut breaks = defaults::Breaks::new("\u{22}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_721() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_722() {
    let mut breaks = defaults::Breaks::new("\u{22}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_723() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_724() {
    let mut breaks = defaults::Breaks::new("\u{22}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_725() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_726() {
    let mut breaks = defaults::Breaks::new("\u{22}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_727() {
    let mut breaks = defaults::Breaks::new("\u{22}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{22}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_728() {
    let mut breaks = defaults::Breaks::new("'\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_729() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_730() {
    let mut breaks = defaults::Breaks::new("'\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_731() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_732() {
    let mut breaks = defaults::Breaks::new("'\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_733() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_734() {
    let mut breaks = defaults::Breaks::new("'\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_735() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_736() {
    let mut breaks = defaults::Breaks::new("'\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_737() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_738() {
    let mut breaks = defaults::Breaks::new("'A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_739() {
    let mut breaks = defaults::Breaks::new("'\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_740() {
    let mut breaks = defaults::Breaks::new("':",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_741() {
    let mut breaks = defaults::Breaks::new("'\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_742() {
    let mut breaks = defaults::Breaks::new("',",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_743() {
    let mut breaks = defaults::Breaks::new("'\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_744() {
    let mut breaks = defaults::Breaks::new("'.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_745() {
    let mut breaks = defaults::Breaks::new("'\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_746() {
    let mut breaks = defaults::Breaks::new("'0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_747() {
    let mut breaks = defaults::Breaks::new("'\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_748() {
    let mut breaks = defaults::Breaks::new("'_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_749() {
    let mut breaks = defaults::Breaks::new("'\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_750() {
    let mut breaks = defaults::Breaks::new("'\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_751() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_752() {
    let mut breaks = defaults::Breaks::new("'\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_753() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_754() {
    let mut breaks = defaults::Breaks::new("'\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_755() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_756() {
    let mut breaks = defaults::Breaks::new("''",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_757() {
    let mut breaks = defaults::Breaks::new("'\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_758() {
    let mut breaks = defaults::Breaks::new("'\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_759() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_760() {
    let mut breaks = defaults::Breaks::new("'\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_761() {
    let mut breaks = defaults::Breaks::new("'\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_762() {
    let mut breaks = defaults::Breaks::new("'a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_763() {
    let mut breaks = defaults::Breaks::new("'\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_764() {
    let mut breaks = defaults::Breaks::new("'a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_765() {
    let mut breaks = defaults::Breaks::new("'\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_766() {
    let mut breaks = defaults::Breaks::new("'a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_767() {
    let mut breaks = defaults::Breaks::new("'\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_768() {
    let mut breaks = defaults::Breaks::new("'a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_769() {
    let mut breaks = defaults::Breaks::new("'\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_770() {
    let mut breaks = defaults::Breaks::new("'a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_771() {
    let mut breaks = defaults::Breaks::new("'\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_772() {
    let mut breaks = defaults::Breaks::new("'1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_773() {
    let mut breaks = defaults::Breaks::new("'\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_774() {
    let mut breaks = defaults::Breaks::new("'1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_775() {
    let mut breaks = defaults::Breaks::new("'\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_776() {
    let mut breaks = defaults::Breaks::new("'1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_777() {
    let mut breaks = defaults::Breaks::new("'\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_778() {
    let mut breaks = defaults::Breaks::new("'1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_779() {
    let mut breaks = defaults::Breaks::new("'\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_780() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_781() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_782() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_783() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_784() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_785() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_786() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_787() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_788() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_789() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_790() {
    let mut breaks = defaults::Breaks::new("\u{ad}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_791() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_792() {
    let mut breaks = defaults::Breaks::new("\u{ad}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_793() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_794() {
    let mut breaks = defaults::Breaks::new("\u{ad},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_795() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_796() {
    let mut breaks = defaults::Breaks::new("\u{ad}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_797() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_798() {
    let mut breaks = defaults::Breaks::new("\u{ad}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_799() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_800() {
    let mut breaks = defaults::Breaks::new("\u{ad}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_801() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_802() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_803() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_804() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_805() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_806() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_807() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_808() {
    let mut breaks = defaults::Breaks::new("\u{ad}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_809() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_810() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_811() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_812() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_813() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_814() {
    let mut breaks = defaults::Breaks::new("\u{ad}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_815() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_816() {
    let mut breaks = defaults::Breaks::new("\u{ad}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_817() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_818() {
    let mut breaks = defaults::Breaks::new("\u{ad}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_819() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_820() {
    let mut breaks = defaults::Breaks::new("\u{ad}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_821() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_822() {
    let mut breaks = defaults::Breaks::new("\u{ad}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_823() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_824() {
    let mut breaks = defaults::Breaks::new("\u{ad}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_825() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_826() {
    let mut breaks = defaults::Breaks::new("\u{ad}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_827() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_828() {
    let mut breaks = defaults::Breaks::new("\u{ad}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_829() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_830() {
    let mut breaks = defaults::Breaks::new("\u{ad}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_831() {
    let mut breaks = defaults::Breaks::new("\u{ad}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{ad}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_832() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_833() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_834() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_835() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_836() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_837() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_838() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_839() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_840() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_841() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_842() {
    let mut breaks = defaults::Breaks::new("\u{300}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_843() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_844() {
    let mut breaks = defaults::Breaks::new("\u{300}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_845() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_846() {
    let mut breaks = defaults::Breaks::new("\u{300},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_847() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_848() {
    let mut breaks = defaults::Breaks::new("\u{300}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_849() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_850() {
    let mut breaks = defaults::Breaks::new("\u{300}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_851() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_852() {
    let mut breaks = defaults::Breaks::new("\u{300}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_853() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_854() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_855() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_856() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_857() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_858() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_859() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_860() {
    let mut breaks = defaults::Breaks::new("\u{300}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_861() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_862() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_863() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_864() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_865() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_866() {
    let mut breaks = defaults::Breaks::new("\u{300}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_867() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_868() {
    let mut breaks = defaults::Breaks::new("\u{300}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_869() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_870() {
    let mut breaks = defaults::Breaks::new("\u{300}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_871() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_872() {
    let mut breaks = defaults::Breaks::new("\u{300}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_873() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_874() {
    let mut breaks = defaults::Breaks::new("\u{300}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_875() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_876() {
    let mut breaks = defaults::Breaks::new("\u{300}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_877() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_878() {
    let mut breaks = defaults::Breaks::new("\u{300}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_879() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_880() {
    let mut breaks = defaults::Breaks::new("\u{300}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_881() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_882() {
    let mut breaks = defaults::Breaks::new("\u{300}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_883() {
    let mut breaks = defaults::Breaks::new("\u{300}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("\u{300}\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_884() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_885() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_886() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_887() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_888() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_889() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_890() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_891() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_892() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_893() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_894() {
    let mut breaks = defaults::Breaks::new("a\u{2060}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_895() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_896() {
    let mut breaks = defaults::Breaks::new("a\u{2060}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_897() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_898() {
    let mut breaks = defaults::Breaks::new("a\u{2060},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_899() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_900() {
    let mut breaks = defaults::Breaks::new("a\u{2060}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_901() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_902() {
    let mut breaks = defaults::Breaks::new("a\u{2060}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_903() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_904() {
    let mut breaks = defaults::Breaks::new("a\u{2060}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_905() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_906() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_907() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_908() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_909() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_910() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_911() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_912() {
    let mut breaks = defaults::Breaks::new("a\u{2060}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_913() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_914() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_915() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_916() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_917() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_918() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_919() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_920() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_921() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_922() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_923() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_924() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_925() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_926() {
    let mut breaks = defaults::Breaks::new("a\u{2060}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_927() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_928() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_929() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_930() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_931() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_932() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_933() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_934() {
    let mut breaks = defaults::Breaks::new("a\u{2060}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_935() {
    let mut breaks = defaults::Breaks::new("a\u{2060}\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a\u{2060}\u{308}1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_936() {
    let mut breaks = defaults::Breaks::new("a:\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_937() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_938() {
    let mut breaks = defaults::Breaks::new("a:\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_939() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_940() {
    let mut breaks = defaults::Breaks::new("a:\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_941() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_942() {
    let mut breaks = defaults::Breaks::new("a:\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_943() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_944() {
    let mut breaks = defaults::Breaks::new("a:\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_945() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_946() {
    let mut breaks = defaults::Breaks::new("a:A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_947() {
    let mut breaks = defaults::Breaks::new("a:\u{308}A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_948() {
    let mut breaks = defaults::Breaks::new("a::",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_949() {
    let mut breaks = defaults::Breaks::new("a:\u{308}:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_950() {
    let mut breaks = defaults::Breaks::new("a:,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_951() {
    let mut breaks = defaults::Breaks::new("a:\u{308},",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_952() {
    let mut breaks = defaults::Breaks::new("a:.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_953() {
    let mut breaks = defaults::Breaks::new("a:\u{308}.",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("."));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_954() {
    let mut breaks = defaults::Breaks::new("a:0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_955() {
    let mut breaks = defaults::Breaks::new("a:\u{308}0",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("0"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_956() {
    let mut breaks = defaults::Breaks::new("a:_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_957() {
    let mut breaks = defaults::Breaks::new("a:\u{308}_",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("_"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_958() {
    let mut breaks = defaults::Breaks::new("a:\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_959() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{1f1e6}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1f1e6}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_960() {
    let mut breaks = defaults::Breaks::new("a:\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_961() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{5d0}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}\u{5d0}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_962() {
    let mut breaks = defaults::Breaks::new("a:\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_963() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{22}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("\u{22}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_964() {
    let mut breaks = defaults::Breaks::new("a:'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_965() {
    let mut breaks = defaults::Breaks::new("a:\u{308}'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_966() {
    let mut breaks = defaults::Breaks::new("a:\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_967() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{ad}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}\u{ad}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_968() {
    let mut breaks = defaults::Breaks::new("a:\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_969() {
    let mut breaks = defaults::Breaks::new("a:\u{308}\u{300}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}\u{300}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_970() {
    let mut breaks = defaults::Breaks::new("a:a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_971() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_972() {
    let mut breaks = defaults::Breaks::new("a:a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_973() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_974() {
    let mut breaks = defaults::Breaks::new("a:a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_975() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_976() {
    let mut breaks = defaults::Breaks::new("a:a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_977() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a'\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some("'\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_978() {
    let mut breaks = defaults::Breaks::new("a:a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_979() {
    let mut breaks = defaults::Breaks::new("a:\u{308}a,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a:\u{308}a"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_980() {
    let mut breaks = defaults::Breaks::new("a:1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_981() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1:",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_982() {
    let mut breaks = defaults::Breaks::new("a:1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_983() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1'",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_984() {
    let mut breaks = defaults::Breaks::new("a:1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_985() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1,",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(","));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_986() {
    let mut breaks = defaults::Breaks::new("a:1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_987() {
    let mut breaks = defaults::Breaks::new("a:\u{308}1.\u{2060}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some(":\u{308}"));
    assert_eq!(breaks.next(), Some("1"));
    assert_eq!(breaks.next(), Some(".\u{2060}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_988() {
    let mut breaks = defaults::Breaks::new("a'\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_989() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{1}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{1}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_990() {
    let mut breaks = defaults::Breaks::new("a'\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_991() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{d}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{d}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_992() {
    let mut breaks = defaults::Breaks::new("a'\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_993() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{a}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{a}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_994() {
    let mut breaks = defaults::Breaks::new("a'\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_995() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{b}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{b}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_996() {
    let mut breaks = defaults::Breaks::new("a'\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_997() {
    let mut breaks = defaults::Breaks::new("a'\u{308}\u{3031}",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a"));
    assert_eq!(breaks.next(), Some("'\u{308}"));
    assert_eq!(breaks.next(), Some("\u{3031}"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_998() {
    let mut breaks = defaults::Breaks::new("a'A",
        defaults::make_word_break_tree());
    assert_eq!(breaks.next(), Some("a'A"));
    assert_eq!(breaks.next(), None);
}

#[test]
fn test_999() {
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

