#!/bin/bash

# Run code generation
(cd code-gen && cargo run --quiet -- ../workspace)

(cd workspace && cargo run $@)