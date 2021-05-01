#!/usr/bin/env bash

# Compiles and runs the Go test script, removing it after usage.

go build . && ./t "$@" && rm ./t
