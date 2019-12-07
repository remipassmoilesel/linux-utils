# Installer un thème Mate

Visiter Gnome Look et rechercher un thème **GTK 2**: https://www.gnome-look.org/browse/cat/136/ord/download/

Télécharger et installer les dépendances si nécéssaire:

	$ sudo dpkg -i gtk2-engines-murrine_0.90.3\ git20100810-0ubuntu1_amd64.deb  

Puis télécharger et décompresser le thème:

	$ cd ~/.themes
	$ tar -xf theme.tar.gz

Quelquefois nécéssite une mise à jour plus reboot:

	$ sudo apt-get update && sudo apt-get upgrade -y && sudo reboot now

Liste de thèmes compatibles:
	https://www.maketecheasier.com/9-great-mate-themes-linux/
