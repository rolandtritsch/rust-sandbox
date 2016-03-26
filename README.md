# My Rust Sandbox - playing with Rust and Cargo and Crate

To make this work ...

* ... you need to install rust (e.g. `brew install rust`) and cargo (e.g. `brew install cargo`) and git (e.g. `brew install git`)
* ... and you need to clone this repo (e.g. `git clone https://github.com/rolandtritsch/rust-sandbox`)
* ... and then you can start to run some of the examples by changing into the respective sub directories and run `cargo run`

For the jni example ...

* ... you need to install scala or sbt (recommended: `brew install sbt`)
* ... you need to go to `rust-adder` and need to run `cargo build`. You can then run `javac Adder.java && java Adder 1 2` from java-adder and `sbt "run 1 2"` from scala-adder
* ... you need to go to `rust-matmul` and need to run `cargo test -- --nocapture`. You can then run `sbt "run 10"` from scala-matmul
