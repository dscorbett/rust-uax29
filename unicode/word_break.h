#ifndef WORDBREAK_H
#define WORDBREAK_H

struct rust_handle;
typedef rust_handle* (*create_wb_t)(const char*);
typedef const char* (*next_word_t)(rust_handle*);

class word_break {

    public:
    	word_break(const char* str);
    	~word_break();
    	const char* next();

    private: 
        void* dll_handle;
        rust_handle* r_handle;
        create_wb_t fn_create_word_breaker;
        next_word_t fn_next_word;
        void close_dll();
};
#endif