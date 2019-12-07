# Configuration d'OpenWRT avec le système UCI

Le système UCI permet de configurer OpenWRT plus simplement.

Tous les fichiers de configuration se trouvent dans le dossier /etc/config/

Pour changer l'adresse IP par exemple:

	$ ssh root@192.168.1.1
	$ uci set network.lan.ipaddr=192.168.0.49
	$ uci commit network

	ou

	$ uci commit
	$ /etc/init.d/network restart

'set' permet de modifier ou d'ajouter le paramètre
'commit' écrit les changements

Source: https://wiki.openwrt.org/doc/uci
