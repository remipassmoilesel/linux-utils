# Memo Openssl

Générer une clef et une requête de certification:

	$ openssl req -nodes -newkey rsa:4096 -keyout registry.key -out registry.csr \
		  -subj "/C=FR/ST=Grenoble/L=Grenoble/O=BeeBuzziness/OU=Docker registry/CN=$DOMAIN"

Afficher les informations d'un certificat (addresses et domaines par exemple):

	$ openssl x509 -in admin-10.0.4.110.pem  -text 

Générer une clef en une ligne:

	$ openssl req -new -newkey rsa:2048 -days 365 -nodes -x509 -keyout server.key -out server.crt
