#!/usr/bin/env bash

set -x
set -e

mkdir -p ~/.conky/custom

cp ~/linux-utils/conky/conkyrc.lua ~/.conkyrc
cp ~/linux-utils/conky/custom.lua ~/.conky/custom
