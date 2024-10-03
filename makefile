default: help

build:
	cargo build --bin main --bin extend
run:
	cargo run --bin main --bin extend

build-rel:
	cargo build --release --bin main --bin extend
run-rel:
	cargo run --release --bin main --bin extend

clean:
	cargo clean

rebuild: clean build build-rel

test:
	cargo test

help:
	@echo "Available commands:"
	@echo "  build 		- Build the project"
	@echo "  run 		- Build and run the project"
	@echo "  build-rel 	- Build the project in release mode"
	@echo "  run-rel 	- Build and run the project in release mode"
	@echo "  clean 		- Clean the project"
	@echo "  rebuild 	- Clean and rebuild the project"
	@echo "  test 		- Run the tests"

.PHONY: build run clean rebuild test help