# Alpine Linux

Distribution Linux de taille réduite, parfaite pour des conteneurs.

Pour avoir accès aux scripts de démarrage installer le package openrc:
    
    $ apk update && apk add openrc

/!\ Mais il est déconseillé de l'utiliser dans un conteneur.

## Gestionnaire de paquets

Toujours mettre à jour de l'index avant quoi que ce soit:
    
    $ apk update

Installer un paquet

    $ apk add package

Mettre à jour tout le système

    $ apk update
    $ apk upgrade

Mettre à jour seulement quelques paquets

    $ apk add --upgrade busybox 

Lister tous les paquets

    $ apk search -v

Rechercher

    $ apk search -v 'acf*'

Montrer toutes les infos sur un paquet
    
    $ apk info -a zlib

## Lister les paquets installés

    $ apk info

## Ajouter le dépôt testing

Un dépôt Edge existe également.

	$ echo "@testing http://nl.alpinelinux.org/alpine/edge/testing" | tee -a /etc/apk/repositories

## Installer et démarrer SSH

Installation:
	
	$ apk add openssh

Créer des clefs avant démarrage:

	$ ssh-keygen -f /etc/ssh/ssh_host_rsa_key -N '' -t rsa
	$ ssh-keygen -f /etc/ssh/ssh_host_dsa_key -N '' -t dsa

Créer le répertoire de démarrage:
	
	$ mkdir -p /var/run/sshd

Démarrer:

	$ /usr/sbin/sshd -D

Bannière disponible dans:

	$ cat /etc/motd

## Shadow

Meilleurs outils que busybox pour la gestion des droits d'utilisateurs:

	$ apk add shadow


Source: https://wiki.alpinelinux.org/


