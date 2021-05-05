LIB_NAME=bson

DIST=dist/lib${LIB_NAME}.a

.PHONY: all

all: lib

lib:
	@echo Building $(LIB_NAME) as a static library
	@go build -buildmode=c-archive -o dist/lib$(LIB_NAME).a bson_deserializer.go
