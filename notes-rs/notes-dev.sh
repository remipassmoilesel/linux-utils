#!/usr/bin/env bash

export NOTES_STORAGE_DIRECTORY="$(pwd)/.dev-repo"

cargo run $@
