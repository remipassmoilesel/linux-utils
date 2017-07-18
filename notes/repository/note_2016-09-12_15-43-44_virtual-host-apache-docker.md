# Créer un hébergement virtuel avec Apache2 et Docker

## Créer un futur hébergement sous la forme d'un conteneur Docker

Installer Docker
Créer une image Ubuntu avec un serveur SSH. Exemple de Dockerfile:

	FROM ubuntu:16.04
	MAINTAINER remipassmoilesel
	LABEL name="ubuntu_prepared" description="Ubuntu with Open/SSH and Apache2 servers"

	ADD entrypoint.sh /opt/entrypoint.sh
	RUN chmod +x /opt/entrypoint.sh

	# Install dependencies
	RUN apt-get update && \
			apt-get install -y openssh-server apache2 php7.0 mariadb-server

	RUN useradd --create-home admin && \
			echo "admin:admin" | chpasswd

	ENTRYPOINT /opt/entrypoint.sh

	EXPOSE 80 22

Construire l'image:

	$ docker build . -t ubuntu_prepared
	
La lancer et exposer les ports appropriés:

	$ docker run -d -p 9010:80 -p 10023:22 ubuntu_prepared  
	

Dans cet exemple les clients SSH pourront se connecter de cette manière:
	
	$ ssh -p 10023 admin@domain

## Adapter le comportement d'Apache 

Exemple de configuration: /etc/apache2/sites-available/http.conf

	Listen 80

	<VirtualHost *:80>

			ServerName container1.ddns.net

			ErrorLog ${APACHE_LOG_DIR}/error.log
			CustomLog ${APACHE_LOG_DIR}/access.log combined

			DocumentRoot /var/www/container1

	</VirtualHost>

	<VirtualHost *:80>

			ServerName container2.ddns.net

			ErrorLog ${APACHE_LOG_DIR}/error.log
			CustomLog ${APACHE_LOG_DIR}/access.log combined

			DocumentRoot /var/www/container2

	</VirtualHost>


Après modifications recharger la conf avec:

	$ sudo service apache2 restart
