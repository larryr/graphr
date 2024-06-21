

# reference dot to use to verify .gv files

help:
	@echo " -- reference dot tests --"
	@echo ""
	@echo "test    -- generate reference output for all .gv files in dot/ directory"


.PHONY: test
test: refout
	for f in $(wildcard *.gv); do \
	     echo "Testing $$f"; \
	     ./refdot.sh -Tsvg -o refout/$$f.svg $$f; \
	 done


refout:
	mkdir -p refout

clean:
	rm -rf refout
