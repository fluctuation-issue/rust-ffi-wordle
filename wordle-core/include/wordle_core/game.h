#ifndef WORDLE_CORE_GAME_H
#define WORDLE_CORE_GAME_H

#include <stdint.h>

#include "hint.h"

typedef enum wc_game_state {
    WC_GAME_STATE_PENDING,
    WC_GAME_STATE_WON,
    WC_GAME_STATE_LOST,
} wc_game_state;

typedef void* wc_game_t;

typedef struct wc_guess_hint_list_node_t {
    wc_guess_hint_t current;
    struct wc_guess_hint_list_node_t* next;
} wc_guess_hint_list_node_t;

typedef enum wc_game_guess_error {
    WC_GAME_GUESS_ERROR_LENGTH_INVALID,
    WC_GAME_GUESS_ERROR_ALREADY_PLAYED,
} wc_game_guess_error;

wc_game_t wc_game_new(char const* word_to_guess);
wc_game_t wc_game_new_with_attempts_count_limit(char const* word_to_guess, uint32_t attempts_count_limit);
void wc_game_free(wc_game_t game);
char* wc_game_get_word_to_guess(const wc_game_t Game);
wc_game_state wc_game_get_state(const wc_game_t Game);
wc_guess_hint_t wc_game_get_current_guess_hint(const wc_game_t Game);
void wc_guess_hint_free(wc_guess_hint_t guess_hint);
wc_guess_hint_list_node_t* wc_game_get_guess_hints(const wc_game_t Game);
void wc_game_guess_hints_free(wc_guess_hint_list_node_t *node);
int wc_game_guess(wc_game_t Game, char const* guessed_word, wc_game_guess_error *error, wc_game_state *new_state);

void rust_str_free(char *string);

#endif
