SHELL := /bin/bash

all: fmt

fmt:
	g++ fmt.cpp -I ../unicode  -o fmt

clean:
	rm -f fmt *~

test: fmt
	./fmt <(echo 'abc def_ghi') 7
	./fmt <(echo 'abc def ghi') 7
	./fmt <(echo -e 'abc def\xe2\x80\x83ghi') 7
