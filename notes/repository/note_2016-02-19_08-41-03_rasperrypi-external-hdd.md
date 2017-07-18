# Déplacer le système de fichier d'un rpi sur un disque externe

Source: http://raspberrypihobbyist.blogspot.fr/2013/07/running-from-external-hard-drive.html

Mise à jour préliminaire:

	$ sudo raspi-config
	$ sudo apt-get update
	$ sudo apt-get upgrade -y
	$ git clone https://github.com/Hexxeh/rpi-update
	$ sudo rpi-update/pi-update
	$ sudo reboot


Créer une partition ext4 sur la clef USB
	
	$ sudo fdisk –l /dev/sda1
	$ sudo mkfs.ext4 /dev/sda1
	$ sudo mkdir -p /mnt/sda1
	$ sudo mount /dev/sda1 /mnt/sda1

Copier le système de fichier sur la nouvelle partition:

	$ sudo rsync -avz --exclude '/mnt' / /mnt/sda1

Mettre la nouvelle partition comme partition principale dans fstab:

	$ sudo blkid
	$ sudo vim /mnt/etc/fstab
	
	/dev/mmcblk0p2  /               ext4    defaults,noatime  0       1

Modifier le fichier de boot

	$ cp /boot/cmdline.txt /boot/cmdline.orig
	$ vi /boot/cmdline.txt 
	
	# Remplacer /dev/mmcblk0p2 par /dev/sda1

Puis redémarrer.

