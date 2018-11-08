.PHONY: buildstrip
build:
	# cargo rustc --release -- -C link-args=-static && strip target/release/fourkay && ls -la target/release
	xargo build --target x86_64-unknown-linux-gnu --release

.PHONY: upx
upx:
	~/upx-3.95-amd64_linux/upx -9 target/release/fourkay

.PHONY: strip
strip:
	strip --strip-all -R .note* -R .comment target/release/fourkay

.PHONY: optimize
optimize: build
	make strip
	make upx
	make filesize

.PHONY: filesize
filesize:
	ls -lah target/release/
	du -h target/release/fourkay
	stat --format="%n %b %B" target/release/fourkay
	wc --bytes target/release/fourkay
	du -b target/release/fourkay

.PHONY: run
run: build
	cargo run --release

.PHONY: dev
dev:
	RUST_BACKTRACE=1 cargo run
