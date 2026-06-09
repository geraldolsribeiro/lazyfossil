TAPES=$(wildcard *.tape)
GIFS=$(TAPES:.tape=.gif)

# %.gif: %.tape ../../target/release/lazyfossil
# 	docker run --rm \
# 		-v $(shell pwd):/vhs \
# 		-v /usr/bin/fossil:/usr/bin/fossil \
# 		-v ../../target/release/lazyfossil:/bin/lazyfossil \
# 		ghcr.io/charmbracelet/vhs $<
# 	sudo chown $(shell id -u):$(shell id -g) *.gif
# 	mv output.gif $@

%.gif: %.tape
	vhs $< -o tmp.gif
	gifsicle -O3 --lossy=100 tmp.gif -o $@

.PHONY: all
all: $(GIFS)

