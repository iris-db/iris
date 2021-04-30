LIB_NAME=expressions

DIST=dist/lib${LIB_NAME}.a

.PHONY: all

all: lib

lib:
	@echo Building static C library
	@go build -buildmode=c-archive -o dist/libexpressions.a main.go
