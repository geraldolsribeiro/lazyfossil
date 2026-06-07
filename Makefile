all:
	rm -f target/debug/lazyfossil
	touch src/*.rs
	cargo run

publish:
	rm -f fossil-debug.log
	cargo publish
