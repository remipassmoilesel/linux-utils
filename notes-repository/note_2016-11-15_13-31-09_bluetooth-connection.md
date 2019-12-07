# Connexion Bluetooth en ligne de commande et graphiquement

Installer une application de connexion graphique:

	$ sudo add-apt-repository ppa:embrosyn/cinnamon  
	$ sudo apt-get update && sudo apt-get install blueberry

	> Note: d'autres applications (ex: blueman) ne demandent pas forcemment de code pin, ce qui empêche la connexion.

Connexion en ligne de commande:

	$ sudo apt-get install bluez bluez-utils
	$ modprobe btusb 	# chargement du pilote

	$ bluetoothctl -a
	$ list		# afficher les controlleur (interfaces) bluetooth disponibles
	$ select ...	# sélectionner l'interface à utiliser
	$ help 		# liste des commandes, auto complétion disponible avec TAB
	$ power on 	# allumer le controlleur
	$ scan on	# démarrer le scan de périphérique

	> Ensuite activer la touche "bluetooth" du périphérique

	$ pair 20:....	# identifier en paire le périphérique

	> Ensuite taper le code pin afficher et appuyer sur "Entrée"

	$ connect 20:....

	> Quelquefois il faut à nouveau appuyer sur la touche "bluetooth"

	$ disconnect	# se déconnecter
	$ exit		# quitter, sans forcemment déconnecter


Manipuler des interfaces bluetooth:

	$ sudo hciconfig -a		# afficher les interfaces disponibles
	$ sudo hciconfig hci0 down	# arrêter une interface
	$ sudo hciconfig hci0 up	# démarrer une interface

Script utilitaire pour changer d'interface:

	$ sudo vim /opt/activate_bluetooth.sh

	#!/bin/bash
	sudo hciconfig hci0 down
	sudo hciconfig hci1 up

	alert 'Bluetoot activated'

	> Puis faire une entrée avec MenuLibre dans le menu de démarrage

Débloquer le bluetooth en cas d'impossibilité de mettre en place une interface:

	$ sudo rfkill unblock bluetooth
	$ sudo service bluetooth restart
