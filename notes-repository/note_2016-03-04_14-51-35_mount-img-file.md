# Monter un fichier .img

Identifier les partitions:

	$ fdisk -l /path/to/file.img

	Exemple:

	Disque 2016-02-26-raspbian-jessie-lite.img : 1,3 GiB, 1361051648 octets, 2658304 secteurs
	Unités : sectors of 1 * 512 = 512 octets
	Sector size (logical/physical): 512 bytes / 512 bytes
	I/O size (minimum/optimal): 512 bytes / 512 bytes
	Disklabel type: dos
	Disk identifier: 0xe6a544c8

	Périphérique                         Amorçage  Start     Fin Secteurs  Size Id Type
	2016-02-26-raspbian-jessie-lite.img1            8192  131071   122880   60M  c W95 FAT32 (LBA)
	2016-02-26-raspbian-jessie-lite.img2          131072 2658303  2527232  1,2G 83 Linux

Exemple pour le montage de la partition *.img2

	# Calculer l'offset: 512 (sector size) * 131072 (start) puis
	$ mount -o loop,offset=$((512*131072)) file.img /mnt/tmp


