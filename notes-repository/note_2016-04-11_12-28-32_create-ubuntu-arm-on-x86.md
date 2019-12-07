# Créer une image Ubuntu Arm à partir d'un système Ubuntu d'une autre architecture

Installer les outils necessaires:

	$ sudo apt-get update; sudo apt-get install ubuntu-dev-tools

Créer un système ARM Ubuntu:

	# la première utilisation réclame l'installation de nouveaux paquets
	$ mk-sbuild --arch armhf wily

Entrer dans le nouveau système et le préparer:

	// 'sudo' permet d'entrer en tant que root dans le système
	# sudo schroot -c wily-armhf
	# apt-get update; apt-get install sudo openssh-server man
	# exit

Créer une image de 3.5G,  la monter et synchroniser le système:

	$ dd if=/dev/zero of=img.debian.root bs=8k count=458752
	$ sudo mkfs.ext4 -L UbuntuWilyArm -c ubuntuWilyArm.img
	$ sudo tune2fs -c 0 ubuntuWilyArm.img
	$ mkdir root-mount
	$ sudo mount -o loop ubuntuWilyArm.img root-mount
	$ sudo rsync -aAXv --numeric-ids --exclude={"/dev/*","/proc/*","/sys/*","/tmp/*","/run/*","/mnt/*","/media/*","/lost+found","/home/*"} /var/lib/schroot/chroots/wily-armhf/* .

