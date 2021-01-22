build:
	cargo build --release

test:
	cargo test --release

install: build
	cargo install --path .
