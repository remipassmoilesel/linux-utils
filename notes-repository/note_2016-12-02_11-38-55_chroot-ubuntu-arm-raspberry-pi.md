# Chrooter un système Ubuntu Mate ARM (Raspberry Pi)

Source:	https://wiki.ubuntu.com/ARM/BuildEABIChroot
	http://dev.lab427.net/rpi-raspbian-arm-chroot-on-x86-linux.html

Télécharger l'image:

	$ wget https://ubuntu-mate.org/raspberry-pi/ubuntu-mate-16.04-desktop-armhf-raspberry-pi.img.xz
	$ mv ubuntu-mate-16.04-desktop-armhf-raspberry-pi.img.xz ubuntu-rpi.img.xz
	$ unxz ubuntu-rpi.img

Monter l'image dans un dossier:

	$ fdisk -l ubuntu-rpi.img
	$ sudo mkdir /mnt/rpi
	$ sudo mount -o loop, offset=$((premier_block_partition*taille_block)) ubuntu-rpi.img /mnt/rpi

Installer Qemu static:

	$ sudo apt-get install qemu-user-static   

Préparer le chroot:

	$ cd /mnt
	$ sudo mount --bind /dev rpi/dev
	$ sudo mount --bind /proc rpi/proc
	$ sudo mount --bind /sys rpi/sys

	# Connexion internet
	$ sudo cp /etc/resolv.conf /rpi/etc/resolv.conf

	# Pour eviter les erreurs d'instruction QEMU, commenter la première ligne
	$ sudo vim rpi/etc/ld.so.preload

Chroot:

	$ sudo chroot rpi /bin/bash
	$ uname -m
	$ apt-get update

