PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
DATADIR ?= $(PREFIX)/share
APPDIR ?= $(DATADIR)/applications
ICONDIR ?= $(DATADIR)/icons/hicolor/scalable/apps

TARGET = target/release/cardwire-tray-icon

.PHONY: all build run clean install uninstall

all: build

build:
	cargo build --release

run:
	cargo run

clean:
	cargo clean

install:
	install -d $(BINDIR)
	install -m 755 $(TARGET) $(BINDIR)/cardwire-tray
	install -d $(APPDIR)
	install -m 644 cardwire-tray.desktop $(APPDIR)/cardwire-tray.desktop
	install -d $(ICONDIR)
	install -m 644 icons/gpu.svg $(ICONDIR)/cardwire-tray.svg
	install -m 644 icons/gpu.svg $(ICONDIR)/cardwire-gpu.svg
	install -m 644 icons/integrated.svg $(ICONDIR)/cardwire-integrated.svg
	install -m 644 icons/hybrid.svg $(ICONDIR)/cardwire-hybrid.svg
	install -m 644 icons/manual.svg $(ICONDIR)/cardwire-manual.svg

uninstall:
	rm -f $(BINDIR)/cardwire-tray
	rm -f $(APPDIR)/cardwire-tray.desktop
	rm -f $(ICONDIR)/cardwire-tray.svg
	rm -f $(ICONDIR)/cardwire-gpu.svg
	rm -f $(ICONDIR)/cardwire-integrated.svg
	rm -f $(ICONDIR)/cardwire-hybrid.svg
	rm -f $(ICONDIR)/cardwire-manual.svg
