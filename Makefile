.PHONY: all
all: cli core


.PHONY: cli
cli: core
	(cd 'cli' && cargo build)
	(cd 'cli' && cargo test)
	(cd 'cli' && cargo fmt -- --check)


.PHONY: core
core:
	(cd 'core' && cargo build)
	(cd 'core' && cargo test)
	(cd 'core' && cargo fmt -- --check)

