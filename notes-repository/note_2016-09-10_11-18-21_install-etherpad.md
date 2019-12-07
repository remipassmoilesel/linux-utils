# Installer Etherpad

Voir le playbook ansible du projet sysadmin-experiments

## Préparation

Télécharger et installer NodeJS:

    $ sudo apt install nodejs npm
    $ sudo npm install -g n
    $ sudo n latest
    $ sudo apt remove --purge nodejs npm

Cloner le dépôt Etherpad:

    $ git clone https://github.com/ether/etherpad-lite.git 

Créer une base de donnée postgres:

    $ sudo apt-get instll posgtresql
    $ sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';"  
    $ sudo -u postgres psql -c "create database etherpad"

Créer un utilisateur:

    $ sudo useradd --create-home etherpad

## Configuration

Configurer Etherpad:

    $ cd etherpad-lite
    $ cp settings.json.template settings.json

Ajouter un bloc de configuration pour postgres:

	"dbType" : "postgres",
	"dbSettings" : {
	    "user"    : "etherpad",
	    "host"    : "localhost",
	    "password": "password",
	    "database": "etherpad",
	    "charset" : "utf8mb4"
	}

Configurer l'interface et l'authentification:

	"ip": "127.0.0.1",
	"requireAuthentication": true,
	"trustProxy": true,
	"users": {
	   "username": {
	   "password": "password",
	       "is_admin": true
	   }
	},


Lancer Etherpad pour essai:

    $ sudo -iu etherpad
    $ bin/run.sh



Pour installer un proxy Apache HTTPS (utiliser de préférence un sous domaine):

	$ sudo a2enmod proxy proxy_http proxy_wstunnel
	$ vim /etc/apache2/sites-available/xxx.conf

	  ProxyVia On
	  ProxyRequests Off
	  ProxyPreserveHost On

	  # Etherpad
	  ProxyPass / http://localhost:9001/
	  ProxyPassReverse / http://localhost:9001/




