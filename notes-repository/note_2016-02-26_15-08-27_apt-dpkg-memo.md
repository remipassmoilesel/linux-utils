# Mémo apt-get / dpkg

## Généralités

Pour adapter un paquet récent à une version lts, voir l'utilitaire "adapt"

Mise à jour nécéssaire avant chaque opération:
    
    $ sudo apt-get update

Mise a jour du système:
    
    $ sudo apt-get upgrade # mise à jour simple
    $ sudo apt-get dist-upgrade # mise à jour complète

Rechercher un paquet:
    
    $ apt-get search keywords

Informations sur un paquet:
    
    $ apt-cache show packqage

Afficher les fichiers d'un paquet:

    $ sudo apt-get install apt-file
    $ apt-file update
    $ apt-file show paquet

Lister toutes les sources d'apt:

    $ grep ^ /etc/apt/sources.list /etc/apt/sources.list.d/*

Utiliser un PPA (Personnal Package Archive):
    
    $ sudo add-apt-repository ppa:damien-moore/codeblocks-stable 
    $ sudo apt-get update

Retirer un ppa:
    
    $ sudo add-apt-repository --remove ppa:elementary-os/daily

Lister les paquets installés:
    
    $ sudo dpkg --get-selections
    # Ou
    # sudo dpkg -l

Suppresion automatique des paquets inutiles:
    
    $ sudo apt-get autoremove

Convertir un paquet rpm en deb:
    
    $ alien MySQL-zrm-1.1-1.noarch.rpm

Suppression d'un paquet et de tous les fichiers de config:
    
    $ sudo apt-get purge paquet

Installation, désinstallation dpkg:

    $ sudo dpkg -i fichier.deb
    $ sudo dpkg -r paquet

Résolution automatique des problèmes de dépendances:

    $ sudo apt-get -f install

## Erreurs

En cas d'erreurs étranges de dépendances, vérifier que le fichier /etc/apt/sources.list est correct.
Il vaut mieux utiliser un générateur en ligne, notamment pour passer de Debian Stable à Testing.


## Erreur avec l'installation de PPA

Executer:

	# echo "deb http://ppa.launchpad.net/deadsnakes/ppa/ubuntu xenial main" > /etc/apt/sources.list.d/python.list
	# apt-get install add-apt-key && add-apt-key -k hkp://keyserver.ubuntu.com:80 6A755776

Pour trouver l'id de la clef, ajouter la source et faire un APT update.
