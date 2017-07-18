# Vérifier une image ISO

Télécharger l'image ISO, la somme SHA512 et le certificat de la somme.

	$ cd ~/playground/
	$ wget http://ftp.acc.umu.se/cdimage/stretch_di_rc1/amd64/iso-dvd/debian-stretch-DI-rc1-amd64-DVD-1.iso
	$ wget http://ftp.acc.umu.se/cdimage/stretch_di_rc1/amd64/iso-dvd/SHA512SUMS
	$ wget http://ftp.acc.umu.se/cdimage/stretch_di_rc1/amd64/iso-dvd/SHA512SUMS.sign

Télécharger la clef publique qui a servi à signer la somme de controle (chercher sur internet 'Debian Keyring' par exemple):

	$ gpg --keyserver keyring.debian.org --recv-keys "1046 0DAD 7616 5AD8 1FBC  0CE9 9880 21A9 64E6 EA7D"

Vérifier que la somme de contrôle est la bonne:

	$ gpg --verify SHA256SUMS.gpg SHA256SUMS


Si le message suivant apparait, importer la bonne clef:

	gpg: Signature faite le sam. 14 janv. 2017 00:47:38 CET avec la clef RSA d'identifiant 6294BE9B

	$ gpg --keyserver keyring.debian.org --recv-keys "6294BE9B"


Puis vérifier l'image CD:

	$ sha512sum SHA512SUMS debian-stretch-DI-rc1-amd64-DVD-1.iso 

	$ shasum -c SHA512SUM
	
	debian-stretch-DI-rc1-amd64-DVD-1.iso: OK




