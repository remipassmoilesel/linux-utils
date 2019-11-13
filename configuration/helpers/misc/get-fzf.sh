#!/bin/bash

# mardi 12 juin 2018, 11:34:02 (UTC+0200)

set -x
set -e

mkdir -p /tmp/fzf
cd /tmp/fzf
wget 'https://github.com/junegunn/fzf-bin/releases/download/0.17.4/fzf-0.17.4-linux_amd64.tgz'
tar -xvf 'fzf-0.17.4-linux_amd64.tgz'
chmod +x fzf
sudo mv fzf /usr/local/bin

