#include <stdlib.h>
#include <string.h>

#include <glib/gi18n.h>

#include "wordle_hints.h"

struct _WordleHints {
    GtkBox parent;
    
    GtkWidget *word_length;
    GtkWidget *grid;
};

WordleHints* wordle_hints_new(void)
{
    return g_object_new(WORDLE_HINTS_TYPE, NULL);
}

static void wordle_hints_init(WordleHints *wordle_hints);
static void wordle_hints_class_init(WordleHintsClass *class);
static char* generate_label_markup(char const* color, char letter);
static int grid_get_rows_count(GtkGrid *grid);
static char* int_to_string(int const number);
static char* get_letter_hint_color(wc_letter_hint letter_hint);

void wordle_hints_clear(WordleHints *wordle_hints)
{
    GtkWidget *next = NULL;
    GtkWidget *iter = gtk_widget_get_first_child(GTK_WIDGET(wordle_hints->grid));
    while (iter != NULL) {
        next = gtk_widget_get_next_sibling(iter);
        gtk_grid_remove(GTK_GRID(wordle_hints->grid), iter);
        iter = next;
    }
}

void wordle_hints_reset(WordleHints *wordle_hints, int guess_word_length)
{
    wordle_hints_clear(wordle_hints);

    gtk_widget_set_visible(GTK_WIDGET(wordle_hints->word_length), TRUE);
    gtk_widget_set_visible(GTK_WIDGET(wordle_hints->grid), FALSE);

    char *word_length_string = int_to_string(guess_word_length);
    char *word_length_label = malloc(sizeof(*word_length_label) * (strlen(_("Word length: %s")) + strlen(word_length_string) + 1));
    if (word_length_label == NULL)
        return;
    sprintf(word_length_label, _("Word length: %s"), word_length_string);

    gtk_label_set_text(GTK_LABEL(wordle_hints->word_length), word_length_label);

    free(word_length_string);
    free(word_length_label);
}

void wordle_hints_add_guess_row(WordleHints *wordle_hints, const wc_guess_hint_t guess_hint)
{
    wc_guessed_letters_and_hints* letters_and_hints = wc_guess_hint_get_guessed_letters_and_hints(guess_hint);

    gtk_widget_set_visible(GTK_WIDGET(wordle_hints->word_length), FALSE);
    gtk_widget_set_visible(GTK_WIDGET(wordle_hints->grid), TRUE);

    int rows_count = grid_get_rows_count(GTK_GRID(wordle_hints->grid));

    for (uint32_t x = 0; x < letters_and_hints->num_letters_and_hints; ++x) {
        char *letter_hint_color = get_letter_hint_color(letters_and_hints->hints[x].hint);
        char *letter_string = generate_label_markup(letter_hint_color, letters_and_hints->hints[x].letter);

        GtkWidget *label = gtk_label_new(letter_string);
        gtk_label_set_text(GTK_LABEL(label), letter_string);
        gtk_label_set_use_markup(GTK_LABEL(label), TRUE);
        gtk_grid_attach(GTK_GRID(wordle_hints->grid), label, x, rows_count, 1, 1);

        free(letter_string);
    }

    wc_guess_hint_free_guessed_letters_and_hints(letters_and_hints);
}

G_DEFINE_TYPE(WordleHints, wordle_hints, GTK_TYPE_BOX)

static void wordle_hints_init(WordleHints *wordle_hints)
{
    gtk_widget_init_template(GTK_WIDGET(wordle_hints));
}

static void wordle_hints_class_init(WordleHintsClass *class)
{
    gtk_widget_class_set_template_from_resource(GTK_WIDGET_CLASS(class), "/local/imgt/wordle_app/wordle_hints.ui");

    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleHints, word_length);
    gtk_widget_class_bind_template_child(GTK_WIDGET_CLASS(class), WordleHints, grid);
}

static char* generate_label_markup(char const* color, char letter)
{
    char const* prefix = "<span foreground='white' background='";
    char const* middle = "'> ";
    char const* suffix = " </span>";
    size_t prefix_length = strlen(prefix);
    size_t middle_length = strlen(middle);
    size_t suffix_length = strlen(suffix);
    size_t total_length = prefix_length + strlen(color) + middle_length + 1 + suffix_length;

    char *result = malloc(sizeof(*result) * (total_length + 1));
    if (result == NULL)
        return NULL;
    sprintf(result, "%s%s%s%c%s", prefix, color, middle, letter, suffix);
    return result;
}

static int grid_get_rows_count(GtkGrid *grid)
{
    GtkWidget *iter = NULL;
    int max_row = 0;
    int current_row = 0;

    iter = gtk_widget_get_first_child(GTK_WIDGET(grid));
    while (iter != NULL)
    {
        gtk_grid_query_child(grid, iter, NULL, &current_row, NULL, NULL);
        max_row = MAX(max_row, current_row);
        iter = gtk_widget_get_next_sibling(iter);
    }

    return max_row + 1;
}

static char* int_to_string(int const number)
{
    int length = snprintf(NULL, 0, "%d", number);
    char *result = malloc(sizeof(*result) * (length + 1));
    snprintf(result, length + 1, "%d", number);
    return result;
}

static char* get_letter_hint_color(wc_letter_hint letter_hint)
{
    switch (letter_hint)
    {
        case WC_LETTER_HINT_CORRECT:
            return "green";
        case WC_LETTER_HINT_PLACEMENT_INCORRECT:
            return "yellow";
        case WC_LETTER_HINT_INCORRECT:
            return "gray";
        default:
            return "";
    }
}
