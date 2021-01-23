.PHONY: clean clean-all install uninstall

target/release/gDiceRoller : src
	cargo build --release

install : target/release/gDiceRoller
	cp target/release/gRollLang /usr/bin/gRollLang
	cp data/gRollLang.desktop /usr/share/applications/gRollLang.desktop

uninstall :
	rm -f /usr/bin/gRollLang
	rm -f /usr/share/applications/gRollLang.desktop

clean-all : clean
	cargo clean

clean :
	rue