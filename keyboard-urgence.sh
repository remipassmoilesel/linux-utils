#!/bin/bash

# dimanche 5 février 2017, 23:10:16 (UTC+0100)

echo "Install and launch a virtual keyboard ..."

# update but don't fail
sudo apt update || true

sudo apt install florence

florence &
