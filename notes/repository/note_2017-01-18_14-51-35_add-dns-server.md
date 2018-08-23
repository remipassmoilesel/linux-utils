# Ajouter un serveur DNS

Serveurs de la FDN:

	ns0.fdn.fr : 80.67.169.12 ou 2001:910:800::12
	ns1.fdn.fr : 80.67.169.40 ou 2001:910:800::40

Serveurs Open DNS (à utiliser avec attention):

	208.67.222.222
	208.67.220.220
	208.67.222.220
	208.67.220.222

Liste de serveurs DNS:

	$ curl -s http://public-dns.info/nameserver/br.csv

Pour tester un serveur:

	$ dig @80.67.169.12 domain.org
	# marche aussi avec 'drill'

Afficher le serveur utilisé:

	$ nmcli device 
	$ nmcli device show enp4s0f2 | grep -i dns 

## Sans resolvconf

Dans le cas ou "resolvconf" n'est pas installé, éditer le fichier:
	
	$ sudo vim /etc/resolv.conf
	
	nameserver 80.67.169.12

## Avec resolvonf

Technique de sioux, rapide mais peu orthodoxe: 

	$ sudo vim /etc/resolvconf/resolv.conf.d/base
	
	nameserver 80.67.169.12

	$ sudo resolvconf -u
	$ cat /etc/resolv.conf

Alternative, dans le fichier d'interface:

	auto eth0
	iface eth0 inet static
		address 91.121.0.0
		netmask 255.255.255.0
		network 91.121.0.0
		broadcast 91.121.0.255
		gateway 91.121.0.254
		dns-nameservers 8.8.8.8 8.8.4.4

Alternative, dans dhclient:

	$ sudo vim /etc/dhcp/dhclient.conf

	prepend domain-name-servers 80.67.169.40;
	prepend domain-name-servers 80.67.169.12;

	# les serveurs seront employés dans l'ordre inverse

Il peut être nécessaire de redémarrer.
Pour tester quels serveurs sont utilisés:

	$ nmcli device show wlp3s0 | grep -i dns

## Avec systemd-resolved

Editer:

	$ vim /etc/systemd/resolved.conf
	
	[Resolve]
	DNS=4.2.2.4 4.2.2.2 209.244.0.4 209.244.0.3

	$ sudo systemctl restart systemd-resolved.service

## Erreurs

Le domaine est résolu mais ne peut pas être pingué:

	$ sudo vim /etc/nsswitch.conf

	hosts: files mdns4_minimal [NOTFOUND=return] dns mdns4
	
	Remplacer par:

	hosts:          files dns


