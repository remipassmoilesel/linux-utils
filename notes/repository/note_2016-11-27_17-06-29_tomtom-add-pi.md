# Ajouter des points d'intérêt Tomtom / RLink

Télécharger les points d'intérêts au format ov2:

	$ cd /tmp
	$ wget http://carte-gps-gratuite.fr/RFFov2.rar
	$ wget http://carte-gps-gratuite.fr/RMFov2.rar
	$ unrar e RFFov2.rar
	$ unrar e RMFov2.rar

Retirer la carte SD du véhicule, en prenant soin de la démonter au préalable dans le menu "Système".

Sauvegarder les fichiers de la carte SD (TOMTOM.001, ...)

## Sous Windows
Sous Windows, utiliser RLink Explorer pour intégrer les points d'intérêt. Télécharger et installer le logiciel,
puis ouvrir le fichier TOMTOM.001. Ensuite copier les fichiers (ov2 + bmp) dans le dossier "Europe".

Le programme: 	http://tomtomax.fr/forum/viewtopic.php?f=215&t=40484&p=305578#p305553
Documentation:	http://renault-clio-4.forumpro.fr/t1308-installer-les-zones-de-danger-sur-r-link

## Sous Linux
Les fichiers doivent être montés ensemble pour que leur contenu soit cohérent. Procédure non testée:

	# Monter les fichiers en RAID
	$ losetup /dev/loop0 TOMTOM.000
	$ losetup /dev/loop1 TOMTOM.001
	$ losetup /dev/loop2 TOMTOM.002
	$ losetup /dev/loop3 TOMTOM.003
	$ mdadm --build --auto=part --verbose /dev/md1 --level=linear -n4 /dev/loop[0-3]
	$ mount /dev/md1 /mnt
	
	... Effectuer les opérations nécessaires ... 

	# Démonter le RAID
	$ umount /mnt
	$ mdadm -S /dev/md1
	$ losetup -d /dev/loop[0-3]

Source: http://www.tomtomheaven.com/forum/viewtopic.php?f=20&t=6268


