#ifndef WORDLE_CORE_HINT_H
#define WORDLE_CORE_HINT_H

#include <stdint.h>

typedef void* wc_guess_hint_t;

typedef enum wc_letter_hint {
    WC_LETTER_HINT_CORRECT,
    WC_LETTER_HINT_PLACEMENT_INCORRECT,
    WC_LETTER_HINT_INCORRECT,
} wc_letter_hint;

typedef struct wc_letter_hints {
    wc_letter_hint* hints;
    uint32_t num_hints;
} wc_letter_hints;

typedef struct wc_guessed_letter_and_hint {
    char letter;
    wc_letter_hint hint;
} wc_guessed_letter_and_hint;

typedef struct wc_guessed_letters_and_hints {
    wc_guessed_letter_and_hint* hints;
    uint32_t num_letters_and_hints;
} wc_guessed_letters_and_hints;

wc_letter_hints* wc_guess_hint_get_letter_hints(const wc_guess_hint_t guess_hint);
void wc_guess_hint_free_letter_hints(wc_letter_hints *letter_hints);
wc_guessed_letters_and_hints* wc_guess_hint_get_guessed_letters_and_hints(const wc_guess_hint_t guess_hint);
void wc_guess_hint_free_guessed_letters_and_hints(wc_guessed_letters_and_hints *letters_and_hints);
char* wc_guess_hint_get_guessed(wc_guess_hint_t guess_hint);

#endif
