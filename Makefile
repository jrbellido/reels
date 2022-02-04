log_level = debug

all: clean build

build:
	cargo build

release:
	cargo build --release

clean:
	rm -rf ./target
	rm -rf ./sandbox

clean_sandbox:
	rm -rf ./sandbox/repo
	rm -rf ./sandbox/recovered

install: release
	sudo cp ./target/release/reels /usr/bin/reels

unit:
	cargo test -- --nocapture

watch_unit:
	cargo watch -w ./src -s "make unit"

test: clean_sandbox unit build
	sh ./test/full.sh

watch_test:
	cargo watch -w ./src -s "make test"

fmt:
	rustfmt ./src/*.rs --edition 2018
