.DEFAULT_GOAL := run

.PHONY: run watch clean

run:
	cargo run

watch:
	watchexec --restart --watch src --watch Cargo.toml -- cargo run

clean:
	cargo clean
