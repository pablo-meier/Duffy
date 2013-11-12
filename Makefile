# Yay Rust!


BUILD_DIR=build


build: prepare
	rustc --out-dir $(BUILD_DIR) src/main.rs

run: build
	$(BUILD_DIR)/main la-overworld.mid

clean:
	 rm -rf $(BUILD_DIR)

prepare: 
	test -d $(BUILD_DIR) || mkdir $(BUILD_DIR)
