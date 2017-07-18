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

Dans le cas ou "resolvconf" n'est pas installé:
	
	$ sudo vim /etc/resolv.conf
	
	nameserver 80.67.169.12

Avec resolvconf: 

	$ sudo vim /etc/resolvconf/resolv.conf.d/base
	
	nameserver 80.67.169.12

	$ sudo resolvconf -u
	$ cat /etc/resolv.conf

resolvconf tire ses informations de plusieurs sources, dont dhclient. Pour ajouter des 
serveurs avant ceux proposés par le serveur dhcp:

	$ sudo vim /etc/dhcp/dhclient.conf

	prepend domain-name-servers 80.67.169.40;
	prepend domain-name-servers 80.67.169.12;

	# les serveurs seront employés dans l'ordre inverse

Il peut être nécessaire de redémarrer.
Pour tester quels serveurs sont utilisés:

	$ nmcli device show wlp3s0 | grep -i dns
