WORDLE_CORE_DIR=$(ROOT_DIR)wordle-core

CC=gcc
C_FLAGS=\
	-Wall -Wextra -pedantic \
	$(shell pkg-config --cflags gtk4) \
	$(shell PKG_CONFIG_PATH=$(WORDLE_CORE_DIR) pkg-config --cflags wordle_core)
LIBS=\
	$(shell pkg-config --libs gtk4) \
	$(shell PKG_CONFIG_PATH=$(WORDLE_CORE_DIR) pkg-config --libs wordle_core)
