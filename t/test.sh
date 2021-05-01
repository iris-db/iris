#!/usr/bin/env bash

# Compiles and runs the Go test script, removing it after usage.

SCRIPT_NAME=__TEST_SCRIPT__

go build -o $SCRIPT_NAME t.go && ./$SCRIPT_NAME "$@"
rm ./$SCRIPT_NAME
