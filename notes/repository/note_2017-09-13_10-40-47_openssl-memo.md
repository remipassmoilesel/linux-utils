# Memo Openssl et TLS

## Outil de gestion simplifiée

Voir Minica: https://github.com/jsha/minica

	$ go get github.com/jsha/minica
	$ minica --domains foo.com
	$ minica --ip-addresses 192.168.0.5

## Avec Openssl

Générer une clef et une requête de certification:

	$ openssl req -nodes -newkey rsa:4096 -keyout registry.key -out registry.csr \
		  -subj "/C=FR/ST=Grenoble/L=Grenoble/O=BeeBuzziness/OU=Docker registry/CN=$DOMAIN"

Afficher les informations d'un certificat (addresses et domaines par exemple):

	$ openssl x509 -in admin-10.0.4.110.pem  -text 

Générer une clef en une ligne:

	$ openssl req -new -newkey rsa:2048 -days 365 -nodes -x509 -keyout server.key -out server.crt

## Ajouter ces certificats à un système Debian ou dérivé

Choisir un dossier:

	$ cd /usr/share/ca-certificates

Ou:

	$ cd /usr/local/share/ca-certificates

Copier les clefs et mettre à jour le système (attention à l'extension des fichiers):

	$ cp key.pem key.crt
	$ sudo update-ca-certificates

## Ajouter à un navigateur web

Pour Vivaldi, Chromium ou Chrome:

	chrome://settings/certificates
