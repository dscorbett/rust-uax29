#include <iostream>
#include <fstream>
#include <stdlib.h>
#include <streambuf>
#include <string>
#include <dlfcn.h>
//#include "fmt.h"
#include "../unicode/utf.h"
#include "../unicode/utf.cpp"

struct word_breaker;

int main(int argc, char *argv[]) {
    // quick and dirty
    void* handle = dlopen("./wordbreaker.dylib", RTLD_LAZY);
    std::cout << "about to create handle" << std::endl;
    if (!handle) {
        std::cerr << "Cannot open library: " << dlerror() << '\n';
        return 1;
    }
/*
    typedef int32_t (*treble_t)(int32_t);
    typedef circle* (*create_circle_t)(int32_t, int32_t, int32_t, char*);
    typedef int32_t (*circle_diameter_t)(circle*);
    typedef char* (*circle_name_t)(circle*);
*/
    typedef word_breaker* (*create_word_breaker_t)(char*);
    typedef char* (*next_word_t)(word_breaker*);

    dlerror();
    /*
    create_circle_t create_circle = (create_circle_t) dlsym(handle, "create_circle");
    circle_diameter_t circle_diameter = (circle_diameter_t) dlsym(handle, "circle_diameter");
    circle_name_t circle_name = (circle_name_t) dlsym(handle, "circle_name");
*/
    create_word_breaker_t create_word_breaker = (create_word_breaker_t) dlsym(handle, "create_word_breaker");
    next_word_t next_word = (next_word_t) dlsym(handle, "next_word");

    const char* dlsym_error = dlerror();
    if (dlsym_error) {
        std::cerr << "Cannot load symbol" << dlsym_error <<
            '\n';
        dlclose(handle);
        return 1;
    }
/*
    std::cout <<"calling treble from rust generated dylib" << std::endl;
    std::cout << treble(4) << std::endl;
    std::cout <<"testing out circle" << std::endl;
    std::cout << circle_diameter(create_circle(1, 2, 3, "foobar")) << std::endl;
    std::cout <<"name is" <<std::endl;
    std::cout << circle_name(create_circle(1, 2, 3, "barbaz")) << std::endl;
*/
    std::cout <<"calling dylib function" << std::endl;
    word_breaker* _wb = create_word_breaker("1.21 gigawatts.");
    std::cout << next_word(_wb) << std::endl;
    std::cout << next_word(_wb) << std::endl;
    std::cout << next_word(_wb) << std::endl;
    









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
