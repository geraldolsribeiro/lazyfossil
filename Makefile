.PHONY: all
all:
	cargo run --release -- --debug

.PHONY: check
check:
	/home/geraldo/git/geraldolsribeiro/cargo/target/release/cargo \
		clippy --fix --allow-dirty --allow-staged
	cargo fmt --all
	cargo fmt --all -- --check

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
