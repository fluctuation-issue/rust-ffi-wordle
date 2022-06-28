.PHONY: all locales run run-fr run-de clean distclean

CURRENT_DIR=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))
SRC_DIR=$(CURRENT_DIR)src
BUILD_DIR=$(CURRENT_DIR)build/
ROOT_DIR=$(shell dirname "$(CURRENT_DIR)")/

SOURCES=$(shell find "$(SRC_DIR)" -name '*.c')
HEADERS=$(patsubst %.c,%.h,$(SOURCES))
OBJECTS=$(patsubst $(SRC_DIR)%.c,$(BUILD_DIR)%.o,$(SOURCES))

TARGET=$(BUILD_DIR)wordle-gtk4

RESOURCES_C=$(CURRENT_DIR)src/resources.c

UI_FILES=$(shell find "$(CURRENT_DIR)" -name '*.ui')

all: $(BUILD_DIR) locales $(TARGET)

include $(CURRENT_DIR)config.mk
include $(CURRENT_DIR)locales.mk

run: $(TARGET)
	$<

run-fr: $(TARGET)
	LANGUAGE=fr $<

run-de: $(TARGET)
	LANGUAGE=de $<

$(BUILD_DIR):
	[ -d "$@" ] || mkdir "$@"

$(BUILD_DIR)%.o: $(SRC_DIR)%.c
	@[ -d $(@D) ] || { mkdir -p $(@D) && echo "MKDIR $(@D)"; }
	$(CC) -o $@ -c $< $(C_FLAGS) $(LIBS)

$(TARGET): $(RESOURCES_C) $(OBJECTS)
	$(CC) -o $@ $(OBJECTS) $(C_FLAGS) $(LIBS)

$(RESOURCES_C): $(CURRENT_DIR)wordle_app.gresource.xml $(UI_FILES)
	glib-compile-resources $(CURRENT_DIR)wordle_app.gresource.xml --target=$@ --generate-source

clean:
	rm -f $(OBJECTS)
	rm -f "$(TARGET)"
	rm -f "$(RESOURCES_C)"
	[ ! -d "$(LOCALES_BUILD_DIR)" ] || rm -r "$(LOCALES_BUILD_DIR)"

distclean: clean
	[ ! -d "$(BUILD_DIR)" ] || rm -r "$(BUILD_DIR)"
