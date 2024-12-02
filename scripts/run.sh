#!/bin/bash

set -e

# TODO: check if day exists, if not propose to create it
# TODO: add utility to run tests

# Run code generation
(cd code-gen && cargo run --quiet -- ../solvers generate_imports)

# Run solver
(cd solvers && cargo run $@)
