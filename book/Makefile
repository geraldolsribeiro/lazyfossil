.PHONY: all
all: build

.PHONY: build
build:
	mdbook build

.PHONY: upload
upload: build
	tar zcvf book.tgz book
	rclone copy book.tgz r2:lazyfossil/ --s3-no-check-bucket
