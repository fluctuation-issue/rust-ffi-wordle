#ifndef WORLDE_HINTS_H
#define WORLDE_HINTS_H

#include <gtk/gtk.h>

#include "wordle_core/game.h"

#define WORDLE_HINTS_TYPE (wordle_hints_get_type())

G_DECLARE_FINAL_TYPE(WordleHints, wordle_hints, WORDLE, HINTS, GtkBox)

WordleHints* wordle_hints_new(void);
void wordle_hints_clear(WordleHints *wordle_hints);
void wordle_hints_reset(WordleHints *wordle_hints, int guess_word_length);
void wordle_hints_add_guess_row(WordleHints *wordle_hints, const wc_guess_hint_t guess_hint);

#endif
