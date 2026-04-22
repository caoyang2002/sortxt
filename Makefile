release:
	cargo build --release

clean:
	cargo clean

help:
	cargo run -- --help
test:
	cargo run -- -v -e -u -i test.txt -o sorted.txt
