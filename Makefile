INSTALL_DIR = /usr/local/bin

default : bitprison-helper

install : bitprison-helper
	 rm -f $(INSTALL_DIR)/bitprison-helper $(INSTALL_DIR)/bitprison
	 cp target/release/bitprison-helper $(INSTALL_DIR)
	 cp bitprison.sh $(INSTALL_DIR)/bitprison

bitprison-helper : src/main.rs
	cargo build --release

clean :
	cargo clean
