alias b:=build

default: work_flow 

fmt:
    cargo fmt

build:
    cargo build --release

run:
    cargo run --release

work_flow: 
    just fmt
    just build 
