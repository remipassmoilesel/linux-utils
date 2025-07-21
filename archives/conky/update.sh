#!/usr/bin/env bash

set -x
set -e

BASE_PATH=~/linux-utils/conky
DESTINATION=~/.conky/remipassmoilesel

cp ${BASE_PATH}/conky_draw.lua ${DESTINATION}
cp ${BASE_PATH}/conky_draw_config.lua ${DESTINATION}
cp ${BASE_PATH}/conky_draw_templates.lua ${DESTINATION}
cp ${BASE_PATH}/conky_helpers.lua ${DESTINATION}
cp ${BASE_PATH}/conky_main.lua ${DESTINATION}
cp ${BASE_PATH}/conkyrc.lua ~/.conkyrc
