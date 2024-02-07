build:
  cargo build

int FILE $RUST_LOG *FLAGS: build
  ./target/debug/rbfc tests/{{FILE}}.bf -i {{FLAGS}}

comp FILE *FLAGS: build
  mkdir -p output
  ./target/debug/rbfc tests/{{FILE}}.bf -o output {{FLAGS}}
  fasm output/{{FILE}}.asm

run FILE *FLAGS:
  just comp {{FILE}} {{FLAGS}}
  ./output/{{FILE}}

clean_output:
  rm -rf output

clean: clean_output
  cargo clean
