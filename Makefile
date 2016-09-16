PROJECT     = tunnelblickctl

prefix      = /usr/local
exec_prefix = $(prefix)
bindir      = $(exec_prefix)/bin

ifeq ($(shell git describe >/dev/null 2>&1; echo $$?),0)
VERSION  := $(shell git describe --tags --always --dirty --match v* | sed 's/^v//')
CARGO_ENV = VERSION=$(VERSION)
endif

$(PROJECT):
	$(CARGO_ENV) cargo build --release
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
