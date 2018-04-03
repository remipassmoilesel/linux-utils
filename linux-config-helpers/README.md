# Linux configuration helpers

Install minimum tools:

	$ curl https://raw.githubusercontent.com/remipassmoilesel/linux-utils/master/linux-config-helpers/cli-config | bash

Install more tools:
	
	$ installation-script -i

Installation script is a small utility to set up a Debian based distro as I like :)

	usage: installation-script [-h] [-i] [-e] [-ai] [-au] [-d]

	Automation script usefull to prepare Debian based systems. Install or
	uninstall packages, and execute various commands. Packages and commands are
	saved in: /home/remipassmoilesel/linux-utils/linux-rapid-
	install/installation.json

	optional arguments:
	  -h, --help            show this help message and exit
	  -i, --install         install packages and run commands
	  -e, --edit            edit list of commands and packages
	  -ai, --append-packet-to-install
				add a package to install
	  -au, --append-packet-to-uninstall
				add a package to uninstall
	  -d, --display         print list of packages and commands
