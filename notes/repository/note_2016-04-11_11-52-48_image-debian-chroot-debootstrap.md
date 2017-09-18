# Créer une image Debian (fichier .img) avec chroot et debootstrap

Source: https://blog.tan-ce.com/chroot-ubuntu-14-04-on-android-nexus-10/

La procédure debootstrap est scindée en deux partie pour permettre par exemple
de créer des images d'une architecture différente (ex: armhf)

Cependant dans ce cas la il peut être plus judicieux d'utiliser "mk-sbuild"

Créer un fichier .img de 3.5G

	$ dd if=/dev/zero of=img.debian.root bs=8k count=458752
	$ sudo mkfs.ext4 -L debian -c img.debian.root
	$ sudo tune2fs -c 0 img.debian.root

Monter l'image:

	$ mkdir debian-root-fs
	$ sudo mount -o loop img.debian.root debian-root-fs

Installer le système de fichier:

	$ sudo debootstrap --arch armhf --foreign trusty chroot http://ports.ubuntu.com/
	$ sudo umount debian-root-fs

Sur le système cible:

	# Il peut être nécéssaire de monter certains dossier et définir des variables 
	# d'environnement.
	$ /debootstrap/debootstrap --second-stage


