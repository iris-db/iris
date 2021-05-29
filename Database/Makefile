.PHONY: all

BSON_PATH = ../bsonDeserializer
STATIC_C_LIB_PATH = build

SHELL = /bin/bash

TOOLCHAINS = +nightly

all: expressions server

expressions:
	@$(MAKE) -w -C $(BSON_PATH) lib
	@mkdir -p $(STATIC_C_LIB_PATH)/bson
	@cp $(BSON_PATH)/dist/* $(STATIC_C_LIB_PATH)/bson

server:
	@echo Building server release
	@cargo $(TOOLCHAINS) build --release
