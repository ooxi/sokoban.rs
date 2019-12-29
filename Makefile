.PHONY: all
all: core


.PHONY: core
core:
	(cd 'core' && cargo build)
	(cd 'core' && cargo test)
	(cd 'core' && cargo fmt -- --check)

