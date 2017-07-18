# Installer un environnement de développement CPOS / Docker sur Ubuntu 17.04

Commande utiles: 

	$ npm run docker-restart-services # Redémarrer les services plus rapidement
	$ npm run docker-remove-all
	$ npm run docker-build-images
	$ npm run docker-clean-rebuild
	$ npm run docker-install
	$ npm run docker-install-module
	$ npm run docker-start
	$ npm run docker-stop
	$ npm run docker-test

Adresses intérressantes:

	(voir aussi docker ps, colonne name)
	http://library-api.docker
	http://job-admin.docker 	bee : poipoi


Documentation disponible: 
	
	- http://10.0.0.79/cpos/cpos/blob/master/docs/installation/docker-local.md

Au préalable:
	
	- Installer docker et docker-compose

	- /!\ Penser à vérifier qu'aucun firewall ne gêne pas (DNS par exemple)
	
	- Se connecter à gitlab.bbuzcloud.com via ssh pour l'ajouter à known_hosts ou:
		$ ssh-keyscan gitlab.bbuzcloud.com >> ~/.ssh/known_hosts

	- Avoir une clef ./ssh/id_rsa SANS PASSPHRASE

	- Vérifier la version du kernel (mongo 2.4 peut ne pas fonctionner avec un kernel > 4.9)

Cloner le projet:

	$ cd /home/remipassmoilesel/projects/bee-buzziness
	$ git clone git@gitlab.bbuzcloud.com:cpos/cpos.git
	$ cd cpos
	$ npm install

Peut nécéssiter:

	$ sudo apt install build-essential

Créer un dossier pour les données:

	$ mkdir /home/remipassmoilesel/projects/bee-buzzinnes/cpos-data

Créer un fichier de variables d'environnement:

	$ cp cpos/
	$ cp docker/local/composefiles/.env.template docker/local/composefiles/.env
	$ vim docker/local/composefiles/.env

Exemple:
	
	CPOS_DIR=/home/remipassmoilesel/projects/cpos
	DATA_DIR=/home/remipassmoilesel/projects/cpos-data
	MONGO_LOCAL_PORT=27017
	STUDIO_DOMAIN=studio.docker
	BEE_DOCKER_REGISTRY=beelab-repositories.bbuzcloud.com:5000
	COMPOSE_HTTP_TIMEOUT=60
	MY_IP_ADD=10.0.1.138
	EXTERNAL_DNS_1=10.0.0.1
	EXTERNAL_DNS_2=10.0.0.2
	OS=linux
	CA_CERTIFICATES=/usr/local/share/ca-certificates/bbuzg.crt
	PRIVATE_KEY_PATH=~/.ssh/id_rsa
	KNOWN_HOSTS_PATH=~/.ssh/known_hosts
			
Créer un réseau de conteneurs:

	$ docker network create cpos_local_network

Configurer la résolution DNS de docker:

	$ vim /etc/docker/daemon.json

	{
	    "insecure-registries": ["beelab-repositories.bbuzcloud.com:5000"],
	    "dns": ["10.0.0.2", "10.0.0.1", "8.8.8.8", "8.8.4.4"]
	}


Installer et configurer DnsMasq (remplacer x.x.x.x):

	$ sudo apt-get install -y dnsmasq
	$ sudo vim /etc/dnsmasq.conf

	conf-dir=/etc/dnsmasq.d/
	server=10.0.0.1
	server=10.0.0.2
	no-resolv
	cache-size=0
	no-negcache

	$ sudo vim /etc/resolv.conf
	
	nameserver 127.0.0.1 # conserver seulement cette ligne

	$ vim /etc/dnsmasq.d/docker
	
	address=/docker/x.x.x.x
	
	$ service dnsmasq restart && service dnsmasq status

Tester si la configuration fonctionne:

	$ ping nimportequoi.docker

Installer les dépendances dans un volume Docker:
	
	$ npm run docker-install

Si le processus bloque, vérifier si il n'y a pas un prompt de confirmation d'hôte 
(The authenticity of host 'gitlab.bbuzcloud.com (192.168.0.79)' can't be established...).
Vérifier également qu'il n'y ai pas de passphrase sur la clef ssh principale.

Construire les images:

	$ npm run docker-build-images

Si le message "still trying to connect to running keycloak container" apparait, attendre.
Si une erreur "port in use" apparait, tuer les conteneurs en marche.

Récupération des données des profils:

	$ npm run docker-fetch-profile-data [dev | preprod | prod]

Attention: le profil prod peut enclencher des actions externes (impressions, etc...)

Premier lancement ou nettoyage des données:

	$ npm run docker-clean-rebuild

En cas d'erreur d'accès à une adresse d'un autre réseau, créer une interface virtuelle:

	$ sudo vim /etc/network/interfaces

	auto wlp3s0:0
	allow-hotplug wlp3s0:0
	iface wlp3s0:0 inet static
	address 192.168.2.32
	netmask 255.255.248.0

	$ sudo service networking restart

Télécharger les loaders:

	$ npm run docker-update-loaders

Construire les build-infos:

	$ npm run docker-build-info

Lancement et arrêt:

	$ npm run docker-start
	$ npm run docker-stop
	$ npm run docker-restart-services

Nettoyer les données au préalable:

	$ npm run docker-clean-rebuild


