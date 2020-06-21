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
	cargo build -Z build-std=core --target i686-unknown-linux-gnu --release --no-default-features

.PHONY: run-optimize
run-optimize: optimize
	target/x86_64-unknown-linux-gnu/release/fourkay

.PHONY: build-release
build-release:
	# cargo clean
	cargo build -Z build-std=core --target $(TARGET) --release  # --features=error-handling --features=println
	wc --bytes target/$(TARGET)/release/fourkay

.PHONY: pack
pack: build-release
	rm -rf build && mkdir build

	strip --strip-all -R '.note*' -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
	wc --bytes target/$(TARGET)/release/fourkay

	nasm -fbin -obuild/vondehi ../vondehi/vondehi.asm
	lzma -c target/$(TARGET)/release/fourkay > build/fourkay-lzma
	cat build/vondehi build/fourkay-lzma > build/fourkay

	chmod +x build/fourkay
	wc --bytes build/fourkay

	build/fourkay

#.PHONY: optimize
#optimize: debug-run
#	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay
#	strip --strip-all -R .note* -R .comment target/x86_64-unknown-linux-gnu/release/fourkay
#	wc --bytes target/x86_64-unknown-linux-gnu/release/fourkay