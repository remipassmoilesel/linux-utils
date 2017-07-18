# Partager un dossier entre une machine vituelle VB et son hôte

## Hôte:

Créer le répertoire à partager:

	$ cd ~
	$ mkdir virtualbox_shared

Créer ou démarrer ensuite la machine virtuelle, et paramétrer un dossier de partage:

	Machine > Configuration > Dossier partagés > Nouveau

	Chemin du dossier: 	/home/.../virtualbox_shared
	Nom du dossier: 	virtualbox_shared

## Machine virtuelle 

Installer les additions invité:

	Périphériques > Insérer l'image CD des additions invités
	$ sudo apt-get install build-essential
	$ /media/.../.../VBoxLinuxAdditions.run
	
Redémarrer et monter le dossier:

	$ sudo reboot now
	$ cd ~
	$ mkdir shared
	$ sudo mount -t vboxsf virtualbox_shared shared

