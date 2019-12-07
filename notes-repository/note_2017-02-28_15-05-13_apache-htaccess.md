# Mise en place d'une authentification rapide avec Apache 2

	$ cd /var/www/
	$ mkdir secretdir
	$ vim secretdir/.htaccess

	AuthType Basic
	AuthName "My Protected Area"
	AuthUserFile /var/www/secretdir/.htpasswd
	Require valid-user

	$ htpasswd -nb 'login' 'password' > secretdir/.htpasswd

Si pas de résultats, ajouter la directive 'AllowOverride' dans la configuration Apache 2.
