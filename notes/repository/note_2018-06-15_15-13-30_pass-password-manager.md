# Gestionnaire de mots de passe pass

## Installation et initialisation

	$ sudo apt install gnupg2 xclip
	$ gpg --gen-key
	$ pass init user@email

## Ajouter

Ajout d'un mot de passe:

	$ pass insert namespace/password-name

## Utiliser

Afficher un mot de passe:

	$ pass namespace/password-name

Copier dans le presse papier:

	$ pass -c namespace/password-name

Afficher en qrcode:

	$ pass namespace/password-name -q

## Générer

Générer:

	$ pass generate namespace/password-name

## Avancé

Chercher un mot de passe:

	$ pass find terms...

Editer:

	$ pass edit pass-name

