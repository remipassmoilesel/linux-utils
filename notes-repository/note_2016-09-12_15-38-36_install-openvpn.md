# Installer et utiliser OpenVPN

## Utiliser OpenVPN Access Server

La méthode la plus simple. Sur Ubuntu 16.04 64 bits:

	(Voir page https://openvpn.net/index.php/download/58-open-source/downloads.html)

        $ wget http://swupdate.openvpn.org/as/openvpn-as-2.1.2-Ubuntu16.amd_64.deb
        $ sudo dpkg -i openvpn-as-2.1.2-Ubuntu16.amd_64.deb

Definir un mot de passe par défaut à l'utilisateur 'openvpn':

        $ sudo passwd openvpn
       
Ensuite visiter cnfigurer le serveur avec un navigateur web à l'adresse:

        https://137.74.161.106:943/admin # pour la configuration

Définir éventuellement d'autres utilisateurs.

**Par défaut l'interface admin est exposée sur toutes les interfaces**. Il est possible de
changer ce réglage dans l'interface admin.


## Ajouter un utilisateur

Dans un terminal, sur le serveur OpenVPN AS:

	$ sudo useradd username

Puis ajouter un utilisateur dans l'interface Admin:

	User Management > User permissions 


## Utiliser OpenVPN AS

Visiter l'interface d'administration:

	https://X.X.X.X:943/admin/

Visiter la page suivante puis télécharger le fichier de configuration:

        https://X.X.X.X:943/      # pour la connexion
        
Sur un poste client Linux:

	# Installer le client
	$ sudo apt-get install openvpn
	
	# Télécharger la configuration à l'aide d'un navigateur web. Nom du fichier: 'client.ovpn'
	# Puis démarrer le service:
	$ openvpn --config client.ovpn
	
Pour utiliser OpenVPN sur 443 installer sslh puis modifier la configuration .ovpn comme suit:

	-- /!\ ne fonctionne qu'avec TCP, pas avec UDP
	remote vps.net 443 tcp


## Ou utiliser et configurer OpenVPN

Configurer la génération de certificats:

	$ make-cadir ~/openvpn-ca 
	$ cd ~/openvpn-ca
	$ vim vars 

	export KEY_COUNTRY="US"
	export KEY_PROVINCE="CA"
	export KEY_CITY="Perlouette"
	export KEY_ORG="Fort-Funston"
	export KEY_EMAIL="me@myhost.mydomain"
	export KEY_OU="PerlouetteInternationalUnlimited"

	export KEY_NAME="server"

Générer les certificats:

	$ source vars
	$ ./clean-all
	$ ./build-ca

Générer la clef serveur (ne pas mettre de mot de passe):

	$ ./build-key-server server
	
Générer une clef Diffie-Hellman:

	$ ./build-dh

Générer une signature HMAC:

	$ openvpn --genkey --secret keys/ta.key

Copier les clefs:

	$ cp -R ~/openvpn-ca/keys /etc/openvpn/

Ajouter un fichier de configuration serveur:

	$ cp /usr/share/doc/openvpn/examples/sample-config-files/server.conf.gz /etc/openvpn/server.conf.gz
	$ gunzip /etc/openvpn/server.conf.gz

Ajuster la configuration:

	$ vim /etc/openvpn/server.conf
	
	tls-auth ta.key 0 	# décommenter en supprimant le ; de début
	key-direction 0		# ajouter
	
	cipher AES-128-CBC	# décommenter
	auth SHA256		# ajouter
	
	user nobody		# décommenter
	group nogroup		# décommenter

	push "redirect-gateway def1 bypass-dhcp"	# décommenter
	push "dhcp-option DNS 208.67.222.222"	
	push "dhcp-option DNS 208.67.220.220"

	dh /etc/openvpn/keys/dh2048.pem

	proto tcp

	ca /etc/openvpn/keys/ca.crt
	cert /etc/openvpn/keys/server.crt
	key /etc/openvpn/keys/server.key  # This file should be kept secret

Autoriser le passage de trafic:

	$ sudo vim /etc/sysctl.conf

	net.ipv4.ip_forward=1	# décommenter

	$ sudo sysctl -p

Démarrer le service: 
	
	sudo systemctl start openvpn@server 


## Configurer un client manuellement

Générer une clef avec passphrase:

	$ cd ~/openvpn-ca
	$ source vars
	$ ./build-key-pass client1

Créer un dossier:

	$ mkdir -p ~/client-configs/files
	$ chmod 700 ~/client-configs/files

Copier une config:

	$ cp /usr/share/doc/openvpn/examples/sample-config-files/client.conf ~/client-configs/base.conf

Modifier la config:

	$ vim client.conf
	
	remote server.domain.com 1194

	proto tcp

	user nobody
	group nogroup

	#ca ca.crt 		# indiquer ici le chemin des clefs
	#cert client.crt	# ou les spécifier à la fin du fichier de config
	#key client.key		# avec <tls-auth><key><cert>

	cipher AES-128-CBC
	auth SHA256
	key-direction 1

	# Ajouter ces lignes et les décommenter sur un client Linux avec fichier update-resolv-conf
	# script-security 2
	# up /etc/openvpn/update-resolv-conf
	# down /etc/openvpn/update-resolv-conf

Il est possible d'utiliser un script utilitaire:

	$ vim make_config.sh

	#!/bin/bash

	# First argument: Client identifier

	KEY_DIR=~/openvpn-ca/keys
	OUTPUT_DIR=~/client-configs/files
	BASE_CONFIG=~/client-configs/base.conf

	cat ${BASE_CONFIG} \
	    <(echo -e '<ca>') \
	    ${KEY_DIR}/ca.crt \
	    <(echo -e '</ca>\n<cert>') \
	    ${KEY_DIR}/${1}.crt \
	    <(echo -e '</cert>\n<key>') \
	    ${KEY_DIR}/${1}.key \
	    <(echo -e '</key>\n<tls-auth>') \
	    ${KEY_DIR}/ta.key \
	    <(echo -e '</tls-auth>') \
	    > ${OUTPUT_DIR}/${1}.ovpn

	$ ./make_config.sh client1


Source: https://www.digitalocean.com/community/tutorials/how-to-set-up-an-openvpn-server-on-ubuntu-16-04
