default: clean test render

.PHONY: render
render:
	cargo run --release
	xdg-open ./canvas.ppm

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean
	rm -f *.ppm
