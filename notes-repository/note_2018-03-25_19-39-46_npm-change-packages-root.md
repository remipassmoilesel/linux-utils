# Changer la racine de stockage des paquets de NPM

Editer .npmrc: 

	$ vim ~/.npmrc

	prefix = /home/user/npm-root

Ajouter ensuite dans son PATH:

	export PATH=$PATH:/home/user/npm-root/bin
