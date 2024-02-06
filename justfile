build:
  cargo build

comp_test: build
  ./target/debug/rbfc tests/hello.bf -o output
  fasm output/hello.asm
  ./output/hello
  
