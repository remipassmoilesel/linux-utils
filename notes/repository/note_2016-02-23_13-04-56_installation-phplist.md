# Installer phplist dans un conteneur docker

Source: http://www.tecmint.com/phplist-open-source-email-newsletter-manager-mass-mailing-application-for-linux/

Installer les outils nécéssaires

    $ apt-get update
    $ apt-get -y install vim, wget, curl, unzip, tar
    $ apt-get -y install apache2
    $ apt-get -y install php5 libapache2-mod-auth-mysql php5-mysql php5-imap
    $ apt-get -y install mysql-server mysql-client

Démarrer les service et les activer au démarrage

    $ sudo service apache2 start
    $ sudo service mysql start

    $ sudo update-rc.d apache2 enable
    $ sudo update-rc.d mysql enable

Créer la base de donnée de phplist

    $ mysql -u root -p
    $ create database phplist;

Créer un utilisateur phplist

    $ grant all on phplist.* to phplist@localhost identified by 'azerty';
    $ flush privileges;
    $ quit;

Télécharger et installer phplist

    $ wget http://garr.dl.sourceforge.net/project/phplist/phplist/3.0.5/phplist-3.0.5.tgz

    $ tar -xvf phplist-3.0.5.tgz
    $ cd phplist-3.0.5
    $ cd public_html/
    $ cp -R lists /var/www/html/

Configurer phplist

    $ vim /var/www/html/lists/config.php

Modifier les sections suivantes

    # what is your Mysql database server hostname
    $database_host = "localhost";

	# what is the name of the database we are using
	$database_name = "phplist";

	# what user has access to this database
	$database_user = "phplist";

	# and what is the password to login to control the database
	$database_password = 'azerty';

	define ("TEST",0);

Quitter puis commiter le conteneur

    exit;
    docker ps -A
    docker commit containerId ubuntu-phplist

Relancer le conteneur en liant le port 80 du ctr au port 8080 de l'hote

    $ docker run -d -p 8080:80 ubuntu-arm-phplist   
    # ou 
    docker run -ti -p 8080:80 ubuntu-arm-phplist  

Ouvrir dans un navigateur

    http://ip-address/lists/admin
