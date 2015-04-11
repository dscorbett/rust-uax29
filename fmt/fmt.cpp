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


    word_break wb("1.21 gigawatts."); // convert std::string to char* str.c_str()
    std::cout << wb.next() << std::endl;
    std::cout << wb.next() << std::endl;
    std::cout << wb.next() << std::endl;
    std::cout << wb.next() << std::endl;


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
    std::string current_space = "";
    std::string current_word = "";
    const size_t total_length = utf::get_length(str, encoding);
    for (size_t i = 0, byte_i = 0; i <= total_length;
         i++, byte_i += i == total_length ? 0 :
         utf::get_char_size(str, byte_i, encoding))
    {
        const uint32_t code_point =
            i == total_length ? 0 : utf::get_char(str, byte_i, encoding);
        if (i == total_length || utf::is_whitespace(code_point)) {
            const unsigned long current_space_length =
                utf::get_length(current_space, encoding);
            const unsigned long current_word_length =
                utf::get_length(current_word, encoding);
            if (line_width + current_space_length + current_word_length >
                max_width)
            {
                std::cout << std::endl;
                line_width = 0;
            } else {
                std::cout << current_space;
                line_width += current_space_length;
            }
            std::cout << current_word;
            line_width += current_word_length;
            current_space = "";
            current_word = "";
            if (i != total_length && utf::is_newline(code_point)) {
                std::cout << std::endl;
                line_width = 0;
            } else {
                add_char(current_space, code_point, encoding);
            }
        } else {
            add_char(current_word, code_point, encoding);
        }
    }
}
