default: clean test render

render:
	cargo run --release
	xdg-open ./canvas.ppm

test:
	cargo test

clean:
	cargo clean
	rm -f *.ppm
