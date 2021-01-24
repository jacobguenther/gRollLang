PREFIX=/usr

INSTALL=install
INSTALL_PROGRAM=$(INSTALL)
INSTALL_DATA=$(INSTALL) -m 644

bindir=$(DESTDIR)$(PREFIX)/bin
sharedir=$(DESTDIR)$(PREFIX)/share

.PHONY: install uninstall clean clean-all

target/release/gRollLang : src
	cargo build --release --offline --verbose

install :
	mkdir -p $(bindir)
	$(INSTALL_PROGRAM) target/release/g_roll_lang $(bindir)/gRollLang
	
	mkdir -p $(sharedir)/applications
	$(INSTALL_DATA) data/quaternion.site.gRollLang.desktop $(sharedir)/applications/quaternion.site.gRollLang.desktop

uninstall :
	rm -f /usr/bin/gRollLang
	rm -f /usr/share/applications/quaternion.site.gRollLang.desktop


flatpak-development : target/release/gRollLang
	mkdir -p flatpak-development
	flatpak-builder flatpak-development data/quaternion.site.gRollLang-dev.json

flatpak-development-install : target/release/gRollLang
	mkdir -p flatpak-development
	flatpak-builder --user --install flatpak-development data/quaternion.site.gRollLang-dev.json


flatpak-release : target/release/gRollLang
	mkdir -p flatpak
	flatpak-builder flatpak data/quaternion.site.gRollLang.json

flatpak-release-install : target/release/gRollLang
	mkdir -p flatpak
	flatpak-builder --user --install flatpak data/quaternion.site.gRollLang-dev.json


clean-all : clean
	cargo clean

clean :
	rm -rf flatpak/ flatpak-dev/ .flatpak-builder
	rm -rf snap/ *.snap