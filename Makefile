all:
	rm -f target/debug/lazyfossil
	touch src/*.rs
	cargo run --release

publish:
	rm -f fossil-debug.log
	cargo publish

fix:
	RUSTFLAGS="-D unused-code" \
		/home/geraldo/git/geraldolsribeiro/cargo/target/release/cargo fix
