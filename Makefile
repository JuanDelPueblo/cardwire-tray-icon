PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
DATADIR ?= $(PREFIX)/share
APPDIR ?= $(DATADIR)/applications
ICONDIR ?= $(DATADIR)/icons/hicolor/scalable/apps

TARGET = target/release/cardwire-tray-icon

.PHONY: all build run clean install uninstall rpm

all: build

build:
	cargo build --release

run:
	cargo run

clean:
	cargo clean
	rm -rf rpmbuild

rpm:
	mkdir -p rpmbuild/SOURCES
	git archive --format=tar.gz --prefix=cardwire-tray-icon/ HEAD > rpmbuild/SOURCES/cardwire-tray-icon-0.1.0.tar.gz
	rpmbuild -ba cardwire-tray-icon.spec --define "_topdir $(PWD)/rpmbuild"

install:
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 $(TARGET) $(DESTDIR)$(BINDIR)/cardwire-tray
	install -d $(DESTDIR)$(APPDIR)
	install -m 644 cardwire-tray.desktop $(DESTDIR)$(APPDIR)/cardwire-tray.desktop
	install -d $(DESTDIR)$(ICONDIR)
	install -m 644 icons/gpu.svg $(DESTDIR)$(ICONDIR)/cardwire-tray.svg
	install -m 644 icons/gpu.svg $(DESTDIR)$(ICONDIR)/cardwire-gpu.svg
	install -m 644 icons/integrated.svg $(DESTDIR)$(ICONDIR)/cardwire-integrated.svg
	install -m 644 icons/hybrid.svg $(DESTDIR)$(ICONDIR)/cardwire-hybrid.svg
	install -m 644 icons/manual.svg $(DESTDIR)$(ICONDIR)/cardwire-manual.svg

uninstall:
	rm -f $(BINDIR)/cardwire-tray
	rm -f $(APPDIR)/cardwire-tray.desktop
	rm -f $(ICONDIR)/cardwire-tray.svg
	rm -f $(ICONDIR)/cardwire-gpu.svg
	rm -f $(ICONDIR)/cardwire-integrated.svg
	rm -f $(ICONDIR)/cardwire-hybrid.svg
	rm -f $(ICONDIR)/cardwire-manual.svg
