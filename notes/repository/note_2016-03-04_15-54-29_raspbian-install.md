# Installer Raspbian

Source: https://www.raspberrypi.org/documentation/installation/installing-images/linux.md

Repérer la partition cible avec df -h ou lsblk

Démonter la partition

	$ sudo umount /dev/xxx*

Se déplacer jusqu'au dossier contenant l'image à copier puis:

	$ sudo dd bs=4M if=2016-02-09-raspbian-jessie.img of=/dev/xxx

Si une erreur survient, remplacer 4M par 1M

Ensuite connexion ssh disponible:

	$ ssh pi@adresse
	# pw: raspberry


