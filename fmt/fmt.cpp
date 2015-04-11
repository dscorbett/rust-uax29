#include <iostream>
#include <fstream>
#include <stdlib.h>
#include <streambuf>
#include <string>
#include <dlfcn.h>
//#include "fmt.h"
#include "../unicode/utf.h"
#include "../unicode/utf.cpp"
#include "../unicode/word_break.h"

int main(int argc, char *argv[]) {

    if (argc != 3) {
        std::cout << "usage: " << argv[0] << " filename max_width" << std::endl;
        return 1;
    }
    std::ifstream t(argv[1]);
    const unsigned long max_width = strtoul(argv[2], NULL, 0);
    std::string str((std::istreambuf_iterator<char>(t)),
                    std::istreambuf_iterator<char>());
    utf::encoding_type encoding = utf::detect_encoding(str);
    unsigned long line_width = 0;
    word_break wb(str.c_str());
    char * next = wb.next();

    while(next != NULL) {
        std::string current_word(next);
        line_width = line_width + utf::get_length(current_word, encoding);
        const uint32_t code_point = utf::get_char(current_word, 0, encoding);
        if(line_width > max_width || utf::is_newline(code_point)) {
            std::cout << "\n";
            line_width = 0;
        }
        std::cout << current_word;
        next = wb.next();
    }
}
