build:
	cargo build

release:
	cargo build --release

install:
	install headers/dsensorsdb.h /usr/local/include/dsensorsdb.h
	install target/release/libdsensorsdb.so /usr/local/lib/

clean:
	rm -rf target/

example:
	gcc examples/example.c -ldsensorsdb -o example

test:
	cargo test
