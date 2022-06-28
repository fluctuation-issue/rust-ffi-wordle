#include <string.h>
#include <ctype.h>
#include <glib/gi18n.h>

#include "wordle_core/picker.h"

#include "wordle_app_window.h"
#include "wordle_hints.h"

struct _WordleAppWindow {
    GtkApplicationWindow parent;

    GtkWidget *content;
    GtkWidget *guess_word;
    GtkWidget *quit;
    GtkWidget *submit;
    GtkWidget *reset;
    GtkWidget *end_state;

    GtkWidget *hints;

    wc_word_picker_t word_picker;
    wc_game_t game;
};

G_DEFINE_TYPE(WordleAppWindow, wordle_app_window, GTK_TYPE_APPLICATION_WINDOW)

static void wordle_app_window_init(WordleAppWindow *window);
static void wordle_app_window_class_init(WordleAppWindowClass *class);
static void wordle_app_window_dispose(GObject *object);
static void reset_game(WordleAppWindow *window);
static void on_guess_entry_activate(GtkEntry *guess_entry, gpointer window_pointer);
static void on_submit_clicked(GtkButton *submit_button, gpointer window_pointer);
static void try_to_guess_word(WordleAppWindow *window);
static char* get_current_guessed_word(WordleAppWindow *window);
static void clear_guess_entry(WordleAppWindow *window);
static int can_guess_word(WordleAppWindow* window, char const *guess_word);
static int word_has_correct_length(WordleAppWindow* window, char const *guess_word);
static int word_has_been_guessed_before(WordleAppWindow *window, char const *guess_word);
static void make_guess(WordleAppWindow* window, char const *guessed);
static void freeze_guess_entry(WordleAppWindow* window);
static void unfreeze_guess_entry(WordleAppWindow* window);
static void show_reset_button_hide_submit_button(WordleAppWindow *window);
static void show_submit_button_hide_reset_button(WordleAppWindow *window);
static void on_game_lost(WordleAppWindow* window);
static void on_game_won(WordleAppWindow* window);
static void on_reset_clicked(GtkButton *reset_button, gpointer window_pointer);
static char* text_with_foreground_color(char const *text, char const *color);

WordleAppWindow *wordle_app_window_new(WordleApp *app)
{
    return g_object_new(WORDLE_APP_WINDOW_TYPE, "application", app, NULL);
}

void wordle_app_window_open(WordleAppWindow* window, GFile *file)
{
    if (file == NULL)
        return;
    char* file_path = g_file_get_path(file);
    wc_word_picker_t new_picker = wc_word_picker_new_random_line_file(file_path);
    g_free(file_path);

    if (new_picker != NULL)
    {
        wc_word_picker_free(window->word_picker);
        window->word_picker = new_picker;

        reset_game(window);
    }
}

static void wordle_app_window_init(WordleAppWindow *window)
{
    char *words[] = { "wordle", "wordlerust", NULL };
    window->word_picker = wc_word_picker_new_from_list(words);
    window->game = NULL;

    gtk_widget_init_template(GTK_WIDGET(window));

    window->hints = GTK_WIDGET(wordle_hints_new());
    gtk_box_prepend(GTK_BOX(window->content), window->hints);

    reset_game(window);

    g_signal_connect_swapped(window->quit, "clicked", G_CALLBACK(gtk_window_destroy), window);
    g_signal_connect(window->guess_word, "activate", G_CALLBACK(on_guess_entry_activate), window);
    g_signal_connect(window->submit, "clicked", G_CALLBACK(on_submit_clicked), window);
    g_signal_connect(window->reset, "clicked", G_CALLBACK(on_reset_clicked), window);
}

static void wordle_app_window_class_init(WordleAppWindowClass *class)
{
    gtk_widget_class_set_template_from_resource(GTK_WIDGET_CLASS(class), "/local/imgt/wordle_app/wordle_app_window.ui");
    G_OBJECT_CLASS(class)->dispose = wordle_app_window_dispose;

    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleAppWindow, content);
    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleAppWindow, guess_word);
    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleAppWindow, quit);
    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleAppWindow, submit);
    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleAppWindow, reset);
    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleAppWindow, end_state);
}

static void wordle_app_window_dispose(GObject *object)
{
    WordleAppWindow* window = WORDLE_APP_WINDOW(object);

    g_clear_pointer(&window->word_picker, wc_word_picker_free);
    g_clear_pointer(&window->game, wc_game_free);

    G_OBJECT_CLASS(wordle_app_window_parent_class)->dispose(object);
}

static void reset_game(WordleAppWindow *window)
{
    if (window->game != NULL)
    {
        wc_game_free(window->game);
        window->game = NULL;
    }

    char *new_word = wc_word_picker_pick_word(window->word_picker);
    window->game = wc_game_new(new_word);
    wordle_hints_reset(WORDLE_HINTS(window->hints), strlen(new_word));
    rust_str_free(new_word);

}

static void on_guess_entry_activate(GtkEntry *guess_entry, gpointer window_pointer)
{
    (void) guess_entry;

    WordleAppWindow *window = (WordleAppWindow*) window_pointer;
    try_to_guess_word(window);
}

static void on_submit_clicked(GtkButton *submit_button, gpointer window_pointer)
{
    (void) submit_button;

    WordleAppWindow *window = (WordleAppWindow*) window_pointer;
    try_to_guess_word(window);
}

static void try_to_guess_word(WordleAppWindow *window)
{
    char *guessed = get_current_guessed_word(window);
    if (!can_guess_word(window, guessed))
    {
        free(guessed);
        return;
    }

    clear_guess_entry(window);
    make_guess(window, guessed);

    free(guessed);
}

static char* get_current_guessed_word(WordleAppWindow *window)
{
    GtkEntryBuffer *buffer = gtk_entry_get_buffer(GTK_ENTRY(window->guess_word));
    char* guessed = strdup(gtk_entry_buffer_get_text(buffer));
    char* guessed_iter = guessed;
    while (*guessed_iter != '\0')
    {
        *guessed_iter = toupper(*guessed_iter);
        guessed_iter++;
    }
    return guessed;
}

static void clear_guess_entry(WordleAppWindow *window)
{
    GtkEntryBuffer *buffer = gtk_entry_get_buffer(GTK_ENTRY(window->guess_word));
    gtk_entry_buffer_set_text(buffer, "", 0);
    gtk_entry_set_buffer(GTK_ENTRY(window->guess_word), buffer);
}

static int can_guess_word(WordleAppWindow* window, char const *guess_word)
{
    return word_has_correct_length(window, guess_word) && !word_has_been_guessed_before(window, guess_word);
}

static int word_has_correct_length(WordleAppWindow *window, char const *guess_word)
{
    char* word_to_guess = wc_game_get_word_to_guess(window->game);
    size_t to_guess_length = strlen(word_to_guess);
    rust_str_free(word_to_guess);
    return strlen(guess_word) == to_guess_length;
}

static int word_has_been_guessed_before(WordleAppWindow *window, char const *guess_word)
{
    wc_guess_hint_list_node_t* first_node;
    wc_guess_hint_list_node_t* node;
    wc_guess_hint_t guess_hint;

    first_node = wc_game_get_guess_hints(window->game);
    node = first_node;
    while (node != NULL)
    {
        guess_hint = node->current;

        char *guessed = wc_guess_hint_get_guessed(guess_hint);

        node = node->next;

        if (strcmp(guess_word, guessed) == 0)
        {
            rust_str_free(guessed);
            return 1;
        }
        rust_str_free(guessed);
    }
    wc_game_guess_hints_free(first_node);

    return 0;
}

static void make_guess(WordleAppWindow* window, char const *guessed)
{
    wc_guess_hint_t guess_hint;
    wc_game_state game_state;

    wc_game_guess(window->game, guessed, NULL, &game_state);

    guess_hint = wc_game_get_current_guess_hint(window->game);
    wordle_hints_add_guess_row(WORDLE_HINTS(window->hints), guess_hint);
    wc_guess_hint_free(guess_hint);

    switch (game_state)
    {
    case WC_GAME_STATE_PENDING:
        break;
    case WC_GAME_STATE_LOST:
        on_game_lost(window);
        break;
    case WC_GAME_STATE_WON:
        on_game_won(window);
        break;
    default:
        g_printerr("was not expecting this game state\n");
    }
}

static void freeze_guess_entry(WordleAppWindow* window)
{
    gtk_widget_set_sensitive(window->guess_word, FALSE);
}

static void unfreeze_guess_entry(WordleAppWindow* window)
{
    gtk_widget_set_sensitive(window->guess_word, TRUE);
}

static void show_reset_button_hide_submit_button(WordleAppWindow *window)
{
    gtk_widget_set_visible(window->reset, TRUE);
    gtk_widget_set_visible(window->submit, FALSE);
}

static void show_submit_button_hide_reset_button(WordleAppWindow *window)
{
    gtk_widget_set_visible(window->submit, TRUE);
    gtk_widget_set_visible(window->reset, FALSE);
}

static void on_game_lost(WordleAppWindow* window)
{
    freeze_guess_entry(window);
    show_reset_button_hide_submit_button(window);

    char *word_to_guess = wc_game_get_word_to_guess(window->game);
    char const *loose_message_format = _("You lost! The word was %s.");
    size_t length = strlen(loose_message_format) + strlen(word_to_guess);
    char *loose_message = malloc(sizeof(*loose_message) * length); 
    sprintf(loose_message, loose_message_format, word_to_guess);
    loose_message[length] = '\0';
    rust_str_free(word_to_guess);
    char *loose_text = text_with_foreground_color(loose_message, "red");
    free(loose_message);

    gtk_label_set_markup(GTK_LABEL(window->end_state), loose_text);
    free(loose_text);
    gtk_widget_set_visible(window->end_state, TRUE);
}

static void on_game_won(WordleAppWindow* window)
{
    freeze_guess_entry(window);
    show_reset_button_hide_submit_button(window);

    char *win_text = text_with_foreground_color(_("You won!"), "green");
    gtk_label_set_markup(GTK_LABEL(window->end_state), win_text);
    free(win_text);
    gtk_widget_set_visible(window->end_state, TRUE);
}

static void on_reset_clicked(GtkButton *reset_button, gpointer window_pointer)
{
    (void) reset_button;

    WordleAppWindow *window = (WordleAppWindow*) window_pointer;

    reset_game(window);

    unfreeze_guess_entry(window);
    show_submit_button_hide_reset_button(window);

    gtk_widget_set_visible(window->end_state, FALSE);
}

static char* text_with_foreground_color(char const *text, char const *color)
{
    char prefix[] = "<span color='";
    char middle[] = "'>";
    char suffix[] = "</span>";

    char length = strlen(prefix) + strlen(color) + strlen(middle) + strlen(text) + strlen(suffix);

    char *result = malloc(sizeof(*result) * (length + 1));
    if (result == NULL)
        return NULL;
    sprintf(result, "%s%s%s%s%s", prefix, color, middle, text, suffix);
    return result;
}
