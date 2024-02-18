.PHONY: run build

run:
	./target/debug/tree-walk test.txt

build:
	cargo build
