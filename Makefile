format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run:
	cargo run

build: format lint
	cargo build 

clean:
	cargo clean	--quiet

clean-build: clean build
