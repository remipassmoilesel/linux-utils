# Se connecter à un serveur distant SSH avec rebond SSH

Pour se connecter à un serveur C, à partir d'un serveur A et à travers un serveur B:

	A -> B -> C

	A: eth0=1.2.3.4
	B: eth0=1.2.3.5 eth1=192.16.0.1
	C: eth0=192.168.0.2

Se connecter à B à partir de A et enregistrer sa clef SSH.
Se connecter à C à partir de B et enregistrer sa clef SSH.

Puis ajouter au fichier ~/.ssh/config:

	Host centreon-poller
	        User root
	        Port 22
	        IdentityFile ~/.ssh/id_rsa
	        ProxyCommand ssh -v root@1.2.3.4.5 nc -q0 192.168.0.2 22

## En une ligne


	$ ssh -v -o "IdentityFile ~/.ssh/id_rsa" -o "ProxyCommand ssh -v -p 22 user@bastion nc -q0 172.16.0.25 22" user@172.16.0.25
