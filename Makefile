default: clean test render

.PHONY: render
render:
	cargo watch -x 'run --release'

.PHONY: test
test:
	cargo test

.PHONY: tdd
tdd:
	cargo watch -x 'test -q'

.PHONY: clean
clean:
	cargo clean
	rm -f *.ppm
