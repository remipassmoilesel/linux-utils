# Récupérer des données / partitions sur un disque dur

Lister les partitions (y compris celles qui n'apparaissent pas dans le système de fichier):

	$ fdisk -l /dev/sdb


Récupérer des partitions:

	$ sudo apt install testdisk
	$ sudo testdisk

Récupérer des fichiers:

	$ sudo photorec

