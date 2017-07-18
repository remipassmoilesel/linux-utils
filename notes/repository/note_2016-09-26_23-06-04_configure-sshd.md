# Configurer SSH / SSHD

## Configurer SSHD

Editer le fichier de configuration:

	$ sudo vim /etc/ssh/sshd_config

	# port d'écoute
	Port 22 

	# adresse d'écoute
	Listen 0.0.0.0

	# clef privée
	HostKey /etc/ssl/key.pem

	# temps de déconnexion si identifiants incorrects
	LoginGraceTime 120

	# interdire le log root 
	# /!\ toujours interdire
	PermitRootLogin no

	# connexion sans mot de passe à l'aide de clefs RSA
	RSAAuthentication yes

## Authentification sans mot de passe

Pour générer et utiliser une clef RSA, coté client (necessite une authentification avec mot de passe):

	# vérifier les droits de lecture
	$ mkdir ~/.ssh && cd ~/.ssh
	$ sudo chmod -R go-rwx . 

	# générer une paire publique/privée forte
	$ ssh-keygen -t rsa -b 4096

	# copier la clef vers un poste distant
	$ ssh-copy-id user@host

	# ou manuellement sur le serveur
	$ cp authorized_keys authorized_keys_Backup
	$ cat id_rsa.pub >> authorized_keys

Pour utiliser une clef fournie:

	# Copier la clef dans le dossier ~/.ssh
	$ cp key ~/.ssh/

	# Changer les droits de la clef
	$ chmod 600 ~/.ssh/key

	$ ssh user@host -i /path/to/key

Pour associer une clef à une connexion, ajouter au fichier ".ssh/config":

    Host XX.XX.XX.XX
        IdentityFile ~/.ssh/keyname
        Port 443

Pour désactiver l'identification par mot de passe:

	$ sudo vim /etc/ssh/sshd_config
	
	ChallengeResponseAuthentication no
	PasswordAuthentication no
	UsePAM no

## Divers

Compter le nombre de tentatives d'authentification ratées:

	$  grep sshd.\*Failed /var/log/auth.log | wc -l

Changer une passphrase sur une clef:

	$ ssh-keygen -p -f ~/.ssh/id_rsa

En cas d'erreur:

	debug1: kex: algorithm: curve25519-sha256@libssh.org
	debug1: kex: host key algorithm: (no match)
	Unable to negotiate with 172.17.0.3 port 22: no matching host key type found. Their offer: 

Coté serveur, générer plus de clefs:

	$ sudo ssh-keygen -t rsa -f /etc/ssh/ssh_host_rsa_key
	$ sudo ssh-keygen -t dsa -f /etc/ssh/ssh_host_dsa_key

	
