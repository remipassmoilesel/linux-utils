# Surveiller un serveur Apache avec mod_status

Activer le module mod_status:

	$ sudo a2enmod status
	
Ajouter dans un virtualhost:

	# Monitoring utility
	<Location /server-status>
                SetHandler server-status
                Order allow,deny
                Allow from all 
        </Location>

/!\ Attention à restreindre l'accès ensuite.

Activer les rapports étendus:

	$ sudo vim /etc/apache2/apache2.conf

	ExtendedStatus on

Puis redémarrer le serveur:

	$ sudo service apache2 restart

