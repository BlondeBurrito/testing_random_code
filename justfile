set shell := ["pwsh", "-c"]
alias t := test
alias b := build
alias r := run

test:
  cargo test --release

build: test
  cargo build --release

run: build
  cargo run --release
