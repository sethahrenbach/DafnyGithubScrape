Code for checking the benchmark files

Requirements:
  - Rust
  - Python >= 3.11
  - Dafny >= 4.6.0
  - Z3 == 4.12.1

To run:

1. `cd dafny_qa_scripts`
1. `cargo build`
1. `cargo run`

Deals with the benchmark in chunks of 20, and writes the failed Dafny files for each chunk to a `.txt` file. Run
 
 ```python get_fails.py``` 
 
to generate a single txt file of the failed Dafny files.