# Créer une machine virtuelle OpenWRT sur VirtualBox

Télécharger OpenWRT:

	$ wget https://downloads.openwrt.org/chaos_calmer/15.05/x86/generic/openwrt-15.05-x86-generic-combined-ext4.img.gz
	
Convertir l'image au format VirtualBox:

	$ VBoxManage convertfromraw --format VDI openwrt-15.05-x86-generic-combined-ext4.img openwrt.vdi

Puis créer une nouvelle machine dans VirtualBox, Linux 2.6 32 bits.

## Configuration réseau

Carte 1 : NAT
Carte 2 : Réseau interne vbox0

Si la connexion telnet ou le webadmin ne sont pas accessibles, une des solutions peut être de changer l'adresse IP par défaut.
Avec les paramètres de réseau VB par défaut ça donne:

	$ vi /etc/config/network

	config interface 'lan'
		option ipaddr '192.168.56.101'

	$ /etc/init.d/network restart

Pour obtenir l'adresse du réseau voir dans le GUI Virtualbox:

	Fichier > Préférences > Réseau > Réseau hôte uniquement > vboxnet0 > Parametres

