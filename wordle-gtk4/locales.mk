LOCALES_BUILD_DIR=$(BUILD_DIR)locales/
MERGED_PO=$(patsubst $(CURRENT_DIR)locales/%,$(LOCALES_BUILD_DIR)%/LC_MESSAGES/wordle.po,$(shell find "$(CURRENT_DIR)locales" -maxdepth 1 -type d))
PO=$(shell find "$(CURRENT_DIR)locales" -name '*.po')
MO=$(patsubst %.po,%.mo,$(MERGED_PO))

locales: $(LOCALES_BUILD_DIR) $(MERGED_PO) $(MO)
	@echo $(MERGED_PO)

$(LOCALES_BUILD_DIR)%/LC_MESSAGES/wordle.po: $(PO)
	@mkdir -p $(@D)
	@language=$$(basename $$(dirname $$(dirname $@))); \
		echo "MERGING po for lang: $$language"; \
		msgcat -o $@ $$(find "$(CURRENT_DIR)locales/$$language" -name '*.po')

$(LOCALES_BUILD_DIR):
	[ -d "$@" ] || mkdir "$@"

%.mo: %.po
	msgfmt -o $@ -c $<
