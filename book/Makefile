.PHONY: all
all: upload

.PHONY: build
build:
	mdbook build

.PHONY: serve
serve:
	mdbook serve

.PHONY: upload
upload: build
	tar zcvf book.tgz book
	rclone copy book.tgz r2:lazyfossil/ --s3-no-check-bucket
