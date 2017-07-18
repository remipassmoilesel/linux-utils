# Monter une archive pour lecture / écriture

Source: http://linuxpoison.blogspot.fr/2013/03/mount-archives-for-readwrite-under.html

Installation:

	$ sudo apt-get install archivemount

Monter une archive:

	$ archivemount archive.xyz mount-point/

Démonter l'archive (modifications sauvegardées, et archive originale également):

	$ sudo umount mount-point/

