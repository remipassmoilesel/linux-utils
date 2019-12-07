# Redimensionner une image de disque .img

Créer un périphérique virtuel:

	$ sudo modprobe loop

	# demander un nom de périph libre
	$ sudo losetup -f

	/dev/loop0

	$ sudo losetup /dev/loop0 myimage.img

Monter les différentes partitions de l'image:

	$ sudo partprobe /dev/loop0
	
	Par exemple:
	> /dev/loop0p1
	> /dev/loop0p2 

Redimensionner les partitions avec GParted:

	$ sudo gparted /dev/loop0

Ensuite démonter le périph virtuel:

	$ sudo losetup -d /dev/loop0

Puis réduire la taille de l'image avec truncate:

	$ fdisk -l myimage.img
	
	# Repérer le block de fin de la dernière partition, ajouter un puis * 512
	
	$ sudo truncate --size=$[(12984319+1)*512] ubuntu-mate-16.04-desktop-armhf-raspberry-pi.img   

