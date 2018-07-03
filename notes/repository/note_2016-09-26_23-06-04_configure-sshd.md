# Mémo SSH / SSHd


## Divers

Compter le nombre de tentatives d'authentification ratées:

	$  grep sshd.\*Failed /var/log/auth.log | wc -l

Changer une passphrase sur une clef:

	$ ssh-keygen -p -f ~/.ssh/id_rsa

Désactiver le prompt GUI de mot de passe SSH sous XFCE:

	Désactiver gnome-keyring-ssh

	$ Startup applications > Startup applications > ... 


## Configurer SSHD

Editer le fichier de configuration:

	$ sudo vim /etc/ssh/sshd_config

	# port d'écoute, le changer de préférence
	Port 22

	# adresse d'écoute
	Listen 0.0.0.0

	# clef privée
	HostKey /etc/ssl/key.pem

	# temps de déconnexion si identifiants incorrects
	LoginGraceTime 120

	# interdire le log root 
	# /!\ toujours interdire les mots de passe
	PermitRootLogin no # prohibit-password

	# connexion sans mot de passe à l'aide de clefs RSA
	RSAAuthentication yes

Pour désactiver l'identification par mot de passe:

	$ sudo vim /etc/ssh/sshd_config
	
	ChallengeResponseAuthentication no
	PasswordAuthentication no
	UsePAM no


## Configuration client

Les dernières versions de SSH supportent l'option Include:

	$ vim ~/.ssh/config

	Include config.d/config_perso

Si les clefs sont dans des sous dossiers de .ssh, elles ne seront pas ajoutées à l'agent SSH. 
Utiliser l'option AddKeysToAgent pour les ajouter automatiquement: 

	$ vim ~/.ssh/config

	AddKeysToAgent yes

Pour associer une clef à une connexion, ajouter au fichier ".ssh/config":

    Host XX.XX.XX.XX
        IdentityFile ~/.ssh/keyname
        Port 443
	IdentitiesOnly yes # Ne pas présenter d'autres clefs

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



## Cas d'erreurs


### Too many authentication failure

Pas de clefs spécifiées, envoi de toutes les clefs disponible
	
Solution: se connecter avec l'option PubKeyAuthentication à no:
	
	$ ssh -o PubkeyAuthentication=no root@10.0.2.75 

Ou configurer l'option IdentityFile correctement.


### Unable to negotiate with 172.17.0.3 port 22

Coté serveur, générer plus de clefs:

	$ sudo ssh-keygen -t rsa -f /etc/ssh/ssh_host_rsa_key
	$ sudo ssh-keygen -t dsa -f /etc/ssh/ssh_host_dsa_key

