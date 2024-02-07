build:
  cargo build

int FILE $RUST_LOG="warn": build
  ./target/debug/rbfc tests/{{FILE}}.bf -i

comp FILE: build
  mkdir -p output
  ./target/debug/rbfc tests/{{FILE}}.bf -o output
  fasm output/{{FILE}}.asm

run FILE:
  just comp {{FILE}}
  ./output/{{FILE}}

clean_output:
  rm -rf output

clean: clean_output
  cargo clean
