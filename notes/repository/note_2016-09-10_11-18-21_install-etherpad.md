# Installer Etherpad

Télécharger et installer NodeJS:

    $ cd /opt
    $ wget https://nodejs.org/dist/v4.5.0/node-v4.5.0-linux-x64.tar.xz
    $ mv node-v4... nodejs4

Cloner le dépôt Etherpad:

    $ git clone https://github.com/ether/etherpad-lite.git 

Créer une base de donnée postgres:

    $ sudo apt-get instll posgtresql
    $ sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';"  
    $ sudo -u postgres psql -c "create database etherpad"

Configurer Etherpad:

    $ cd etherpad-lite
    $ cp settings.json.template settings.json

Lancer Etherpad:

    $ bin/run.sh

Pour installer un proxy Apache HTTPS:

	$ sudo a2enmod proxy proxy_http proxy_wstunnel
	$ vim /etc/apache2/sites-available/xxx.conf

	  ProxyVia On
	  ProxyRequests Off
	  ProxyPreserveHost On

	  # Etherpad
	  ProxyPass /etherpad/p/ http://localhost:9001/p/
	  ProxyPassReverse /etherpad/p/ http://localhost:9001/p/

	  ProxyPass /etherpad/ http://localhost:9001/
	  ProxyPassReverse /etherpad/ http://localhost:9001/


