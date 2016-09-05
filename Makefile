PROJECT     = tunnelblickctl

prefix      = /usr/local
exec_prefix = $(prefix)
bindir      = $(exec_prefix)/bin

$(PROJECT):
	cargo build --release
	install -m 755 -T target/release/$(PROJECT) $(PROJECT)

clean:
	cargo clean
	$(RM) $(PROJECT)

install: $(PROJECT)
	install -m 755 target/release/$(PROJECT) $(DESTDIR)$(bindir)

uninstall:
	$(RM) $(DESTDIR)$(bindir)/$(PROJECT)

fmt:
	find src -name '*.rs' -exec rustfmt {} \;

.PHONY: clean fmt install uninstall $(PROJECT)
