SHELL := /bin/bash
DYLIB_NAME := wordbreaker.dylib
MANIFEST_PATH := ../uax29/Cargo.toml

all: fmt

fmt:
	cargo build --release --manifest-path ${MANIFEST_PATH}
	cp ../uax29/target/release/libuax29-*.so ${DYLIB_NAME}
	g++ fmt.cpp -I ../unicode ../unicode/utf.cpp ../unicode/word_break.cpp -Wl,--no-as-needed -ldl -o fmt

clean:
	rm -f fmt ${DYLIB_NAME} *~
	cargo clean --manifest-path ${MANIFEST_PATH}

test: fmt
	./fmt <(echo 'abc def_ghi') 7
	./fmt <(echo 'abc def ghi') 7
	./fmt <(echo -e 'abc def\xe2\x80\x83ghi') 7
	./fmt <(echo -e 'a"b') 1
	./fmt <(echo -e '\xd7\x90"\xd7\x91') 1
