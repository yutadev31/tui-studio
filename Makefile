PLUGINS := $(shell find plugins -mindepth 1 -maxdepth 1 -type d -exec basename {} \;)

all: run

build:
	cargo build

copy: build
	mkdir -p ~/.tui-studio/debug/plugins
	for plugin in $(PLUGINS); do \
		cp target/debug/$$plugin ~/.tui-studio/debug/plugins/; \
	done

run: copy
	cargo run --package tui-studio

install:
	cargo install --path .
