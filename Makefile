CPU = native

# CC = gcc -O4
CC = clang-12 -O3
CFLAGS = -march=$(CPU)

RUSTCPU = -C target-cpu=$(CPU)

all: filter-c.s filter-rs.s

filter-c.s: filter.c Makefile
	$(CC) $(CFLAGS) -S -o filter-c.s filter.c

filter-rs.s: filter.rs Makefile Cargo.toml
	cargo clean
	cargo rustc --release -- $(RUSTCPU) --emit=asm
	cp target/release/deps/filter-*.s filter-rs.s

test-c: filter-c.s
	$(CC) $(CFLAGS) -o test-c filter-c.s

test-rs: filter.rs
	cargo test

clean: 
	cargo clean
	-rm -f test-c filter-c.s filter-rs.s
