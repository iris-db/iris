#!/usr/bin/env bash

# Executes tests by category

DOCKERFILE_PATH=.
E2E_TEST_ENTRY_PATH=e2e/run_tests.py

CARGO_MANIFEST="source/Cargo.toml"

# Util

function help_msg {
	echo "\
options:
  -u --unit    run unit tests only
  -c --cluster run cluster tests only
  -e --e2e     run e2e tests only
  -a --all     run all tests"
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

	cargo +nightly build --manifest-path "$CARGO_MANIFEST"
	cargo +nightly test --manifest-path "$CARGO_MANIFEST"
}

function test_e2e {
	check_cmd_exists docker
	check_cmd_exists python3

	TAG=iris/test-e2e

	docker build $DOCKERFILE_PATH -t $TAG
	docker run $TAG

	python3 $E2E_TEST_ENTRY_PATH

	docker rm -f $TAG
}


# Main

set -e

case "$1" in
	-u|--unit)    TEST_SET="unit"             ;;
	-e|--e2e)     TEST_SET="e2e"              ;;
	-c|--cluster) TEST_SET="cluster"          ;;
	-a|--all)     TEST_SET="unit:cluster:e2e" ;;
	*)            help_msg; exit 0            ;;
esac

# Prepare tests
make all

TEST_SEP=':' read -r -a TESTS <<< "$TEST_SET"

for TEST in "${TESTS[@]}"
do
	"test_$TEST"
done

echo "Tests completed"
