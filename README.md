* David Corbett `<corbett.dav@husky.neu.edu>`
* Mimi Lin `<hloople@gmail.com>`

This is an implementation of a word-breaking algorithm in Rust following [Unicode
Standard Annex #29](http://www.unicode.org/reports/tr29/) as an extension of the
C++ library [`boyers/unicode`](https://github.com/boyers/unicode). It also
includes a barebones C++ version of `fmt`.

`README.md` is this file.

`fmt` is the directory for the `fmt` program. In it are `fmt.cpp` (the source
code) and `Makefile`. Run `make` to make it and `./fmt filename max_width` to
use it.

`unicode` is the modified copy of `boyers/unicode`. The modification is the
addition of the two files `word_break.cpp` and `word_break.h`. These are used by
`fmt`.

`uax29` is the Rust project directory. `uax29/Cargo.toml` and `uax29/Cargo.lock`
are Rust project metadata files.

In `uax29/src`, `main.rs` can be ignored. This is supposed to be a library, so it
should not have a main file. `lib.rs` is the library file. `tests.rs` is a file
of tests. `defaults.rs` is the primary file of the library. `breaks.rs` contains
information about word sentence breaks. `c_defaults.rs` contains functions
for using this library from C++.

`uax29/scripts` contains scripts. `unicode.py` creates `breaks.rs`.
`generate_tests.py` creates `tests.rs`.
