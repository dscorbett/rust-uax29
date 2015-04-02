#!/usr/bin/env python
#
# Copyright 2011-2013 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# Based on https://raw.githubusercontent.com/rust-lang/regex/d25c39f86568a147f9b7080c25711fb1f98f056a/scripts/unicode.py

import fileinput, re, os, sys, operator

preamble = '''// NOTE: The following code was generated by "scripts/unicode.py", do not edit
// directly

#![allow(warnings)]

'''

def fetch(f):
#    if not os.path.exists(f):
#        os.system("curl -O http://www.unicode.org/Public/UNIDATA/%s"
#                  % f)
    if not os.path.exists(f):
        os.system('curl -O http://www.unicode.org/Public/7.0.0/ucd/auxiliary/%s' % f)
    if not os.path.exists(f):
        sys.stderr.write("cannot load %s" % f)
        exit(1)

def is_surrogate(n):
    return 0xD800 <= n <= 0xDFFF

def load_unicode_data(f):
    fetch(f)
    gencats = {}
    combines = {}

    udict = {};
    range_start = -1;
    for line in fileinput.input(f):
        data = line.split(';');
        if len(data) != 15:
            continue
        cp = int(data[0], 16);
        if is_surrogate(cp):
            continue
        if range_start >= 0:
            for i in xrange(range_start, cp):
                udict[i] = data;
            range_start = -1;
        if data[1].endswith(", First>"):
            range_start = cp;
            continue;
        udict[cp] = data;

    for code in udict:
        [code_org, name, gencat, combine, bidi,
         decomp, deci, digit, num, mirror,
         old, iso, upcase, lowcase, titlecase ] = udict[code];

        # place letter in categories as appropriate
        for cat in [gencat, "Assigned"] + expanded_categories.get(gencat, []):
            if cat not in gencats:
                gencats[cat] = []
            gencats[cat].append(code)

        # record combining class, if any
        if combine != "0":
            if combine not in combines:
                combines[combine] = []
            combines[combine].append(code)

    # generate Not_Assigned from Assigned
    gencats["Cn"] = gen_unassigned(gencats["Assigned"])
    # Assigned is not a real category
    del(gencats["Assigned"])
    # Other contains Not_Assigned
    gencats["C"].extend(gencats["Cn"])
    gencats = group_cats(gencats)
    combines = to_combines(group_cats(combines))

    return (gencats, combines)

def group_cats(cats):
    cats_out = {}
    for cat in cats:
        cats_out[cat] = group_cat(cats[cat])
    return cats_out

def group_cat(cat):
    cat_out = []
    letters = sorted(set(cat))
    cur_start = letters.pop(0)
    cur_end = cur_start
    for letter in letters:
        assert letter > cur_end, \
            "cur_end: %s, letter: %s" % (hex(cur_end), hex(letter))
        if letter == cur_end + 1:
            cur_end = letter
        else:
            cat_out.append((cur_start, cur_end))
            cur_start = cur_end = letter
    cat_out.append((cur_start, cur_end))
    return cat_out

def ungroup_cat(cat):
    cat_out = []
    for (lo, hi) in cat:
        while lo <= hi:
            cat_out.append(lo)
            lo += 1
    return cat_out

def gen_unassigned(assigned):
    assigned = set(assigned)
    return ([i for i in range(0, 0xd800) if i not in assigned] +
            [i for i in range(0xe000, 0x110000) if i not in assigned])

def to_combines(combs):
    combs_out = []
    for comb in combs:
        for (lo, hi) in combs[comb]:
            combs_out.append((lo, hi, comb))
    combs_out.sort(key=lambda comb: comb[0])
    return combs_out

def format_table_content(f, content, indent):
    line = " "*indent
    first = True
    for chunk in content.split(","):
        if len(line) + len(chunk) < 78:
            if first:
                line += chunk
            else:
                line += ", " + chunk
            first = False
        else:
            f.write(line + ",\n")
            line = " "*indent + chunk
    f.write(line)

def load_properties(f, interestingprops):
    fetch(f)
    props = {}
    re1 = re.compile("^ *([0-9A-F]+) *; *(\w+)")
    re2 = re.compile("^ *([0-9A-F]+)\.\.([0-9A-F]+) *; *(\w+)")

    for line in fileinput.input(f):
        prop = None
        d_lo = 0
        d_hi = 0
        m = re1.match(line)
        if m:
            d_lo = m.group(1)
            d_hi = m.group(1)
            prop = m.group(2)
        else:
            m = re2.match(line)
            if m:
                d_lo = m.group(1)
                d_hi = m.group(2)
                prop = m.group(3)
            else:
                continue
        if interestingprops and prop not in interestingprops:
            continue
        d_lo = int(d_lo, 16)
        d_hi = int(d_hi, 16)
        if prop not in props:
            props[prop] = []
        props[prop].append((d_lo, d_hi))
    return props

# load all widths of want_widths, except those in except_cats
def load_east_asian_width(want_widths, except_cats):
    f = "EastAsianWidth.txt"
    fetch(f)
    widths = {}
    re1 = re.compile("^([0-9A-F]+);(\w+) +# (\w+)")
    re2 = re.compile("^([0-9A-F]+)\.\.([0-9A-F]+);(\w+) +# (\w+)")

    for line in fileinput.input(f):
        width = None
        d_lo = 0
        d_hi = 0
        cat = None
        m = re1.match(line)
        if m:
            d_lo = m.group(1)
            d_hi = m.group(1)
            width = m.group(2)
            cat = m.group(3)
        else:
            m = re2.match(line)
            if m:
                d_lo = m.group(1)
                d_hi = m.group(2)
                width = m.group(3)
                cat = m.group(4)
            else:
                continue
        if cat in except_cats or width not in want_widths:
            continue
        d_lo = int(d_lo, 16)
        d_hi = int(d_hi, 16)
        if width not in widths:
            widths[width] = []
        widths[width].append((d_lo, d_hi))
    return widths

def escape_char(c):
    return "'\\u{%x}'" % c

def emit_table(f, name, t_data, t_type = "&'static [(char, char)]", is_pub=True,
        pfun=lambda x: "(%s,%s)" % (escape_char(x[0]), escape_char(x[1]))):
    pub_string = ""
    if is_pub:
        pub_string = "pub "
    f.write("    %sconst %s: %s = &[\n" % (pub_string, name, t_type))
    data = ""
    first = True
    for dat in t_data:
        if not first:
            data += ","
        first = False
        data += pfun(dat)
    format_table_content(f, data, 8)
    f.write("\n    ];\n\n")

def emit_property_module(f, name, cat, _):
    f.write("""pub mod %s {
    use std::slice::SliceExt;
    pub use self::Category::*;
    use std::result::Result::{Ok, Err};
    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, PartialEq, Show)]
    pub enum Category {
""" % name)
    for cat_name in cat:
        f.write("        " + cat_name + ",\n")
    f.write('''    }

    fn bsearch_range_value_table(c: char, r: &'static [(char, char, Category)])
        -> Option<Category>
    {
        use std::cmp::Ordering::{Equal, Less, Greater};
        match r.binary_search_by(|&(lo, hi, _)| {
            if lo <= c && c <= hi { Equal }
            else if hi < c { Less }
            else { Greater }
        }) {
            Ok(idx) => Some(r[idx].2),
            Err(_) => None,
        }
    }
    pub fn category(c: char) -> Option<Category> {
        bsearch_range_value_table(c, cat_table)
    }

''')
    data = []
    for (k, v) in cat.iteritems():
        for rng in v:
            data.append((rng, k))
    data.sort()
    emit_table(f, 'cat_table', data, "&'static [(char, char, Category)]",
               pfun=lambda x: '({}, {}, {})'.format(escape_char(x[0][0]),
                                                    escape_char(x[0][1]),
                                                    x[1]),
               is_pub=False)
    f.write('}\n')

def emit_regex_module(f, cats, w_data):
    f.write("pub mod regex {\n")
    regex_class = "&'static [(char, char)]"
    class_table = "&'static [(&'static str, %s)]" % regex_class

    emit_table(f, "UNICODE_CLASSES", cats, class_table,
        pfun=lambda x: "(\"%s\",super::%s::%s_table)" % (x[0], x[1], x[0]))

    f.write("    pub const PERLD: %s = super::general_category::Nd_table;\n\n"
            % regex_class)
    f.write("    pub const PERLS: %s = super::property::White_Space_table;\n\n"
            % regex_class)

    emit_table(f, "PERLW", w_data, regex_class)

    f.write("}\n\n")

def remove_from_wtable(wtable, val):
    wtable_out = []
    while wtable:
        if wtable[0][1] < val:
            wtable_out.append(wtable.pop(0))
        elif wtable[0][0] > val:
            break
        else:
            (wt_lo, wt_hi, width, width_cjk) = wtable.pop(0)
            if wt_lo == wt_hi == val:
                continue
            elif wt_lo == val:
                wtable_out.append((wt_lo+1, wt_hi, width, width_cjk))
            elif wt_hi == val:
                wtable_out.append((wt_lo, wt_hi-1, width, width_cjk))
            else:
                wtable_out.append((wt_lo, val-1, width, width_cjk))
                wtable_out.append((val+1, wt_hi, width, width_cjk))
    if wtable:
        wtable_out.extend(wtable)
    return wtable_out



def optimize_width_table(wtable):
    wtable_out = []
    w_this = wtable.pop(0)
    while wtable:
        if w_this[1] == wtable[0][0] - 1 and w_this[2:3] == wtable[0][2:3]:
            w_tmp = wtable.pop(0)
            w_this = (w_this[0], w_tmp[1], w_tmp[2], w_tmp[3])
        else:
            wtable_out.append(w_this)
            w_this = wtable.pop(0)
    wtable_out.append(w_this)
    return wtable_out

if __name__ == "__main__":
    try:
        r = sys.argv[1]
    except:
        sys.stderr.write('Usage: {} filename\n'.format(sys.argv[0]))
        exit(1)
    if os.path.exists(r):
        os.remove(r)
    with open(r, "w") as rf:
        # write the file's preamble
        rf.write(preamble)

        # download and parse all the data
        wb = load_properties('WordBreakProperty.txt', [])
        sb = load_properties('SentenceBreakProperty.txt', [])

        allcats = []
        for (name, cat, pfuns) in ("word_break", wb, []), \
                                  ('sentence_break', sb, []):
            emit_property_module(rf, name, cat, pfuns)
            allcats.extend(map(lambda x: (x, name), cat))
