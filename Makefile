build:
	cargo build --release

test:
	cargo test --release

install: build
	sudo cp target/release/web-grep /usr/local/bin/
