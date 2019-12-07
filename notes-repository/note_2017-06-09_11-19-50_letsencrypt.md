# Memo Let's Encrypt

## Installation

Installation du client ACME.

	$ sudo add-apt-repository ppa:certbot/certbot
	$ sudo apt update	
	$ sudo apt-get install python-certbot-apache

	
## Créer un certificat et configurer Apache automatiquement

Configurer d'abord le DNS, le serveur doit être accessible par son nom.

Créer une configuration Apache basique et l'activer:

	$ vim /etc/apache2/sites-available/example.com.conf

	$ sudo a2ensite example.conf

Créer un certificat et configurer automatiquement Apache 2:

	$ sudo certbot -d example.com -d www.example.com

Renouvelement automatique (normalement plus necéssaire, un cron est inclu avec le paquet certbot):

	$ sudo crontab -e
	
	15 3 * * * /usr/bin/certbot renew --quiet

	# Le renouvellement sera vérifié tous les jours, mais effectué que lorsque nécéssaire.

Rediriger tout le traffic vers la connextion sécurisée:

	Redirect / https://example.com/	# Attention au slash de fin


## Certificat wildcard

Commande:

    $ certbot certonly --server https://acme-v02.api.letsencrypt.org/directory \
        --manual -d '*.domain.fr'

Remplir ensuite le challenge DNS.


## En cas d'erreur

**> Client with the currently selected authenticator does not support any combination of challenges that will satisfy the CA.**

Mettre à jour le client certbot.

