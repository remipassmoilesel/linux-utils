# Installer un serveur Lighttpd et PHP sur OpenWRT

Se connecter au routeur:

	$ ssh root@192.168.0.49

## Libérer le port 80

Modifier le port de connexion de Luci pour libéer 80:

	$ vim /etc/config/uhttpd

	list listen_http [0.0.0.0:85]

	$ /etc/init.d/uhttpd restart

## Installation

Installer lighttpd et PHP:

	$ opkg install lighttpd php5 php5-cgi lighttpd-mod-cgi 

Les modules sont disponibles sous cette forme:

	$ opkg install php5-mod-openssl php5-mod-session

## Configuration

Configurer lighttpd pour PHP:

	$ vim /etc/lighttpd/conf.d/30-cgi.conf

	cgi.assign = ( ".php"  => "/usr/bin/php-cgi" )

Configurer PHP:

	$ vim /etc/php.ini
	
	upload_max_filesize = 150M
	post_max_size = 150M

Si l'erreur 'No input file.' apparait:
	
	$ vim /etc/php/php.ini

	doc_root = "/www/lighttpd"
	cgi.fix_pathinfo = 1

Créer un dossier ou servir les fichiers:

	$ mkdir /www/lighttpd

Ecouter le port 80 dans le dossier /www/lighttpd:

	$ vim /etc/lighttpd/lighttpd.conf

	dir-listing.activate 	= "enable"
	server.document-root    = "/www/lighttpd"
	server.port             = "80"

Droits en lecture / écriture:

	$ chown -R http:www-data /www/lighttpd

Activation + redémarrage:

	$ /etc/init.d/uhttpd restart
	$ /etc/init.d/lighttpd enable
	$ /etc/init.d/lighttpd restart

	$ reboot

## Debuggage

Pour debugger le traffic:

	$ vim /etc/lighttpd/lighttpd.conf

	server.errorlog             = "/var/log/lighttpd/error.log"
	debug.log-request-handling = "enable" 

Pour surveillerlighttpd utiliser mod_status:

	$ opkg install lighttpd-mod-status

	$ vim /etc/lighttpd/lighttpd.conf
	
	status.status-url = "/server-status"

	>> Visiter ensuite /server-status

Pour debugger PHP:

	$ vim /etc/php.ini

	error_reporting  =  E_ALL                                                                   
	display_errors = On                                                                        
	display_startup_errors = On                                                                
	log_errors = On                                                                            
	log_errors_max_len = 4096                                                                  
	report_memleaks = On                                                                       
	error_log = /var/log/php_errors.log                                                        

## Erreurs courantes

	Cannot move ... from /tmp/php

	>> $ df -h

