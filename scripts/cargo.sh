#!/bin/bash

set -e

# TODO: check if day exists, if not propose to create it
# TODO: add utility to run tests
# TODO: import 2021
# TODO: remove old repos from github and rename this one


# Run code generation
(cd code-gen && cargo run --quiet -- ../solvers generate_imports)

# Run solver
(cd solvers && cargo $@)
