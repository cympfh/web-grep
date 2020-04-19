install: build
	sudo cp target/release/web-grep /usr/local/bin/

build:
	cargo build --release
