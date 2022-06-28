#include "wordle_app.h"
#include "wordle_app_window.h"

struct _WordleApp {
    GtkApplication parent;
};

G_DEFINE_TYPE(WordleApp, wordle_app, GTK_TYPE_APPLICATION)

static void wordle_app_init(WordleApp *wordle_app);
static void wordle_app_class_init(WordleAppClass *class);
static void wordle_app_activate(GApplication *application);
static void wordle_app_open(GApplication *app, GFile **files, int n_files, const char *hint);

WordleApp* wordle_app_new(void)
{
	return g_object_new(WORDLE_APP_TYPE, "application-id", "local.imgt.wordle", "flags", G_APPLICATION_HANDLES_OPEN, NULL);
}

static void wordle_app_init(WordleApp *wordle_app)
{
    (void) wordle_app;
}

static void wordle_app_class_init(WordleAppClass *class)
{
    G_APPLICATION_CLASS(class)->activate = wordle_app_activate;
    G_APPLICATION_CLASS(class)->open = wordle_app_open;
}

static void wordle_app_activate(GApplication *application)
{
    WordleAppWindow *window;
    window = wordle_app_window_new(WORDLE_APP(application));
    gtk_window_present(GTK_WINDOW(window));
}

static void wordle_app_open(GApplication *app, GFile **files, int n_files, const char *hint)
{
    (void) hint;

    GList *windows;
    WordleAppWindow *window;

    windows = gtk_application_get_windows(GTK_APPLICATION(app));
    if (windows)
        window = WORDLE_APP_WINDOW(windows->data);
    else
        window = wordle_app_window_new(WORDLE_APP(app));

    if (n_files > 0)
        wordle_app_window_open(window, files[0]);
    gtk_window_present(GTK_WINDOW(window));
}
