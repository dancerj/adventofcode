TEST_TARGETS=$(patsubst %/,%-stamp,$(wildcard */*/))

all: ${TEST_TARGETS}

clean:
	-rm -f ${TEST_TARGETS}

%-stamp: %
	cd $^ && cargo test
	touch $@

.PHONY: clean
