.PHONY: build
build:
	# cargo rustc --release -- -C link-args=-static && strip target/release/fourkay && ls -la target/release
	cargo build --release && strip target/release/fourkay
	ls -lah target/release/
	du -h target/release/fourkay
	stat --format="%n %b %B" target/release/fourkay
	wc -c < target/release/fourkay
	du -b target/release/fourkay

.PHONY: run
run: build
	cargo run --release

.PHONY: dev
dev:
	RUST_BACKTRACE=1 cargo run
