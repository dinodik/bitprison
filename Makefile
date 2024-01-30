INSTALL_DIR = /usr/local/bin

default : bw-tool-helper

install : bw-tool-helper
	sudo cp -f target/release/bw-tool-helper $(INSTALL_DIR)
	sudo cp -f bw-tool.sh $(INSTALL_DIR)/bw-tool

bw-tool-helper : src/main.rs
	cargo build --release

clean :
	cargo clean
