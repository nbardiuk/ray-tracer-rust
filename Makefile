default: render

render: run
	xdg-open ./canvas.ppm

run: build
	cargo run --release

build: clean
	cargo build --release

test: clean
	cargo test

clean:
	cargo clean
	rm -f *.ppm
