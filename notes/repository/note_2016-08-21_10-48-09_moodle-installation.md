# Installer Moodle 3

Installer Apache2 et postgres
    
    $ sudo apt-get install php5 libapache2-mod-php5 php5-mcrypt postgres postgresql postgresql-client php5-psql php5-curl php5-gd php5-xmlrpc php5-intl

Télécharger l'archive et la décompresser dans un dossier accessible par apache
    
    $ cd /var/www/
    $ wget -O moodle3.tgz https://download.moodle.org/download.php/direct/stable31/moodle-latest-31.tgz
    $ tar -xvf moodle3.tgz

Utiliser l'outil en ligne de commande pour installation ou se rendre avec son navigateur sur le dossier moodle
    
    $ /usr/bin/php moodle/admin/cli/install.php

Informations:

    admin
    Admin123#
    Azerty123#

