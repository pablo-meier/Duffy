# Yay Rust!

BIN_DIR=bin
BUILD_DIR=build


build: prepare
	rustpkg build midi
	rustpkg build duffy

test: build
	rustc --test -o bin/test-midi src/midi/lib.rs
	./bin/test-midi

clean:
	 rm -rf $(BUILD_DIR)
	 rm -rf $(BIN_DIR)

prepare: 
	test -d $(BUILD_DIR) || mkdir $(BUILD_DIR)
	test -d $(BIN_DIR) || mkdir $(BIN_DIR)
