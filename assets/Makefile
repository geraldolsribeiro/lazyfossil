.PHONY: upload
upload:
	rclone copy ./ r2:lazyfossil/ --include "*.png" --transfers 16 --progress
	rclone copy ./ r2:lazyfossil/ --include "*.svg" --transfers 16 --progress
