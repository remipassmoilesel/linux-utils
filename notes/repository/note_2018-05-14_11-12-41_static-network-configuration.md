# Configuration statique de réseau sur système Debian

Exemple:

	$ vim /etc/network/interfaces

	auto enp2s0
		allow-hotplug enp2s0
		iface enp2s0 inet static
		address 10.0.0.XXX
		netmask 255.255.XXX.XXX
		gateway 10.0.0.XXX
		dns-nameservers 127.0.0.1

Tip: pour empêcher Network Manager d'écraser /etc/resolv.conf

	$ sudo vim /etc/NetworkManager/NetworkManager.conf
	
	[main]
	...
	dns=none

