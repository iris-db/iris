LIB_NAME=bson

DIST=dist/lib${LIB_NAME}.a

.PHONY: all

all: lib

lib:
	@echo Building expressions as a static library
	@go build -buildmode=c-archive -o dist/libexpressions.a main.go
