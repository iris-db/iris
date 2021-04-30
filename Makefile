SUBDIRS = database

.PHONY: $(SUBDIRS) all

all: $(SUBDIRS)

$(SUBDIRS):
	@echo Building
	@$(MAKE) -w -C $@ all

image:
	@echo Building docker image
	@echo TODO

ci:
	@echo Executing all tests
	./test.sh -a
