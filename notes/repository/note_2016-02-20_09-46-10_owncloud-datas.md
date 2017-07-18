# Changer le répertoire de données de OwnCloud

Stopper apache

    $ service apache2 stop

Editer le fichier de config
    
    $ sudo vim /var/www/owncloud/config/config.php

Modifier la section 'datadirectory'

Modifier le flag 'maintenance' pour le mettre à 'false' si besoin

Redémarrer apache

    $ service apache2 start



