Q=@
TAPES=$(filter-out terminal_config.tape,$(wildcard *.tape))
GIFS=$(TAPES:.tape=.gif)

%.gif: %.tape terminal_config.tape ../target/release/lazyfossil
	$(Q)echo "Rendering $@"
	$(Q)PATH=../target/release/:$(PATH) vhs $< -o tmp.gif
	$(Q)gifsicle -O3 --lossy=80 tmp.gif -o $@
	$(Q)rm tmp.gif

# %.gif: %.tape terminal_config.tape ../target/release/lazyfossil
# 	$(Q)echo "Rendering $@"
# 	$(Q)PATH=../target/release/:$(PATH) vhs $< -o $@

.PHONY: all
all: $(GIFS)

.PHONY: upload
upload:
	rclone lsf r2:lazyfossil
	rclone copy ./ r2:lazyfossil/ --include "*.gif" --transfers 16 --progress
	rclone copy ./ r2:lazyfossil/ --include "*.png" --transfers 16 --progress
	rclone copy ../doc/images/lazyfossil_logo.png r2:lazyfossil --s3-no-check-bucket

.PHONY: record
record:
	$(Q)echo "Type exit when you finish the demo"
	$(Q)PATH=../target/release/:$(PATH) vhs record > new_tape.tape
