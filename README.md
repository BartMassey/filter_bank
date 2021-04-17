# filter_bank: optimization of filter bank function in Rust and C
Bart Massey

The code here originated from this Reddit
[thread](https://www.reddit.com/r/rust/comments/mscxr7/why_no_loop_unrolling_in_this_function/). It apparently
is an implementation of a piece of a DSP filter bank.

The Rust version here is my rewrite of that code to clean it
up a bit. The C version was hand-translated from the Rust
version. Both versions include very minimal tests that
nothing was harmed in the process.

Build with `make`. Please check the `Makefile` for CPU type
settings and C compiler options.

To summarize my results: `rustc` and `clang` produce an
essentially identical inner loop. None of the compilers
manage to vectorize the code, which is not surprising given
its data dependencies in the inner loop.

On my recent machine (`znver3`) the loop is not unrolled.
On my older machine (`haswell`) the loop is unrolled. The
difference might be that the compiler CPU model detects no
performance advantage from unrolling with the newer CPU.
