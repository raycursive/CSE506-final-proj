build:
	@rm -rf bins
	@mkdir bins

	@cargo build -p benchrunner --release --features=glibcmalloc
	@mv target/release/benchrunner bins/benchrunner-glibc

	@cargo build -p benchrunner --release --features=jemalloc
	@mv target/release/benchrunner bins/benchrunner-jemalloc

	@cargo build -p benchrunner --release --features=tcmalloc
	@mv target/release/benchrunner bins/benchrunner-tcmalloc

#@cargo build -p benchrunner --release --features=hoard
#@mv target/release/benchrunner bins/benchrunner-hoard

test:
	@echo "Warmup..."
	@for i in {1..5}; do bins/benchrunner-tcmalloc ${ARGS} > /dev/null; done

	@echo "GLIBC"
	bins/benchrunner-glibc ${ARGS}
	@echo ""

	@echo "Jemalloc"
	bins/benchrunner-jemalloc ${ARGS}
	@echo ""

	@echo "TCmalloc"
	bins/benchrunner-tcmalloc ${ARGS}
	@echo ""

#@echo "Hoard"
#bins/benchrunner-hoard ${ARGS}
#@echo ""

test-timev:
	@echo "Warmup..."
	@for i in {1..5}; do bins/benchrunner-glibc ${ARGS} > /dev/null; done

	@echo "GLIBC"
	/usr/bin/time -f %U,%S,%P,%M,%R,%w  bins/benchrunner-glibc ${ARGS}
	@echo ""

	@echo "Jemalloc"
	/usr/bin/time -f %U,%S,%P,%M,%R,%w bins/benchrunner-jemalloc ${ARGS}
	@echo ""

	@echo "TCmalloc"
	/usr/bin/time -f %U,%S,%P,%M,%R,%w bins/benchrunner-tcmalloc ${ARGS}
	@echo ""

#@echo "Hoard"
#/usr/bin/time -v bins/benchrunner-hoard ${ARGS}
#@echo ""

build_test: build test
