all:
	rm -f target/debug/lazyfossil
	rm -f target/release/lazyfossil
	touch src/*.rs
	cargo run --release -- --debug

.PHONY: doc
doc:
	cargo doc --no-deps --open

publish:
	rm -f fossil-debug.log
	cargo publish

# docker run --rm -v $PWD:/vhs ghcr.io/charmbracelet/vhs <cassette>.tape

media-check:
	@command -v vhs >/dev/null 2>&1 || (echo "vhs is not installed" && exit 1)

media-demo: media-check
	vhs docs/vhs/lazyfossil-demo.cast

media-screenshot: media-check
	vhs docs/vhs/lazyfossil-demo.cast

media-export:
	./scripts/export-media.sh $(VERSION)

fix:
	RUSTFLAGS="-D unused-code" \
		/home/geraldo/git/geraldolsribeiro/cargo/target/release/cargo fix
