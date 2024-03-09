This is a tiny test program.

This is meant to be used with the journal-bot test.

Steps are:

- `cargo build` this app
- move to `journal-bot` directory
- uncomment the `test_stdout` #[test] at the bottom of `main.rs` for the `journal-bot/src/main.rs` 
- run the test with `$ cargo test -- --nocapture`

You should be able to see numbers from 1..9 slowly output to STDOUT.

This was done only to test the usage of STDOUT streams with Rust code. 

A simple way of debugging code and trying around ideas with `std::process::{Command, Stdio};`.

