# Mettre en place un réseau wifi et un point d'accès avec hostapd

Essayé sur Raspbian, mais possible sur toutes dérivées Debian.

## Vérifier le matériel

Vérifier si le périphérique Wifi est reconnu:

	$ dmesg | tail

	... Product: 802.11n WIFI ...

Vérifier si le périphérique WIFI supporte l'Access Point:

	$ iw list 

	Supported interface mode:
		...
		AP
		...
	
## Installer les logiciels

Installer le serveur DHCP, le serveur de point d'accés et un utilitaire pour conserver les règles de NAT:

	$ sudo apt-get install isc-dhcp-server hostapd iptables-persistent

## Configurer DHCP

	$ sudo vim /etc/dhcp/dhcpd.conf

	# Commenter:
	option domain-name "example.org";
	option domain-name-servers ns1.example.org, ns2.example.org;

	# Décommenter:
	#authoritative

	# Ajouter tout en bas du fichier (corriger la plage d'adresses si necessaire)
	subnet 192.168.42.0 netmask 255.255.255.0 {
    		range 192.168.42.10 192.168.42.50;
	    	option broadcast-address 192.168.42.255;
	    	option routers 192.168.42.1;
	    	default-lease-time 600;
	    	max-lease-time 7200;
	    	option domain-name "local";
	    	option domain-name-servers 8.8.8.8, 8.8.4.4;
	}

Activer DHCP sur wlan0:

	$ sudo vim /etc/default/isc-dhcp-server

	INTERFACES="wlan0"

Attribuer une adresse fixe au périphérique WIFI:

	$ sudo ifdown wlan0
	$ sudo vim /etc/network/interfaces

	# auto wlan0
	allow-hotplug wlan0
	iface wlan0 inet static
        	address 192.168.42.1
	        netmask 255.255.255.0

	$ sudo ifconfig wlan0 192.168.42.1

## Configurer le point d'accés

Créer un fichier de configuration hostapd. Un exemple détaillé peut être copié dans /usr/share/doc/hostapd/examples/hostapd.conf.gz

	$ sudo vim /etc/hostapd/hostapd.conf

	interface=wlan0
	#driver=rtl871xdrv 		# Spécifier le nom du driver si problème
	ssid=Pi_AP			# Nom du réseau
	country_code=FR		
	hw_mode=g
	channel=7
	macaddr_acl=0
	auth_algs=1			# WPA seulement
	ignore_broadcast_ssid=0		# Diffuser le nom du réseau
	wpa=2
	wpa_passphrase=Raspberry	# mot de passe du réseau
	wpa_key_mgmt=WPA-PSK
	wpa_pairwise=CCMP
	wpa_group_rekey=86400
	ieee80211n=1
	wme_enabled=1

/!\ Le 'channel' utilisé doit être adapté au pays où est employé le matériel (pour ne pas gêner éventuellement d'autres transmissions)

Activer la configuration:

	$ sudo vim /etc/default/hostapd

	DAEMON_CONF="/etc/hostapd/hostapd.conf"

A ce stade les périphériques doivent pouvoir se connecter au réseau mais n'ont pas accès à internet.
Le routeur est disponible sur 192.168.42.1 .

## Configurer le NAT

Activer le NAT au prochain démarrage:

	$ sudo vim /ets/sysctl.conf
	
	# Ajouter à la fin 
	net.ipv4.ip_forward=1

Activer tout de suite:

	$ sudo sh -c "echo 1 > /proc/sys/net/ipv4/ip_forward"

Activer les régles de NAT pour faire suivre les paquets entre eth0 et wlan0:

	$ sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
	$ sudo iptables -A FORWARD -i eth0 -o wlan0 -m state --state RELATED,ESTABLISHED -j ACCEPT
	$ sudo iptables -A FORWARD -i wlan0 -o eth0 -j ACCEPT

Sauvegarder les régles pour qu'elles puissent être réinstantiées par iptables-persistent au prochain démarrage:

	$ sudo sh -c "iptables-save > /etc/iptables.ipv4.nat"

Sources:

https://seravo.fi/2014/create-wireless-access-point-hostapd
https://wiki.archlinux.org/index.php/software_access_point
https://learn.adafruit.com/setting-up-a-raspberry-pi-as-a-wifi-access-point/install-software


