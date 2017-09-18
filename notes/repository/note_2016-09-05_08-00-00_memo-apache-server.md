# Memo pour Apache

Tester la configuration:

	$ apachectl configtest

Afficher les configurations chargées dans l'ordre:
	
	$ apachectl -S

Installation d'Apache2 avec PHP 7 et MariaDB:

	sudo apt-get update
	sudo apt-get install apache2
	sudo apt-get install php libapache2-mod-php
	sudo apt-get install mariadb-server
	
	
Servir un dossier avec Apache en ajoutant un alias:

	# Dans un fichier d'hôte virtuel ou dans httpd.conf
	Alias /www/ "/home/remipassmoilesel/projects/www/"
	Alias /www "/home/remipassmoilesel/projects/www"

	<Directory "/home/remipassmoilesel/projects/www">
		Options Indexes FollowSymLinks
		AllowOverride All
		Order allow,deny
		Allow from all
	</Directory>


Ajouter un hote virtuel:

	# Créer un fichier d'hôte virtuel
	$ sudo vim /etc/apache2/new-virtual-host.conf

	# Ajouter dans le fichier
	Listen 80
    <VirtualHost *:80>
        ServerAdmin admin@example.com
        ServerName example.com
        ServerAlias www.example.com
        DocumentRoot /var/www/example.com/public_html
        ErrorLog ${APACHE_LOG_DIR}/error.log
        CustomLog ${APACHE_LOG_DIR}/access.log combined
    </VirtualHost>

	# Activer la configuration
	$ sudo a2ensite example.com.conf
	$ sudo a2ensite test.com.conf

	# En cas d'erreur: ERROR: Site im.silverpeas.net does not exist!
	# C'est que le fichier ne se termine pas par ".conf"

	# En cas d'erreur de ce type, vérifier le fichier /etc/apache2/ports.conf
    # (98)Address already in use: AH00072: make_sock: could not bind to address 212.83.142.6:443
    

Mettre en place TLS(https) avec Apache pour développement

	# Créer une clef:

	$ cd /etc/ssl
	$ sudo openssl genrsa -out /etc/ssl/test-messagerie.ddns.net.key 4096 -sha512
	$ sudo chmod 400 test-messagerie.ddns.net.key
	$ sudo openssl req -new -key /etc/ssl/test-messagerie.ddns.net.key -out /etc/ssl/test-messagerie.ddns.net.csr

	# Ou en une commande
	$  openssl req -new -newkey rsa:4096 -days 365 -nodes -x509 \
		-subj "/C=AA/ST=BB/L=CC/O=DD/CN=EE" \
		-keyout djoe.key -out djoe.crt

	# Activer SSL, OpenSSL doit être installé
	$ sudo a2enmod ssl
	

Exemple de configuration apache avec redirection:

	Listen 80
	Listen 443

    # Redirection vers l'hote virtuel sécurisé
    <VirtualHost *:80>
        ServerName test-messagerie.ddns.net
        ServerAlias www.test-messagerie.ddns.net

        Options -Indexes

		# Attention au slash de fin !
        Redirect permanent / https://test-messagerie.ddns.net/
    </VirtualHost>


    # Hote virtuel sécurisé
    <VirtualHost *:443>

        SSLEngine on
        SSLCertificateFile /etc/ssl/test-messagerie.ddns.net.crt
        SSLCertificateKeyFile /etc/ssl/test-messagerie.ddns.net.key

        ServerAdmin remi.pace@silverpeas.org
        ServerName test-messagerie.ddns.net
        ServerAlias www.test-messagerie.ddns.net
        DocumentRoot /var/www/djoe/var.www.html/

        ErrorLog /var/www/djoe/log/apache-errors.log
        CustomLog /var/www/djoe/log/apache-access.log combined

    </VirtualHost>

	# Pour maximiser la compatibilité avec les navigateurs mobiles il faut en plus:
	
	Ajouter une référence vers le certificat intermédiaire dans le virtual host:
	    SSLCertificateChainFile  /etc/ssl/GandiStandardSSLCA2.pem
		SSLVerifyClient None

	Ajouter le certificat intermédiaire à la fin du certificat 
		-----BEGIN CERTIFICATE-----
		XXXXXXXXX   <- certificat intermédiaire
		-----END CERTIFICATE-----
		-----BEGIN CERTIFICATE-----
		XXXXXXXXX   <- certificat cross-signed 
		-----END CERTIFICATE-----

Hote virtuel par défaut:
	
	ServerAlias 136.251.XX.XX
	
Limiter les informations affichées par le serveur:

	$ sudo vim /etc/apache2/conf-available/security.conf  
	
	ServerSignature Off
	ServerTokens Prod

Rediriger toutes les requêtes http vers une page:

	RewriteEngine On

	RewriteCond %{REQUEST_URI} !^.*pre-https.*$
	RewriteCond %{REQUEST_URI} !^.*bower_components.*$
	RewriteRule ^ /pre-https.html [R=302]

Autoriser l'utilisation de .htaccess

	<Directory /var/www/>
		...
	        AllowOverride All
		...
	</Directory>


Commenter un block:

	<IfDefine IgnoreBlockComment>
	...
	</IfDefine>

