#!/usr/bin/env bash

wget http://www.deb-multimedia.org/pool/main/d/deb-multimedia-keyring/deb-multimedia-keyring_2016.8.1_all.deb && dpkg -i deb-multimedia-keyring_2016.8.1_all.deb
curl -fsSL https://apt.dockerproject.org/gpg | sudo apt-key add -
apt-key adv --keyserver ha.pool.sks-keyservers.net --recv-keys 1397BC53640DB551
apt-key adv --keyserver ha.pool.sks-keyservers.net --recv-keys 1397BC53640DB551
apt-key adv --recv-keys --keyserver keyserver.ubuntu.com 0xcbcb082a1bb943db
wget https://nginx.org/keys/nginx_signing.key && apt-key add nginx_signing.key
apt-key adv --keyserver ha.pool.sks-keyservers.net --recv-keys 63F7D4AFF6D61D45
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys F24AEA9FB05498B7
apt-key adv --keyserver ha.pool.sks-keyservers.net --recv-keys 74A941BA219EC810
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 72B97FD1D9672C93
wget -q https://www.virtualbox.org/download/oracle_vbox_2016.asc -O- | sudo apt-key add -
wget -q https://www.virtualbox.org/download/oracle_vbox.asc -O- | sudo apt-key add -
apt-key adv --keyserver ha.pool.sks-keyservers.net --recv-keys 2CC26F777B8B44A1

