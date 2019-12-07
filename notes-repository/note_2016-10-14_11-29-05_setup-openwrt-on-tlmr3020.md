# Installer OpenWRT sur le TP Link TL-MR3020

Source: https://wiki.openwrt.org/toh/tp-link/tl-mr3020

Image d'origine du TL MR3020: 	http://www.tp-link.com/resources/gpl/150Router.rar
				http://www.tp-link.com/en/download/TL-MR3020_V1.html#Firmware


## Télécharger OpenWRT

Eventuellement se rendre sur la table de compatibilité pour vérifier le modèle:

	https://wiki.openwrt.org/toh/start

Télécharger l'image adaptée au modèle:

	$ wget http://downloads.openwrt.org/chaos_calmer/15.05/ar71xx/generic/openwrt-15.05-ar71xx-generic-tl-mr3020-v1-squashfs-factory.bin -O openwrt.bin

## Se connecter à l'interface admin Web

Connecter la première fois le périphérique directement au PC avec un cable droit.

Eventuellement assigner une adresse à l'interface réseau cablée:

	$ sudo ifconfig enp4s0f2 192.168.0.1 

Se rendre sur l'interface admin:

	http://192.168.0.254
	admin / admin

Puis dans System tools / Firmware Upgrade choisir le fichier téléchargé précédemment.
Attendre la fin de l'installation.

Essayer d'attendre le routeur:

	$ ping 192.168.1.1

Si pas de résultats, débrancher puis rebrancher le cable Ethernet du routeur:

	$ ping 192.168.1.1

Première configuration du mot de passe: 

	$ telnet 192.168.1.1
	$ passwd

Une fois le mot de passe changer, se déconnecter et utiliser SSH:

	$ ssh root@192.168.1.1

## Configuration en point d'accés LAN

Si lors du branchement du routeur MR3020 à un autre routeur l'appareil n'est pas disponible,
 changer son comportement:

	- en attribuant une adresse IP fixe valide
	- ou en le configurant comme client DHCP (adresse dynamique)

Aller sur l'interface Web:

	Network > Interfaces > Lan > Edit 

	Protocol: 
	- 'Static Adress' puis remplir une adresse valide
	- ou 'DHCP Client'

## Etendre la mémoire avec une clef USB

Voir la note dédiée.

## Installer l'interface Web SSL

	$ opkg update
	$ opkg install luci-ssl
	$ /etc/init.d/uhttpd enable
	$ /etc/init.d/uhttpd start

