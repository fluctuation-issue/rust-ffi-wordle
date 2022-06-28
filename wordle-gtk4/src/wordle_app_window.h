#ifndef WORDLE_APP_WINDOW_H
#define WORDLE_APP_WINDOW_H

#include <gtk/gtk.h>
#include "wordle_app.h"

#define WORDLE_APP_WINDOW_TYPE (wordle_app_window_get_type())
G_DECLARE_FINAL_TYPE(WordleAppWindow, wordle_app_window, WORDLE, APP_WINDOW, GtkApplicationWindow)

WordleAppWindow* wordle_app_window_new(WordleApp *app);
void wordle_app_window_open(WordleAppWindow* window, GFile *file);

#endif
