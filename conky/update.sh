#!/usr/bin/env bash

set -x

cp ~/linux-utils/conky/conkyrc.lua ~/.conkyrc

mkdir -p ~/.conky/custom
cp custom.lua ~/.conky/custom