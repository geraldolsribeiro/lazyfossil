.PHONY: all
all:
	rm -f target/debug/lazyfossil
	rm -f target/release/lazyfossil
	touch src/*.rs
	cargo fmt --all
	cargo fmt --all -- --check
	cargo run --release -- --debug

.PHONY: doc
doc:
	$(MAKE) -C book/

.PHONY: publish
publish:
	find . -name fossil-debug.log -delete
	cargo publish

.PHONY: fix
fix:
	RUSTFLAGS="-D unused-code" \
		/home/geraldo/git/geraldolsribeiro/cargo/target/release/cargo fix

.PHONY: media
media:
	cargo build --release
	$(MAKE) -C vhs
