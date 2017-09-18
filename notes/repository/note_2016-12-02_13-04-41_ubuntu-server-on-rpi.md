# Installer Ubuntu server sur Raspberry pi

Télécharger et décompresser Ubuntu server:

	$ wget http://cdimage.ubuntu.com/ubuntu/releases/16.04/release/ubuntu-16.04-preinstalled-server-armhf+raspi2.img.xz
	$ unxz ubuntu-16.04-preinstalled-server-armhf+raspi2.img.xz
	$ mv ubuntu.....xz ubuntu-rpi.xz

Ecrire l'image sur disque:

	$ sudo dd if=ubuntu-rpi.xz of=/dev/mmcblk0 bs=4M && sync
	$ sudo umount /dev/mmcblk*

Démarrer ensuite le RPI et se connecter en SSH (après quelques minutes):

	$ ssh ubuntu@rpi_ip_adress
	Password: ubuntu

Si problèmes de locale:

	$ sudo apt-get install --reinstall locales && sudo dpkg-reconfigure locales
        $ locale-gen fr_FR.UTF-8
        $ locale-gen --no-purge --lang fr_FR.UTF-8

En cas de problème "sudo: unable to resolve host ubuntu":

	$ sudo vim /etc/hosts

	127.0.0.1 	ubuntu
	
Installer le Wifi:

	$ cd /opt
	$ git clone https://github.com/pvaret/rtl8192cu-fixes
	$ sudo dkms add ./rtl8192cu-fixes
	$ sudo dkms install 8192cu/1.10
	$ sudo depmod -a
	$ sudo cp ./rtl8192cu-fixes/blacklist-native-rtl8192.conf /etc/modprobe.d/

## Déplacer la partition principale sur une clef USB

Insérer la clef dans le RPI et créer une nouvelle partition ext4:

	$ sudo fdisk /dev/sda
	
		d
		n
		w

	$ sudo mkfs.ext4 /dev/sda1
	$ sudo mkdir -p /mnt/sda1
	$ sudo mount /dev/sda1 /mnt/sda1

Copier le système de fichier sur la clef:

	$ sudo rsync -avz --exclude '/mnt' / /mnt/sda1

Chrooter dans le nouveau système:

	$ cd /mnt
	$ sudo mount --bind /dev sda1/dev/
	$ sudo mount --bind /proc sda1/proc/
	$ sudo mount --bind /sys sda1/sys/
	$ sudo chroot sda1

	# La suite se déroule dans le chroot
	$ sudo blkid
	$ sudo vim /etc/fstab
		# modifier la ligne / avec l'UUID de la clef usb précedemment listé
		UUID=41f4fb13-d116-4108-b019-030f86bdcb0e       /               ext4    defaults        0       0
	$ update-grub-legacy-ec2
	$ exit	

Mettre à jour le firmware:
	
	$ cd /boot/firmware
	$ sudo cp cmdline.txt cmdline.txt.bak

	# Remplacer 
	root=/dev/mmcblk0p2 
	root=/dev/sda1

Redémarrer (le redémarrage peut prendre du temps):

	$ sudo reboot now
