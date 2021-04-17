CPU = native

# CC = gcc -O4
CC = clang-12 -O3
CFLAGS = -march=$(CPU)

FMA = --features=fma
RUSTCPU = -C target-cpu=$(CPU)

all: filter-c.s filter-rs.s

filter-c.s: filter.c Makefile
	$(CC) $(CFLAGS) -S -o filter-c.s filter.c

filter-rs.s: filter.rs Makefile Cargo.toml
	cargo clean
	cargo rustc $(FMA) --release -- $(RUSTCPU) --emit=asm
	cp target/release/deps/filter-*.s filter-rs.s

test-c: filter-c.s Makefile
	$(CC) $(CFLAGS) -o test-c filter-c.s

test: filter-rs.s test-c Makefile Cargo.toml
	cargo test $(FMA)
	./test-c

clean: 
	cargo clean
	-rm -f test-c filter-c.s filter-rs.s
