# ------------------
# Internal variables
dummy1 := $(shell mkdir bin 2> /dev/null)

# ------------------
# Primary targets
all: bin/runits

check: bin/test-runits
	export RUST_LOG=runits=1 && ./bin/test-runits

check1: bin/test-runits
	export RUST_LOG=runits=2 && ./bin/test-runits units::test_div_unit

install:
	install -p `find bin -name "librunits*" -type f -maxdepth 1` /usr/local/lib/rust

clean:
	rm -rf bin
	
# ------------------
# Binary targets 
src/generated.rs: units.in gen_units.py
	./gen_units.py --in=units.in --out=src/generated.rs

bin/runits: src/runits.rc src/*.rs
	rustc -o $@ $<

bin/test-runits: src/runits.rc src/*.rs
	rustc --test -o $@ $<
