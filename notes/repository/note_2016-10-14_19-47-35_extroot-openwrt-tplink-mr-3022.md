# Etendre la mémoire du TPLINK MR3022 avec une clef USB et extroot

A partir d'une installation OpenWRT, si on veut étendre la mémoire avec une clef USB il faut installer les outils necessaires. Or, la mémoire étant très limitée, 
il faut installer d'abord une nouvelle image réduite d'OpenWRT.

D'abord, sur un poste Linux complet créer une image minimale d'OpenWRT:

	$ wget https://downloads.openwrt.org/chaos_calmer/15.05.1/ar71xx/generic/OpenWrt-ImageBuilder-15.05.1-ar71xx-generic.Linux-x86_64.tar.bz2
	$ cd OpenWrt-ImageBuilder-15.05.1-ar71xx-generic.Linux-x86_64   
	$ make image PROFILE=TLMR3020 PACKAGES="blkid block-mount kmod-fs-ext4 kmod-usb2 kmod-usb-uhci kmod-usb-ohci kmod-usb-storage"  

Mettre l'image à disposition avec Apache:

	$ cp bin/ar71xx/openwrt-15.05.1-ar71xx-generic-tl-mr3020-v1-squashfs-factory.bin /var/www/opwrt.bin

Puis se connecter au routeur:

	$ ssh root@192.168.0.49
	
Télécharger et installer l'image:

	$ wget http://192.168.0.49/opwrt.bin
	$ sysupgrade opwrt.bin

Le routeur redémarre, ne pas le débrancher.

Ensuite formatter la clef USB (par exemple avec une seule partition ext4) et la brancher au routeur.

Monter la clef USB:

	$ mkdir -p /mnt/sda1
	$ mount /dev/sda1 /mnt/sda1

Copier l'intégralité du dossier overlay vers la clef USB:

	$ tar -C /overlay/ -c . -f - | tar -C /mnt/sda1 -xf -

Configurer fstab:

	$ block detect > /etc/config/fstab	
	$ vim /etc/config/fstab

	Remplacer "/mnt/sda1" par "/overlay"
	Remplacer en dessous option enabled '0' par '1'

	Exemple de fichier final:

	config 'global'
        	option  anon_swap       '0'
	        option  anon_mount      '0'
	        option  auto_swap       '1'
	        option  auto_mount      '1'
	        option  delay_root      '5'
	        option  check_fs        '0'

	config 'mount'
	        option  target  '/overlay' 				# avant: '/mnt/sda1'
	        option  uuid    '4de293fe-4c4e-466d-87d7-a4338cf8ea0c'
	        option  enabled '1' 					# avant: '0'

Activer le service fstab:

	$ /etc/init.d/fstab enable

Redémarrer:

	$ reboot

Vérifier que tout fonctionne:

	$ df -h
	$ mount

Il faudra ensuite installer l'interface webadmin, ...

Source: https://fixmynix.com/configuring-extroot-with-openwrt-on-tp-link-mr-3220/
