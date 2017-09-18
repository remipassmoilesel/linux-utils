# Construire une image OpenWRT avec l'image builder

Source: https://wiki.openwrt.org/doc/howto/obtain.firmware.generate

Cas du TL-MR3020 pour créer une image avec des paquets supplémentaires.

Installer les dépendances:

	$ apt-get install subversion build-essential libncurses5-dev zlib1g-dev gawk git ccache gettext libssl-dev xsltproc wget

Télécharger l'image builder OpenWrt Chaos Calmer approprié:

	$ cd ~/playground
	$ wget https://downloads.openwrt.org/chaos_calmer/15.05.1/ar71xx/generic/OpenWrt-ImageBuilder-15.05.1-ar71xx-generic.Linux-x86_64.tar.bz2
	$ tar -xf OpenWrt-ImageBuilder-15.05.1-ar71xx-generic.Linux-x86_64.tar.bz2

Avant de commencer une nouvelle image il peut etre utile de lister les paquets de l'image d'origine. Sur une image OpenWRT en fonction lister les paquets présents:

	$ echo $(opkg list_installed | awk '{ print $1 }') 

Exemple d'une image minimale TLMR3020 pour mettre en place un extroot:

	$ make image PROFILE=TLMR3020 PACKAGES="blkid block-mount kmod-fs-ext4 kmod-usb2 kmod-usb-uhci kmod-usb-ohci kmod-usb-storage"
			
Pour supprimer des paquets, faire précéder le nom du paquet par un "-".

Avant chaque build un clean est effectué automatiquement.

Dans le cas du TLMR3020 l'image à utiliser est bin/ar71xx/openwrt-15.05.1-ar71xx-generic-tl-mr3020-v1-squashfs-factory.bin 
Dans certains cas ce fichier n'est pas généré. Hypothése: si trop de paquets sont choisis, et que l'image finale dépasse 4M 
alors le fichier final n'est pas généré.

Commandes utiles:

	$ make help	# aide 
	$ make info	# afficher toutes les cibles dispo
	$ make clean	# effacer les fichiers temporaires
