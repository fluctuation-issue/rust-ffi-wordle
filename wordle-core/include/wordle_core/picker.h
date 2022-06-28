#ifndef WORDLE_CORE_PICKER_H
#define WORDLE_CORE_PICKER_H

typedef void* wc_word_picker_t;

wc_word_picker_t wc_word_picker_new_from_list(char **words);
wc_word_picker_t wc_word_picker_new_random_line_file(char* path);
char* wc_word_picker_pick_word(wc_word_picker_t picker);
void wc_word_picker_free(wc_word_picker_t picker);

#endif
