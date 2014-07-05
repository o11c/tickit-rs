RUSTC = rustc
RUSTCFLAGS = -g --opt-level=2

PKGCONFIG_TICKIT = $$(pkg-config --libs tickit) $$(pkg-config --libs-only-L tickit | sed 's:-L:-Wl,-rpath=:')

default: tickit
all: demo tickit
demo: $(patsubst examples/demo-%.rs,demo-%,$(wildcard examples/demo-*.rs))
demo-%: examples/demo-%.rs tickit.stamp
	${RUSTC} ${RUSTCFLAGS} $< -L .

source.stamp: $(filter-out src/test.rs,$(wildcard src/*.rs))
	@touch $@

tickit.stamp: src/lib.rs source.stamp
	${RUSTC} ${RUSTCFLAGS} $< -C link-args="${PKGCONFIG_TICKIT}"
	@touch $@
tickit: tickit.stamp
.SUFFIXES:

test.stamp: src/test.rs tickit.stamp
	${RUSTC} ${RUSTCFLAGS} $< -L . --cfg=test -o run-tests
	@touch $@
test: test.stamp
	./run-tests

clean:
	rm -f libtickit*.so run-tests demo-input demo-pen demo-termctl demo-timer demo-xterm256 *.stamp
