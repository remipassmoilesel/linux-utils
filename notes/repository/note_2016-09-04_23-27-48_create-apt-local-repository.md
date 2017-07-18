# Créer un dépôt local APT à partir d'une installation existante

Copier tous les paquets .deb du cache APT:
    
    $ sudo cp -R /var/cache/apt/archives local-repo

Créer un index:

    $ sudo dpkg-scanpackages . /dev/null | gzip -9c > Packages.gz

Rendre le dossier accessible via un serveur Apache si nécéssaire:

    $ sudo apt-get install -y apache2
    $ ln -s /var/www/local-repo local-repo

Pour installer un paquet sur une autre machine, la préparer:

    $ sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak
    $ sudo vim /etc/apt/source.list
    
        > Ajouter dans le cas d'un serveur Apache:
        > deb http://192.168.0.4/ubuntu_local ./

        > Et dans le cas d'un emplacement local:
        > deb file:/media/$USER/doc/ubuntu_local  ./ 

Ensuite:

    $ sudo apt-get update 
    $ sudo apt-get install inkscape ....

Attention aux architectures: les dépôts doivent correspondre (x86, amd64, arm, ...)

Source: https://doc.ubuntu-fr.org/tutoriel/comment_installer_un_depot_local





