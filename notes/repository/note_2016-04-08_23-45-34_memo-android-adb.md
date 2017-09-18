# Mémo Android / ADB

## Faire apparaitre un téléphone avec la commande "adb device"

Activer le mode développeur:

	Tapper 7 fois sur le numéro de build dans Paramètres / Système

Ensuite dans options pour les développeurs:

	Activer "Rester activé" et "Debuggage USB"
	Pour rooter "Déverouillage OEM" peut être utile.

Installer les outils adb si nécéssaire:

	$ sudo apt-get install android-tools-fastboot android-tools-adb

Puis connecter le téléphone et activer le mode "Photo"

Ensuite:

	$ adb start-server
	$ adb devices

## Commandes utiles:

Journaux:

	$ adb logcat

Installer une application:

	$ adb install

Ouvrir un shell:

	$ adb shell

## Modes alternatifs

	$ adb reboot fastboot
	$ adb reboot recovery

	Ou:

	Eteindre le téléphone,
	Maintenir Power et Vol Haut
	Relacher Power lorsque l'apareil démarre
	Maintenir Vol Haut une seconde puis relacher

## En cas d'erreur étrange, ou de type "connect: Bad file number" 
Afficher les informations sur les opérations refusées et éventuellement passer SELinux en mode permissif

	$ su
	# dmesg | grep -i denied
	# setenforce 0

## En cas d'erreur lors de l'ouverture d'un port (bdd, ...)
Ajouter l'utilisateur au groupe aid_net:

	$ sudo usermod -aG aid_inet mysql
