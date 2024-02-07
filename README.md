# Rust Brainfuck Compiler (and Interpreter)

My dream is to implement a selfhosted language for fun at some point. So this is my first endavour into writing a compiler.

## Info

The compiler and Interpreter currently do not work the same way and I haven't figured out why yet. I'm unsure if I want to keep the `--wrap` flag. For now the `rot.bf` test only works compiled without wrapping.

## Usage

```
The arguments for the program

Usage: rbfc [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  The file to interpret

Options:
  -o, --output <OUTPUT>  The output folder
  -i, --interpret        Whether to interpret the file
  -w, --wrap             Whether to wrap the tape
  -h, --help             Print help
```

The compilation compiles to `.asm` in the fasm assembler language. To make it executable it has to be assembled using `fasm`:

```bash
fasm [output].asm
./output
```

## Flake and direnv

This program includes a flake which is currently only used for the dev shell. It includes everything needed for rust development as well as `fasm`. To use it run:

```bash
nix develop
```

This can also be done automatically by using [direnv](https://direnv.net/). If it is setup and allowed the shell should load automatically when entering the directory.

## Documentation

For fun I've implemented all of the used structs and functions in a lib and wrote/generated Docs for it using github copilot. The docs can be viewed using:

```bash
cargo doc --open
```

I'm still working on a github pages action to deploy them automatically.
