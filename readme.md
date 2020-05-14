# Vanilla Rust Command Line Project

This repository shows a basic setup for a command line application in Rust.


## Getting Started

Rust is expected to be installed on your system.


### Installing

After cloning this repository, change into the newly created directory and run

```
cargo build
```

This will download all dependencies needed for the project and build/compile it.


## Running the Tests

All tests can be run by executing

```
cargo test
```

`test` will automatically run all tests found in the source modules.


### Testing Approach

The test for function `example::message` is only verifying the return value of it.

Testing `main::print_messaage` on the other hand is done via a test-double that gets injected.
This allows us to _spy_ on the output it produces.
We want to avoid printing anything to the screen while running the tests.
Injecting a test double in this instance is a nice way to isolate our application from the command line.


## Running the Application

To run the application we can run it with cargo: `cargo run`.
You should see the text &ldquo;Rust Example&rdquo; being printed.

```
$: cargo run
   Compiling rust-command-line v0.1.0 (/home/vanilla/rust-command-line)
    Finished dev [unoptimized + debuginfo] target(s) in 1.90s
     Running `target/debug/rust-command-line`
Rust Example
```


## Built With

- [Rust](https://www.rust-lang.org)


## License

This project is licensed under the MIT License - see the [license.md](license.md) file for details.

