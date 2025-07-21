#!/bin/bash

# Small script to set up a shell with must-have tools

cat << Banner

██╗     ██╗███╗   ██╗██╗   ██╗██╗  ██╗    ██╗   ██╗████████╗██╗██╗     ███████╗
██║     ██║████╗  ██║██║   ██║╚██╗██╔╝    ██║   ██║╚══██╔══╝██║██║     ██╔════╝
██║     ██║██╔██╗ ██║██║   ██║ ╚███╔╝     ██║   ██║   ██║   ██║██║     ███████╗
██║     ██║██║╚██╗██║██║   ██║ ██╔██╗     ██║   ██║   ██║   ██║██║     ╚════██║
███████╗██║██║ ╚████║╚██████╔╝██╔╝ ██╗    ╚██████╔╝   ██║   ██║███████╗███████║
╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═╝     ╚═════╝    ╚═╝   ╚═╝╚══════╝╚══════╝

Banner

echo
echo "Running script for user $USER"
echo

set -x
set -e

# Tools

cd ~
sudo dnf install byobu vim curl wget ranger ncdu zsh pass jq cmatrix tree \
                git tig git-extras python-pip nmap z htop @development-tools \
                make automake gcc gcc-c++ kernel-devel

# Own utilities and scripts

cd ~
git clone https://github.com/remipassmoilesel/notes-repository ~/notes-repository
git clone https://github.com/remipassmoilesel/linux-utils ~/linux-utils/

# Change shell, before installing oh my zsh

sudo chsh ${USER} -s /bin/zsh

# Oh my zsh

sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Configuration

rm ~/.zshrc
ln -s ~/linux-utils/configuration/dot-files/dot-zshrc ~/.zshrc
rm ~/.vimrc
ln -s ~/linux-utils/configuration/dot-files/dot-vimrc ~/.vimrc
rm ~/.gitconfig
ln -s ~/linux-utils/configuration/dot-files/dot-gitconfig ~/.gitconfig

zsh
