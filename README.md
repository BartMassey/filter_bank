# filter_bank: optimization of filter bank function in Rust and C
Bart Massey

The code here originated from this Reddit
[thread](https://www.reddit.com/r/rust/comments/mscxr7/why_no_loop_unrolling_in_this_function/). It apparently
is an implementation of a piece of a DSP filter bank.

The Rust version here is my rewrite of that code to clean it
up a bit. The C version was hand-translated from the
rewritten Rust version. Both versions include very minimal
tests that nothing was harmed in the process.

Build with `make`. Please check the `Makefile` for CPU type
settings and C compiler options, and check the `Cargo.toml`
for other Rust stuff. `make clean` to clean up, `make test`
to run the tests: an assertion will happen if a test fails.

To summarize my results:

* My Rust rewrite produced better code than the original.

* `rustc` and `clang-12` produced an essentially identical
  inner loop.

* GCC 10.2 managed to use `VFMA` and `VNFMA` instructions to
  improve on the LLVM result somewhat, although the C
  standard maybe possibly does not allow this by default due
  IEEE standards… stuff. Use the `fma` feature with the Rust
  code to get a version that uses `f32::mul_add()` to get
  essentially the GCC inner loop.

* None of the compilers managed to vectorize the code, which
  in retrospect is not surprising given its data
  dependencies in the inner loop.

* On my recent machine (`znver3`) the inner loop is not
  unrolled.  On my older machine (`haswell`) the inner loop
  is unrolled. The difference might be that the compiler CPU
  model detects no performance advantage from unrolling with
  the newer CPU.
