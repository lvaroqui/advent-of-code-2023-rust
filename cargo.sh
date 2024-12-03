#!/bin/bash

set -e

# Run code generation
(cd code-gen && cargo run --quiet -- ../solvers generate_imports)

# Run solver
(cd solvers && cargo $@)
