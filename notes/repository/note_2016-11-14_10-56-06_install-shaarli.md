# Installer Shaarli

Nécéssite un serveur Apache fonctionnel.

Téléchargement et installation:

	$ cd /var/www
	$ wget https://github.com/shaarli/Shaarli/releases/download/v0.8.0/shaarli-v0.8.0-full.zip  
	$ unzip Shaarli
	$ cd Shaarli
	$ sudo chown -R www-data:www-data . 

Dépendances Apache:

	$ sudo apt-get install openssl php7.0-mbstring php7.0-gd php7.0-intl php7.0-curl

Configuration:
	
	$ vim data/config.json.php

Installation d'un thème (peu de thèmes satisfaisants):

	$ cd shaarli/tpl
	$ git clone https://github.com/alexisju/albinomouse-template.git
	$ cd ../data
	$ sud vim config.json.php

	"raintpl_tpl": "tpl\/albinomouse-template\/",

Vider les raccourcis:

	$ sudo rm data/datastore.php
