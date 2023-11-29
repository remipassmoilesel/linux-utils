#!/bin/bash

# jeudi 22 08 2016, 08:59:10 (UTC+0200)

# If you like to live dangerously:
# curl https://raw.githubusercontent.com/remipassmoilesel/linux-utils/master/config-helpers/cli-config | bash

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

# Tools

cd ~
sudo pacman -Syu byobu vim curl wget ranger ncdu zsh pass jq cmatrix tree \
                git tig git-extras python-pip nmap z base-devel


# Yay

cd /tmp
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si


# Better git diff
cd ~
yay -S git-delta-bin


# Rust
cd ~
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
rustup default nightly

cargo install notes


# Own utilities and scripts

cd ~
git clone https://github.com/remipassmoilesel/notes-repository ~/.notes
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

