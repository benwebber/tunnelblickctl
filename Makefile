.PHONY: clean

PROJECT  = tunnelblickctl
SOURCES := $(wildcard *.applescript)
OBJECTS := $(SOURCES:.applescript=.scpt)

$(PROJECT): $(OBJECTS)
	sed -i '1s|^|#!/usr/bin/env osascript\n|' $<
	install -m 0755 -T $< $@

clean:
	$(RM) $(OBJECTS)

%.scpt: %.applescript
	osacompile -o $(@) -x $<
