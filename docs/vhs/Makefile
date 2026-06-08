%.gif: %.tape
	docker run --rm \
		-v $(shell pwd):/vhs \
		-v /usr/bin/fossil:/usr/bin/fossil \
		-v ../../target/release/lazyfossil:/bin/lazyfossil \
		ghcr.io/charmbracelet/vhs $<

all: \
	open_01.gif \
	open_02.gif \

		# -u "$(shell id -u):$(shell id -g)" \
