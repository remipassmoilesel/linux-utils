# Upload de fichiers vers un cloud privé OVH avec SFTP

Voir https://docs.ovh.com/fr/private-cloud/connexion-en-sftp/#objectif

Cette procédure permet par exemple d'uploader un fichier ISO vers un datacenter d'OVH.

Se connecter:

	$ sftp rpace@pcc.xxxxxx.ovh.net
	> ls
	> ls pcc-xxxx


Envoyer un fichier:

	> put /home/ubuntu-11.10-desktop-i386-fr.iso to /datastore/pcc-000714/ubuntu.iso



