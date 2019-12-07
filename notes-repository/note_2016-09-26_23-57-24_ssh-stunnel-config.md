# Utiliser SSH a travers SSL avec stunnel

Fonctionne avec d'autres protocoles.

## Coté serveur:

Installer stunnel4:

	$ sudo apt-get install stunnel4

Créer une clef et un certificat autosigné:

	$ openssl genrsa 4096 > stunnel.key
	$ openssl req -new -key stunnel.key -x509 -days 1000 -out stunnel.crt
	$ cat stunnel.crt stunnel.key > stunnel.pem
	$ sudo mv stunnel.pem /etc/stunnel/

Activer stunnel4:

	$ sudo vim /etc/default/stunnel4

	# changer 0 en 1
	ENABLED=1

Créer une configuration pour ssh, avec adresse ip publique ('public_ip'):

	$ sudo vim /etc/stunnel4/stunnel-ssh.conf

	pid = /var/run/stunnel.pid
	cert = /etc/stunnel/stunnel.pem
	
	[ssh] 
	accept = public_ip:443
	connect = 127.0.0.1:22

Démarrer le service:

	$ sudo service stunnel4 start

## Coté client

Installer stunnel4 comme décrit ci-dessus

Activer stunnel comme décrit ci-dessus

Créer une configuration pour le serveur avec adresse ip publique ('remote_ip'):

	$ sudo vim /etc/stunnel4/server.conf

	pid = /var/run/stunnel.pid
	cert = /etc/stunnel/stunnel.pem

	client = yes

	[ssh] 
	accept = 127.0.0.1:2200
	connect = remote_ip:443

