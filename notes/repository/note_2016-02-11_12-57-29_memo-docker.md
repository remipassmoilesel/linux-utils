# Memo sur le fonctionnement de docker

## Installation

Installation:

	/!\ Procédure changeante, voir sur le site de docker-ce

	...

	# s'ajouter au groupe docker pour se passer de sudo
	$ usermod -aG docker remipassmoilesel
	$ docker run hello-world

En cas d'erreur de démarrage du daemon:

	$ cat /var/log/syslog

	....
	[dockerd] ...  level=error msg="[graphdriver] prior storage driver aufs failed: driver not supported"

	$ sudo rm -rf /var/lib/docker/aufs
	$ sudo systemctl start docker

Essayer plusieurs démarrages si nécéssaire.


Installer Docker Compose:

	$ sudo pip install docker-compose 

	# ou

	# Vérifier la version  nécessaire sur: https://github.com/docker/compose/releases
	
	$ export dockerComposeVersion=2.1
	$ curl -L https://github.com/docker/compose/releases/download/$dockerComposeVersion/docker-compose-`uname -s`-`uname -m` > /usr/local/bin/docker-compose
	$ chmod +x /usr/local/bin/docker-compose

## Utilisation

/!\ Les arguments sont (souvent) ordonnés !

Inspecter un volume:
	
	$ docker volume inspect volume-media 

Afficher des stats en temps réel:

	$ docker stats

Lancer un conteneur et obtenir un terminal
    
    $ docker run -ti ubuntu

Relancer un conteneur arrêté:

	$ docker exec -ti container_name bash

Lister les images disponibles
    
    $ docker images 

Lister les modifications d'une image
    
    $ docker diff $id

Enregistrer les modifications d'un conteneur (! id: conteneur, pas image)

    $ docker commit $id image_ubuntu_modifie

Exporter une image
    
    $ docker save image_ubuntu_modifie > /path/to/archive.tar

Lancer un conteneur daemon et lier un port (hote: 8080 / ctr: 80)
    
    $ docker run -d -p 127.0.0.1:8080:80 -p 443:443 image_ubuntu_modifie
    # En cas d'erreur d'ecoute, préciser l'adresse

Lancer un conteneur et lier un dossier (host:ctr)
    
    $ docker -v /var/www:/var/www

Stopper un conteneur

    $ docker stop id

Supprimer un conteneur
    
    $ docker rmi id_image

Surcharge de point d'entrée 
    
    $ docker run -ti --entrypoint /bin/bash docker-djoe

Lancer un script à partir d'un Dockerfile. "sync" permet d'éviter des erreurs

    $ RUN chmod +x script.sh && sync && ./script.sh

Récupérer l'adresse IP d'un conteneur:
    
    $ docker inspect --format '{{ .NetworkSettings.IPAddress  }}' containerid

Démarrer un conteneur avec systemd:

	# Nommer le conteneur pour pouvoir l'arrêter
	# Penser à supprimer le conteneur avant de le relancer

	$ vim /etc/systemd/system/docker-djoe.service

	[Unit]
	Description=Djoe Docker image

	[Service]
	ExecStartPre=-/usr/bin/docker rm heydjoe-container
	ExecStart=/usr/bin/docker run -d -p 20443:443 -p 20080:80 -p 20022:22 -p 29001:9001 -p 27070:7070 -p 29090:9090 -p 23000:3000 --name heydjoe-container heydjoe
	ExecStop=/usr/bin/docker stop -t 2 heydjoe-container


	[Install]
	WantedBy=default.target

Créer une clef GPG sans prompt:

	RUN bash -c "echo -e 'Key-Type: 1\nKey-Length: 2048\nSubkey-Type: 1\nSubkey-Length: 4096\nName-Real: root-user\nExpire-Date: 0' > /tmp/gpg-gen-key-informations"
	RUN gpg --gen-key --batch /tmp/gpg-gen-key-informations

Ajouter un hôte dans les known hosts:

	RUN mkdir -p ~/.ssh/
	RUN ssh-keyscan gitlab.bbuzcloud.com >> ~/.ssh/known_hosts
	RUN chown -R user:user ~

Obtenir l'adresse IP de l'hôte à partir d'un conteneur:

	$ /sbin/ip route|awk '/default/ { print $3 }'

