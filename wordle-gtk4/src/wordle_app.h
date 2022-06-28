#ifndef WORDLE_APP_H
#define WORDLE_APP_H

#include <gtk/gtk.h>

#define WORDLE_APP_TYPE (wordle_app_get_type())
G_DECLARE_FINAL_TYPE(WordleApp, wordle_app, WORDLE, APP, GtkApplication)

WordleApp* wordle_app_new(void);

#endif
