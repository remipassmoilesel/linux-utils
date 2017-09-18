# JTR

Exemple avec /etc/passwd

En cas de message d'erreur suivant:

        $ ./john --wordlist=password.lst --rules passwd
        No password hashes loaded (see FAQ)

        Effectuer la procédure 'unshadow'

Télécharger le code source et compiler:
Remarque: privilégier la compilation sur système cible pour optimisation.

	$ wget ...
	$ make
	$ make clean linux-x86-64-avx
	# pas besoin d'install

'unshadow': Préparer les fichiers si necessaire:

	$ sudo cp /etc/shadow
	$ sudo cp /etc/passwd
	$ ./unshadow passwd shadow > unshadowed-passwd 

Lancer:

	$ john unshadowed-passwd &

En cas de message:

	$ ./john --wordlist=password.lst --rules passwd
	No password hashes loaded (see FAQ)

	Effectuer la procédure 'unshadow'



Voir cet utilitaire pour faire un dump sur Windows:
	http://www.openwall.com/passwords/pwdump
