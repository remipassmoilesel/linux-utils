# Utiliser Git à travers SSH

Créer un utilisateur 'git':

    $ adduser git --create-home
    $ sudo chsh git
        "/usr/bin/git-shell"

Puis créer un depot distant vide distant:

    $ sudo -iu git
    $ mkdir /git-rep
    $ cd /git-rep
    $ mkdir repository-name
    $ cd repository-name
    $ git init --bare

Puis ajouter l'origine dans le depot à pousser, et pousser:

    $ git remote add origin ssh://git@host:port/path/to/rep-rep
    $ git push origin master

Pour cloner:

    $ git clone ssh://git@host:port/git-rep/repository-name

Pour utiliser des clefs, modifier ".ssh/config". Voir la note sur l'utilisation des clefs et de SSH.
