#!/usr/bin/env bash

# Executes tests by category.

DOCKERFILE_PATH=.
E2E_TEST_ENTRY_PATH=e2e/run_tests.py

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

# Main loop.

case $1 in
	db)
		check_cmd_exists cargo

		cargo test --manifest-path database/Cargo.toml
		;;
	e2e)
		check_cmd_exists docker
		check_cmd_exists python3

		TAG=callistodb/test-e2e

		docker build $DOCKERFILE_PATH -t $TAG
		docker run $TAG

		python3 $E2E_TEST_ENTRY_PATH

		docker rm -f $TAG
		;;
	*)
		help_msg
esac
