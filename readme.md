# DTMF Rust

This program decodes DTMF signals straight from the microphone and prints the result in the console in real-time. It is written in Rust, a powerful and efficient programming language. If you're interested in DTMF signal processing or Rust development, this project might be a great starting point for you.

## Installation

To run the program, you first need to clone the repository and navigate into the project directory using the following command:

```shell
git clone https://github.com/denizsincar29/dtmf_rust.git
cd dtmf_rust
```

The program uses Cargo, the package manager and build system for Rust, to handle dependencies and build the project. Make sure you have Rust and Cargo installed on your system. You can install Rust by following the instructions provided on the [official Rust website](https://www.rust-lang.org/).

It's recommended to use cargo make tool to build this package. Windows users are required to do this, because it will copy the dll file and the executable into dtmf_decoder directory. You can than ship this directory everywhere.  
First, install cargo_make:

```shell
cargo install --force cargo-make
```

Once you have Rust and Cargo set up, you can use the following command to run the program in release mode:

```shell
cargo make release
```

linux and mac users can use the simple "cargo run --release", however I don't know how to ship the target binary, so any help would be appritiated.  
While running, let the program to listen some DTMF signals. E.g. open your phone's keypad, turn up the volume and press some keys. you will get them printed out in console.  
Please note that the program does not currently support sound device selection and listing.

# Note for screenreader users

For some reason, if just 1 letter is printed into the console, NVDA doesn't read it. You need to manually check the output. Please suggest me how to improve this.
UPD: as a workarround I added parentheses around the printed number.


## Contributing

Contributions to this project are welcome! If you have any ideas for improvements, bug fixes, or new features, feel free to submit a pull request.

