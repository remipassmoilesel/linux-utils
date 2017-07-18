# Installer la dernière version de Firefox sur Stretch

Ajouter la clef de dépôt de Firefox:

	$ wget -q -O - http://mozilla.debian.net/archive.asc | sudo apt-key add -

Ajouter le dépôt comme source:

	$ sudo bash -c "echo 'deb http://mozilla.debian.net/ jessie-backports firefox-release' > /etc/apt/sources.list.d/mozilla-firefox.list"

Vérifier et installer:

	$ apt-cache policy firefox
	$ sudo apt-get update 
	$ sudo apt-get install firefox


Source: https://blog.sleeplessbeastie.eu/2016/03/21/how-to-use-recent-version-of-firefox-in-debian-jessie/
