# Memo Openssl

Générer une clef et une requête de certification:

	$ openssl req -nodes -newkey rsa:4096 -keyout registry.key -out registry.csr \
		  -subj "/C=FR/ST=Grenoble/L=Grenoble/O=BeeBuzziness/OU=Docker registry/CN=$DOMAIN"
