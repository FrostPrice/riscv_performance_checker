## How to run

### Starting dev environment

- There are 2 ways that this can be achieved:
  - Using `cargo build` and `cargo run` in sequence
  - **(Recomended)** using `cargo watch` to enable live reload of the program and on development testing. Run the following: `cargo watch -q -c -w src/ -x "run ./riscv_bin_dump/laco5x.txt"` for just the live-reload
