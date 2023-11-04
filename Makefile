TARGET_DIR=target
TARGET=x86_64-unknown-none


.PHONY: install

clean:
	rm -Rf target

install :
	cargo build --release
	cp $(TARGET_DIR)/$(TARGET)/release/custom_init /var/custominit/init