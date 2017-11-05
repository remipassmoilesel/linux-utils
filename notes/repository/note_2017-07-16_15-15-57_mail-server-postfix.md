# Mettre en place un serveur mail Postfix

Voir: https://www.digitalocean.com/community/tutorials/how-to-install-and-setup-postfix-on-ubuntu-14-04

## Installation

	$ sudo apt update && sudo apt upgrade -y
	$ sudo apt install postfix

Choisir la configuration "Site internet"

## Configurer
	
S'assurer que le serveur n'écoute que localhost (mynetwork=...)

	$ sudo vim /etc/postfix/main.cf

Il est possible de créer des alias:

	$ sudo vim /etc/postfix/virtual

	# mail			# username
	blah@example.com        demouser
	dinosaurs@example.com   demouser
	roar@example.com        root
	contact@example.com     demouser,root

	$ sudo postmap /etc/postfix/virtual
	$ sudo systemctl restart postfix

Pour tester la configuration, envoyer un mail à une des adresses puis:

	$ cat /var/mail/username

## Configuration TLS avec Let's Encrypt

Générer un certificat puis:

	$ sudo vim /etc/postfix/main.cf

	# TLS parameters
	smtpd_use_tls=yes
	smtpd_tls_security_level = encrypt # ou 'may' pour accepter les connexions non TLS
	smtp_tls_security_level = encrypt
	smtpd_tls_session_cache_database = btree:${data_directory}/smtpd_scache
	smtp_tls_session_cache_database = btree:${data_directory}/smtp_scache

	smtp_tls_CAfile = /etc/letsencrypt/live/domain.fr/fullchain.pem
	smtpd_tls_cert_file = /etc/letsencrypt/live/domain.fr/fullchain.pem
	smtpd_tls_key_file = /etc/letsencrypt/live/domain.fr/privkey.pem

	smtp_tls_loglevel = 1
	smtpd_tls_loglevel = 1






