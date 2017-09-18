# Installer Piwik, un outil de stats libre

Nécessite un serveur Apache, PHP et MariaDB:

    $ sudo apt-get install apache2 php7.0 mariadb-server php7.0-mysql libapache2-mod-php7.0  
    $ sudo apt-get install php7.0-curl php7.0-gd php7.0-cli php-geoip php7.0-mbstring php-xml
    $ sudo service apache2 restart

Créer ensuite une BDD:

    $ sudo mysql -u root
    $ CREATE DATABASE piwik_db;
    $ CREATE USER 'piwik'@'localhost' IDENTIFIED BY 'password';
    $ GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, ALTER, CREATE TEMPORARY TABLES, LOCK TABLES ON piwik_db.* TO 'piwik'@'localhost';

Télécharger et décompresser Piwik:

    $ cd /var/www
    $ wget https://builds.piwik.org/piwik.zip && unzip piwik.zip  
    $ sudo chown -R www-data:www-data piwik

Puis se rendre avec son navigateur à l'adresse du dossier Piwik.
Lorsque Piwik demandera le login/mdp du superutilisateur, utiliser des informations arbitraires (pour l'adminsitration de Piwik uniquement)

/!\ Piwik effectue des requêtes vers l'extérieur, et n'aime pas les bloqueurs de pub.

Récupérer un mot de passe perdu:

	> UPDATE `piwik_user` SET `password` = MD5( 'changeMe' ), `token_auth` = MD5( CONCAT('admin', password)) WHERE `login` = 'admin' AND superuser_access = 1 

## Mise à jour

Lors d'une mise à jour automatique, une migration de base de données peut être nécéssaire.

	# Bloquer temporairement le trafic	
	$ sudo vim /var/www/piwik/config/config.ini.php
		
		[Tracker]
		record_statistics = 0

		[General]
		maintenance_mode = 1

	$ sudo chown www-data:www-data /var/www/piwik/config/config.ini.php

	# Effectuer la migration
	$ sudo -u www-data bash -c "php /var/www/piwik/console core:update" 
	
	# Enlever la configuration insérée ci-dessus
