#include "word_break.h"
#include <iostream>
#include <dlfcn.h>

#define DLL_PATH "./wordbreaker.dylib"
// DLL Symbols
#define DLL_WORD_BREAKER "create_word_breaker"
#define DLL_NEXT "next_word"

word_break::word_break(const char* str){
    dll_handle = dlopen(DLL_PATH, RTLD_LAZY);
    if (!dll_handle) {
        std::cerr << "Cannot open dll: " << dlerror() << '\n';
    }
    fn_create_word_breaker = 
        (create_wb_t) dlsym(dll_handle, DLL_WORD_BREAKER);
    fn_next_word = 
        (next_word_t) dlsym(dll_handle, DLL_NEXT);
        
    const char* dlsym_error = dlerror();
    if (dlsym_error) {
        std::cerr << "Cannot load dll symbol" << dlsym_error << '\n';
        dlclose(dll_handle);
    }
    r_handle = fn_create_word_breaker(str);
}

const char* word_break::next(){
    return fn_next_word(r_handle);
}

void word_break::close_dll(){
    dlclose(dll_handle);
    fn_create_word_breaker = NULL;
    fn_next_word = NULL;
    r_handle = NULL;
}

word_break::~word_break(){
    close_dll();
}