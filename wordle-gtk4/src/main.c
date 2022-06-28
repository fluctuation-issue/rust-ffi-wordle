#include <locale.h>
#include <glib/gi18n.h>

#include "wordle_app.h"

int main(int argc, char *argv[])
{
    setlocale(LC_ALL, "");
    bindtextdomain("wordle", "build/locales");
    textdomain("wordle");

	return g_application_run(G_APPLICATION(wordle_app_new()), argc, argv);
}
