## How to run
A small project focused on classifying instruction types within a bin dump file originating from the RISCV-i32 CPU architecture. This endeavor involves a meticulous comparison of code execution between two distinct organizational structures, with the primary objective of assessing their relative performance and efficiency.

A project focused on classifying instruction types within a bin dump file originating from the RISCV-i32 CPU architecture. This endeavor involves a meticulous comparison of code execution between two distinct organizational structures, with the primary objective of assessing their relative performance and efficiency.

### Starting dev environment

- There are 2 ways that this can be achieved:
  - Using `cargo build` and `cargo run` in sequence
  - **(Recomended)** using `cargo watch` to enable live reload of the program and on development testing. Run the following: `cargo watch -q -c -w src/ -x run` for just the live-reload
