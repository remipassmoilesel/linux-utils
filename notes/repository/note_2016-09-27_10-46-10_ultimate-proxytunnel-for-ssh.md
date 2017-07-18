# Utiliser SSH à travers HTTP(S), version ultime de la mort qui tue

## Coté client

Installer proxytunnel:

	$ sudo apt-get install proxytunnel

Créer une configuration `.ssh/config`:

	$ sudo vim ~/.ssh/config
	
	Host vps
		Hostname vps.net
		Port 443

	Host vps.proxy443
		Hostname  vps.net
		ProxyCommand /usr/bin/proxytunnel -v -E -p vps.net:443 -d localhost:10022
		ServerAliveInterval 30

	Host vps.proxy80
		Hostname  vps.net
		ProxyCommand /usr/bin/proxytunnel -v -p vps.net:80 -d localhost:10022
		ServerAliveInterval 30

	Host vps.proxy8080
		Hostname  vps.net
		ProxyCommand /usr/bin/proxytunnel -v -E -p vps.net:8080 -d localhost:10022
		ServerAliveInterval 30

Détail:

	Host:		Alias de domaine de connexion
	Hostname: 	Veritable domaine de connexion
	ProxyCommand:	Commande proxy	

	proxitunel:
		/usr/bin/proxytunnel	chemin absolu de l'utilitaire proxytunnel
		-v			verbose
		-p 			domaine et port du proxy (vps)
		-E			Utiliser HTTPS
		-d			destination finale, après le proxy

	ServerAliveInterval 	Delai en secondes avant que client n'envoie un paquet null
	  			Permet d'éviter des coupures

## Coté serveur

Installer OpenSSH:

	$ sudo apt-get install openssh-server

Ecouter le port 10022:

	$ sudo vim /etc/ssh/sshd_config

	Port 10022

Installer Apache et activer les modules nécessaires:

	$ sudo apt-get install apache2
	$ sudo a2enmod proxy_http proxy_connect ssl
	$ sudo service apache2 restart

Créer une configuration, exemple pour un VPS:

	$ sudo vim /etc/apache2/sites-available /etc/apache2/sites-available/000-vps.conf
	
	# Attention, la configuration doit être la première à être interprétée par Apache
	# vérifier avec apachectl -S

	# Il est toujours possible de servir des fichiers, ou de faire proxy avec ProxyPass
	
	# Connexion sécurisée sur un hote
	# Utilise des certificats 
	<VirtualHost *:443>

	        ServerName vps.net
	        ServerAlias 137.##.##.##

	        DocumentRoot /var/www/

	        LogLevel alert
	        #LogLevel trace8

	        ErrorLog ${APACHE_LOG_DIR}/error.log
	        CustomLog ${APACHE_LOG_DIR}/access.log combined
	
		# Certificats TLS
	        SSLEngine on
	        SSLCertificateFile      /etc/ssl/vps.crt
	        SSLCertificateKeyFile   /etc/ssl/vps.key

	        # Proxy SSH and others
	        ProxyVia Full
	        ProxyRequests On
	        ProxyBadHeader Ignore
	        AllowCONNECT 10022
	
	</VirtualHost>

	# Connexion 'non sécurisée', en tout cas pas au niveau du proxy
	# Mais le protocole en transit est déjà sécurisé (SSH)
	# Certains pare-feux peuvent bloquer le trafic sécurisé sur 80
	<VirtualHost *:80>

	        ServerName vps.net
	        ServerAlias 137.##.##.##

	        DocumentRoot /var/www/empty

	        # SSH proxy
	        ProxyVia Full
	        ProxyRequests On
	        ProxyBadHeader Ignore
	        AllowCONNECT 10022

	</VirtualHost>


Vérifier la configuration et redémarrer Apache:
	
	$ sudo a2ensite vps.conf
	$ apachectl -S
	$ sudo service apache2 restart

Connexion:

	# HTTPS
	$ ssh remipassmoilesel@vps.proxy443

	# HTTP
	$ ssh remipassmoilesel@vps.proxy80

## Sécuriser le serveur 

Pour éviter que d'autres utilisent le serveur pour accéder à des sites anonymement, et accessoirement pour ne pas se faire bannr de freenode,
interdire la connexion par proxy ailleurs qu'en direction de localhost.

        ProxyVia On
        ProxyRequests On
        ProxyPreserveHost On

        <Proxy "*">
                Deny from all
        </Proxy>

        <ProxyMatch "localhost.*">
                Allow from all
        </ProxyMatch>

        AllowCONNECT 1443 10022

 
Pour tester si ça fonctionne:

	$ export http_proxy=http://vps303506.ovh.net:80
	$ curl -v http://google.com  # ne doit pas fonctionner













