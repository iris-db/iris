SUBDIRS = source

.PHONY: $(SUBDIRS) all

all: $(SUBDIRS)

$(SUBDIRS):
	@$(MAKE) -w -C $@ all

image:
	@echo Building docker image
	@echo TODO

ci:
	@echo Executing all tests
	./test.sh -a
