# Installer Let's Encrypt sur Ubuntu 16 / Apache 2

Installation du client ACME.

	$ sudo add-apt-repository ppa:certbot/certbot
	$ sudo apt update	
	$ sudo apt-get install python-certbot-apache
	
Activer un certificat:

	$ sudo certbot --apache -d example.com -d www.example.com

Le renouveler:

	$ sudo crontab -e
	
	15 3 * * * /usr/bin/certbot renew --quiet

	# Le renouvellement sea vérifié tous les jours, mais effectué que lorsque nécéssaire.

Rediriger tout le traffic vers la connextion sécurisée:

	Redirect / https://example.com/	# Attention au slash de fin


