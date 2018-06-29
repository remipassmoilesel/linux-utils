# Mémo réseau GNU/Linux

## Débugger une mauvaise connexion réseau

	- essayer un ping
	- essayer dig ou host
	- essayer traceroute
	- regarder si une route existe
	- vérifier que les interfaces soit "up"
	- vérifier /etc/hosts /etc/network/interfaces /etc/resolv.conf

## CIDR / Masques

	CIDR		Min		Max
	10.0.0.1/8	10.0.0.0	10.255.255.254
	10.0.0.1/16	10.0.0.0	10.0.255.254
	10.0.0.1/24	10.0.0.0	10.0.0.255

	192.168.0.1/24	192.168.0.1	192.168.0.254
	
## Commandes courantes

Afficher des informations sur les interfaces réseaux:

	$ ip a

(Adresse MAC dans la section link/ether)

## Manipuler des routes avec l'outil ip

Voir: https://access.redhat.com/sites/default/files/attachments/rh_ip_command_cheatsheet_1214_jcs_print.pdf

Afficher les routes:

	$ ip route show

Ajoute une route par défaut:
	
	$ ip route add default via 192.168.1.254

Ajouter une route vers un réseau, à travers l'interface eth0: 

	$ ip route add 192.168.1.0/24 dev eth0

Supprimer une route:

	$ ip route delete 192.168.1.0/24 dev eth0

Activer une interface:

	$ ip link set eth0 up

Désactiver une interface: 
	
	$ ip link set eth0 down

Ne pas hésiter à se servir d'un calculateur de masque comme:

	http://www.hobbesworld.com/reseaux/calcip.php
	http://www.subnet-calculator.com

## Ajouter une route vers une machine virtuelle

Utile par exemple pour DCOS-Vagrant. Afficher la route:

	$ ip a 					# afficher l'adresse de l'interface pour calculer l'id reseau et masque

	$ sudo ip link set dev vboxnet0 up 	                    # verifier que l'interface est up
	$ sudo ip route add 192.168.65.0/24 dev vboxnet0	    # enregistrer la route
	$ ip route show				                            # verifier que tout est ok

	$ ping vm.domain


