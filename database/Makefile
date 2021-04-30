.PHONY: all

EXPRESSIONS_PATH = ../expressions
STATIC_C_LIB_PATH = build

TOOLCHAINS = +nightly

all: expressions server

expressions:
	@echo Building expressions library
	@$(MAKE) -w -C $(EXPRESSIONS_PATH) lib
	@mkdir -p $(STATIC_C_LIB_PATH)/expressions
	@cp $(EXPRESSIONS_PATH)/dist/* $(STATIC_C_LIB_PATH)/expressions

server:
	@echo Building server release
	@cargo $(TOOLCHAINS) build --release
