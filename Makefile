.PHONY: all
all: clean cli core


.PHONY: clean
clean:
	@if [ -d 'cli/target' ]; then	\
		rm -rf 'cli/target';	\
	fi

	@if [ -d 'core/target' ]; then	\
		rm -rf 'core/target';	\
	fi


.PHONY: cli
cli: core
	(cd 'cli' && cargo build --release)
	(cd 'cli' && cargo test)
	(cd 'cli' && cargo fmt -- --check)


.PHONY: core
core:
	(cd 'core' && cargo build --release)
	(cd 'core' && cargo test)
	(cd 'core' && cargo fmt -- --check)

