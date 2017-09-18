# Reconfigurer la langue sur Débian / Ubuntu

Sur Raspberry PI essayer:

	$ sudo raspi-config

Ou alors:

	$ vim /etc/locales.gen
	# decommenter les lignes adequates
	
	$ sudo locale-gen

Sur Ubuntu:

	$ sudo apt-get install --reinstall locales && sudo dpkg-reconfigure locales
	$ locale-gen fr_FR.UTF-8
	$ locale-gen --no-purge --lang fr_FR.UTF-8
