TEST_TARGETS=$(patsubst %/,%-stamp,$(wildcard */*/))

all: ${TEST_TARGETS}

clean:
	-rm -f ${TEST_TARGETS}

%-stamp: % %/src/main.rs
	cd $< && cargo test
	touch $@

.PHONY: clean
