build:
	@rm -rf bins
	@mkdir bins

	@cargo build -p benchrunner --release
	@mv target/release/benchrunner bins/benchrunner-glibc

	@cargo build -p benchrunner --release --features=jemalloc
	@mv target/release/benchrunner bins/benchrunner-jemalloc

	@cargo build -p benchrunner --release --features=tcmalloc
	@mv target/release/benchrunner bins/benchrunner-tcmalloc

	@cargo build -p benchrunner --release --features=hoard
	@mv target/release/benchrunner bins/benchrunner-hoard

test:
# Pass the arguments for the binary as ARGS in make command
	@echo "GLIBC"
	bins/benchrunner-glibc ${ARGS}
	@echo ""

	@echo "Jemalloc"
	bins/benchrunner-jemalloc ${ARGS}
	@echo ""

	@echo "TCmalloc"
	bins/benchrunner-tcmalloc ${ARGS}
	@echo ""

	@echo "Hoard"
	bins/benchrunner-hoard ${ARGS}
	@echo ""

build_test: build test