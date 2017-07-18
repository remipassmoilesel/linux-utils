# Installer Tor et un serveur HTTP

## Installation

	$ sudo apt-get install tor

## Configuration
	
	$ sudo vim /etc/tor/torrc

	# dossier où sont enregistrés les fichier de service de tor, dont 'hostname' contenant l'URL du service
	HiddenServiceDir /var/lib/tor/hidden_service-https/

	# Port *virtuel* du service suivi de l'adresse ou faire suivre le traffic, par exemple un serveur HTTPS
	HiddenServicePort 443 127.0.0.1:443

	HiddenServiceDir /var/lib/tor/hidden_service/
	HiddenServicePort 80 127.0.0.1:80

	HiddenServiceDir /var/lib/tor/hidden_service/
	HiddenServicePort 22 127.0.0.1:22

## Droits d'utilisateurs

Si le dossier est indiqué hors dela zone par défaut, modifier les droits:

	$ sudo chown -R debian-tor:debiant-tor /path/to/dir

## Utiliser Tor à travers obfsproxy


