.PHONY: all

EXPRESSIONS_PATH = ../expressions
STATIC_C_LIB_PATH = build

TOOLCHAINS = +nightly

all: expressions test server

expressions:
	@echo Building expressions lib
	@$(MAKE) -w -C $(EXPRESSIONS_PATH)
	@mkdir -p $(STATIC_C_LIB_PATH)/expressions
	@mv $(EXPRESSIONS_PATH)/dist/* $(STATIC_C_LIB_PATH)/expressions

server:
	@echo Building server release
	@cargo $(TOOLCHAINS) build --release

test:
	@echo Running unit tests
	@cargo $(TOOLCHAINS) test
