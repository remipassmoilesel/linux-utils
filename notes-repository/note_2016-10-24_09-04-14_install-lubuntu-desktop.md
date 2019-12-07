# Installer l'environnement de bureau LXDE sur Ubuntu

Version adaptée à Ubuntu:

	$ sudo apt-get install lubuntu-desktop  

Ecran de démarrage et d'arrêt:

	$ sudo apt-get install plymouth-themes plymouth-theme-lubuntu-logo plymouth-theme-lubuntu-text 
	$ sudo reboot
	$ sudo update-alternatives --config default.plymouth 

Quelquefois un autre redémarrage est nécéssaire.

Pour améliorer l'expérience utilisateur, installer un menu dynamique:

	$ sudo apt-get install synapse
	
	CTRL + ESPACE pour ouvri le menu de recherche

Personnalisation:

	$ obconf &

Editer les raccourscis clavier avec GUI:

	$ cd playground
	$ wget https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/obkey/obkey-1.0.tar.gz
	$ tar -xf obkey-1.0.tar.gz
	$ cd obkey-1.0
	$ ./obkey

Ou editer les fichiers:

	$ cd ~/.config/openbox/
	>> rc.xml lubuntu-rc.xml lxde-rc.xml

	# Chercher le raccourci utilisant la touche L
	$ grep -rnw . -e "l"
	
