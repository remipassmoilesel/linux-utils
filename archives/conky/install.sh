#!/usr/bin/env bash

set -x
set -e

mkdir -p ~/.conky/remipassmoilesel
mkdir -p ~/.config/systemd/user

cp ~/linux-utils/conky/conky.service ~/.config/systemd/user/conky.service

sudo apt -y install conky-all fonts-font-awesome

~/linux-utils/conky/update.sh

systemctl --user daemon-reload
systemctl --user enable conky
systemctl --user start conky
systemctl --user status conky