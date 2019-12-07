# Installer Virtualbox sur Debian Stretch

Ajouter la source APT:

	$ sudo vim /etc/apt/sources.list.d/virtualbox.list

	deb http://download.virtualbox.org/virtualbox/debian stretch contrib

Importer la clef:

	$ curl -O https://www.virtualbox.org/download/oracle_vbox_2016.asc
	$ sudo apt-key add oracle_vbox_2016.asc

Installer Virtualbox + additions:

	$ sudo apt-get update
	$ sudo apt-get install virtualbox-5.1

Source: https://wiki.debian.org/VirtualBox#Debian_9_.22Stretch.22
