# Ajouter un disque LVM sur Vsphere

## VSphere

Sur le client VSphere, ouvrir les paramètres de la machine:

- Ajouter un nouveau périphérique
- Sélectionner disque dur, la taille puis redémarrer la machine

## Création LVM via SSH sur la machine

Installer lvm2:

	$ sudo apt install lvm2
	$ systemctl start lvm2 start # Masked ?

Créer un volume physique:

	$ pvcreate /dev/sdb

Créer un groupe de volume:

	$ vgcreate docker /dev/sdb

Créer un volume logique:

	$ lvcreate -n docker01 -L 99G docker

Afficher les volumes dispos:

	$ lvdisplay
	$ lsblk

Formatter et monter le volume:

	$ mkfs -t ext4 /dev/docker/docker01
	$ mkdir /var/lib/registry
	$ mount /dev/docker/docker01 /var/lib/registry
	$ df -h

## Suppression

Suppression de groupe:

	$ vgremove volume-docker
