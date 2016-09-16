PROJECT     = tunnelblickctl
VERSION     = 0.1.0

prefix      = /usr/local
exec_prefix = $(prefix)
bindir      = $(exec_prefix)/bin

$(PROJECT):
	cargo build --release
	install -m 755 -T target/release/$(PROJECT) $(PROJECT)

clean:
	cargo clean
	find src/ -name '*.bk' -delete
	$(RM) -r dist
	$(RM) $(PROJECT)

dist: $(PROJECT)
	mkdir -p dist
	tar -czvf dist/$(PROJECT)-$(VERSION)-x86_64-apple-darwin.tar.gz $(PROJECT)

install: $(PROJECT)
	install -m 755 target/release/$(PROJECT) $(DESTDIR)$(bindir)

uninstall:
	$(RM) $(DESTDIR)$(bindir)/$(PROJECT)

fmt:
	cargo fmt

lint:
	cargo fmt -- --write-mode=diff

.PHONY: clean dist fmt install uninstall $(PROJECT)
