/* Class word_break
 * 
 * word_break implements the iterator which follows 
 * the default UAX #29 word-breaking specification.
 *
 */

#ifndef WORDBREAK_H
#define WORDBREAK_H

struct rust_handle;
typedef rust_handle *(*create_wb_t)(const char *);
typedef const char *(*next_word_t)(rust_handle *);

class word_break {

    public:
        // Construct the word_break iterator
        word_break(const char *str);
        
        // Destruct the word_break_iterator
        ~word_break();
        
        // Get the next work in the iterator
        const char *next();

    private: 
        void *dll_handle;
        rust_handle *r_handle;
        create_wb_t fn_create_word_breaker;
        next_word_t fn_next_word;

        // Close and nullify the links to the dll file
        void close_dll();
};
#endif
