# Simple Makefile to easily compile both the plugin and the main binary

.PHONY: debug release clean

debug:
	cargo build
	cd application && cargo run 
release:
	cargo build --release
	cd application && cargo run --release 

clean:
	rm -rf target 
	rm -rf ./*/target 
