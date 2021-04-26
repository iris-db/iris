#!/usr/bin/env bash

# Executes tests by category

DOCKERFILE_PATH=.
E2E_TEST_ENTRY_PATH=e2e/run_tests.py

# Util

function help_msg {
	echo "\
CallistoDB Test Script
e2e | Executes all e2e tests
db  | Executes all database related tests in the database directory"
}

function check_cmd_exists {
	if ! command -v "$1" &> /dev/null
	then
		echo "command $1 could not be found"
		exit
	fi
}

# Tests

function test_unit {
	check_cmd_exists cargo

	MANIFEST="--manifest-path database/Cargo.toml"

	cargo build "$MANIFEST"
	cargo test "$MANIFEST"
}

function test_e2e {
	check_cmd_exists docker
	check_cmd_exists python3

	TAG=callistodb/test-e2e

	docker build $DOCKERFILE_PATH -t $TAG
	docker run $TAG

	python3 $E2E_TEST_ENTRY_PATH

	docker rm -f $TAG
}


# Main

case "$1" in
	-u|--unit)    TEST_SET="unit"             ;;
	-e|--e2e)     TEST_SET="e2e"              ;;
	-c|--cluster) TEST_SET="cluster"          ;;
	-a|--all)     TEST_SET="unit:cluster:e2e" ;;
	*)            help_msg; exit 0            ;;
esac

TEST_SEP=':' read -r -a TESTS <<< "$TEST_SET"

for TEST in "${TESTS[@]}"
do
	"test_$TEST"
done

echo "Tests completed"