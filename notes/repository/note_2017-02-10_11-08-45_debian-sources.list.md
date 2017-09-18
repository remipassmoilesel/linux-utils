# Gérer les sources de paquets pour Debian Stretch

Objectif: activer les dépôts testing pour toutes les installations et les dépôt experimentaux pour installation de logiciels spécififques.

	$ sudo vim /etc/apt/sources.list

	# Testing repositories
	deb http://httpredir.debian.org/debian stretch main contrib non-free
	deb-src http://httpredir.debian.org/debian stretch main contrib non-free

	deb http://httpredir.debian.org/debian stretch-updates main contrib non-free
	deb-src http://httpredir.debian.org/debian stretch-updates main contrib non-free

	deb http://security.debian.org/ stretch/updates main contrib non-free
	deb-src http://security.debian.org/ stretch/updates main contrib non-free

	# unstable and experimental repositories
	deb http://httpredir.debian.org/debian unstable main contrib non-free
	deb-src http://httpredir.debian.org/debian unstable main contrib non-free

	deb http://httpredir.debian.org/debian experimental main contrib non-free
	deb-src http://httpredir.debian.org/debian experimental main contrib non-free


Important: spécifier l'ordre d'utilisation par défaut des dépôts. Ici cet ordre sera privilégié: stable, testing, unstable.
	
	$ sudo vim /etc/apt/preferences.d/settings

	Package: *
	Pin: release a=stable
	Pin-Priority: 700

	Package: *
	Pin: release a=testing
	Pin-Priority: 650

	Package: *
	Pin: release a=unstable
	Pin-Priority: 600

	Package: *
	Pin: release a=experimental
	Pin-Priority: 500

Pour vérifier le comportement d'APT:

	$ sudo apt policy firefox

Pour installer un paquet expérimental:

	$ sudo apt install firefox/experimental
	$ sudo apt -t experimental firefox

Source: http://www.binarytides.com/enable-testing-repo-debian/

