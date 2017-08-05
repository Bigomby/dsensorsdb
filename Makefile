PREFIX?=/usr/local

build:
	cargo build

release:
	cargo build --release

install:
	install headers/dsensorsdb.h $(PREFIX)/include/dsensorsdb.h
	install target/release/libdsensorsdb.so $(PREFIX)/lib

clean:
	rm -rf target/

install-rustup:
	curl https://sh.rustup.rs -sSf | sh -s -- -y

example:
	gcc examples/example.c -ldsensorsdb -o example

test:
	cargo test
