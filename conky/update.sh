#!/usr/bin/env bash

set -x
set -e

DESTINATION=~/.conky/remipassmoilesel

cp ~/linux-utils/conky/conky_draw.lua ${DESTINATION}
cp ~/linux-utils/conky/conky_draw_config.lua ${DESTINATION}
cp ~/linux-utils/conky/conky_helpers.lua ${DESTINATION}
cp ~/linux-utils/conky/conky_main.lua ${DESTINATION}
cp ~/linux-utils/conky/conkyrc.lua ~/.conkyrc
