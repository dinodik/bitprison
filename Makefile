INSTALL_DIR = /usr/local/bin

default : bitprison-helper

install : bitprison-helper
	sudo cp -f target/release/bitprison-helper $(INSTALL_DIR)
	sudo cp -f bitprison.sh $(INSTALL_DIR)/bitprison

bitprison-helper : src/main.rs
	cargo build --release

clean :
	cargo clean
