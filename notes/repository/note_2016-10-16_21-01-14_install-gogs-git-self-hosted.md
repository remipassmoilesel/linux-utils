# Installer Gogs, outils d'auto-hébergement Git 

Suppose qu'un utilisateur 'git' existe. Cet utilisateur possédera tous les dépôts ainsi que les 
fichiers du seveur.

## Installation

Télécharger gogs:

	$ cd /opt
	$ wget https://dl.gogs.io/gogs_latest_linux_amd64.zip   
	$ unzip gogs_latest_linux.amd64

Tout les fichiers du serveur appartiennet à 'git':

	$ sudo chown -R git:git /opt/gogs

Créer une base de donnée:

	$ sudo mysql -u root
	> CREATE USER 'gogs'@'localhost' IDENTIFIED BY 'password';
	> CREATE DATABASE gogs;
	> GRANT ALL PRIVILEGES ON gogs.* TO 'gogs'@'localhost';

Lancer le serveur en tant qu'utilisateur 'git':

	$ sudo -iu git
	$ cd /opt/gogs web

Se connecter à l'URL http://server-domain:3000 pour configuration avec GUI.

## Proxy Apache TLS

Ajouter à la configuration Apache en cours:

	ProxyPass /gogs http://localhost:3000
        ProxyPassReverse /gogs http://localhost:3000

Ajouter ensuite à /opt/gogs/custom/conf/app.init

	[server]
	ROOT_URL = https://server-domain/gogs

Penser à activer le module proxy d'Apache.

## Configuration TLS sans proxy

Activer TLS dans gogs:

	$ sudo -iu git
	$ vim custom/conf/app.ini 

	[server]
	PROTOCOL = https
	ROOT_URL = https://vps303506.ovh.net/
	CERT_FILE = custom/https/vps-git.cert
	KEY_FILE = custom/https/vps-git.key

Générer des clefs:

	$ cd /opt/gogs/custom
	$ mkdir https && cd https
	$ openssl genrsa -out vps-git.key 4096 -sha256
	$ openssl req -new -key vps-git.key -out vps-git.cert
   	$ openssl x509 -req -days 365 -in vps-git.cert -signkey vps-git.key -out vps-git.cert.signed




