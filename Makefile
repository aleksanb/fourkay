TARGET=x86_64-unknown-linux-gnu

.PHONY: shaders
shaders:
	ruby ../glsl-minifier/glsl_min.rb src/shaders/balls.glsl > src/shaders/balls.glsl.out
	ruby ../glsl-minifier/glsl_min.rb src/shaders/flower.glsl > src/shaders/flower.glsl.out
	ruby ../glsl-minifier/glsl_min.rb src/shaders/snake.glsl > src/shaders/snake.glsl.out
	ruby ../glsl-minifier/glsl_min.rb src/shaders/blobby.glsl > src/shaders/blobby.glsl.out
	ruby ../glsl-minifier/glsl_min.rb src/shaders/quad-vertex.glsl > src/shaders/quad-vertex.glsl.out

.PHONY: optimize-build
optimize-build:
	xargo build --target i686-unknown-linux-gnu --release --no-default-features

.PHONY: optimize
optimize: optimize-build
	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay
	# strip --strip-all -R .note* -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
	//wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: run-optimize
run-optimize: optimize
	target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: vondehi
vondehi:
	cargo build -Z build-std=core --target $(TARGET) --release --no-default-features

	 wc --bytes target/$(TARGET)/release/fourkay
	strip -R '.note*' -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
	wc --bytes target/$(TARGET)/release/fourkay
	cp ../vondehi/vondehi build/vondehi
	nasm -fbin -obuild/vondehi ../vondehi/vondehi.asm
	lzma -c target/$(TARGET)/release/fourkay > build/fourkay-lzma
	cat build/vondehi build/fourkay-lzma > build/fourkay
	chmod +x build/fourkay
	wc --bytes build/fourkay
	build/fourkay

.PHONY: debug
debug:
	cargo build -Z build-std=core --target $(TARGET) --no-default-features

.PHONY: debug-run
debug-run:
	cargo build -Z build-std=core --target $(TARGET) --release --no-default-features