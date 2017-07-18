# Configurer une clef SSH avec Github

Générer une clef:

	$ ssh-keygen -t rsa -b 4096 -C "name@mail.com"    
	$ ssh-add key-file-name

Copier la clef dans le presse papier:

	$ xclip -sel clip < ~/.ssh/id_rsa_github.pub

L'ajouter au compte Github:

	Settings > SSH and GPG keys > Add new SSH key

Utiliser ensuite SSH pourles transferts:

	$ git remote rm origin
	$ git remote add origin git@github.com/login/repo.git
	$ git push origin master
