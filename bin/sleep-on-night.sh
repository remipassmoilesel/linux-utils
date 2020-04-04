#!/usr/bin/env bash

# To enable this script:
#
#   $ sudo cp ~/linux-utils/bin/sleep-on-night.sh /usr/local/bin
#   $ sudo crontab -e
#
#   30 0 * * * sudo /usr/local/bin/sleep-on-night.sh

rtcwake -m disk -l -t $(date +%s -d 'tomorrow 07:30')