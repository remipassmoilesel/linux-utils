# Gestionnaire de mots de passe pass

Initialisation:

	$ sudo apt install gnupg2
	$ gpg --gen-key
	$ pass init user@email

Ajout d'un mot de passe:

	$ pass insert hierarchical/namespace/password-name

Afficher un mot de passe:

	$ pass hierarchical/namespace/password-name

Copier dans le presse papier:

	$ pass -c hierarchical/namespace/password-name

Générer:

	$ pass generate

Afficher en qrcode:

	$ pass namespace/password-name -q

