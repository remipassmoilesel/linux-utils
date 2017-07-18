# Mettre en place un serveur mail Postfix

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

Source: https://www.digitalocean.com/community/tutorials/how-to-install-and-setup-postfix-on-ubuntu-14-04



