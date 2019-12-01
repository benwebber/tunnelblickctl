PROJECT     = tunnelblickctl

prefix      = /usr/local
exec_prefix = $(prefix)
bindir      = $(exec_prefix)/bin

ifeq ($(shell git describe >/dev/null 2>&1; echo $$?),0)
VERSION  := $(shell git describe --tags --always --dirty --match v* | sed 's/^v//')
CARGO_ENV = VERSION=$(VERSION)
endif

$(PROJECT): src/tunnelblick.js
	$(CARGO_ENV) cargo build --release
	install -m 755 target/release/$(PROJECT) .

clean:
	cargo clean
	find src/ -name '*.bk' -delete
	find src/ -name '*.js' -delete
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
	tslint src/*.ts

src/%.js: src/%.ts
	tsc --lib es2015 --strict --target es5 $<

.PHONY: clean dist fmt install uninstall $(PROJECT)
