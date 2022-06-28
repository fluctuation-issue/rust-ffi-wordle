.PHONY: all clean

CURRENT_DIR=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))

WORDLE_CORE_PC=$(CURRENT_DIR)wordle_core.pc
WORDLE_CORE_PC_IN=$(CURRENT_DIR)wordle_core.pc.in

all: $(WORDLE_CORE_PC)

$(WORDLE_CORE_PC): $(WORDLE_CORE_PC_IN)
	sed 's#\$$\$$PWD#$(CURRENT_DIR)#g' $< > $@

clean:
	rm -f $(WORDLE_CORE_PC)
