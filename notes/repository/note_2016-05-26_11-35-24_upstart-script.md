# Ecrire un script Upstart

Source: https://www.digitalocean.com/community/tutorials/the-upstart-event-system-what-it-is-and-how-to-use-it

Les scripts Upstart sont dépréciés, utiliser plutôt Systemd. 

Les scripts doivent être placés dans /etc/init/
Les scripts ne peuvent pas être vides

Métadonnées:

    description "A test job file for experimenting with Upstart"
    author "Your Name"

Démarrer lorsque tous les servieces sont prêts, et s'arrêter à l'arrêt de la machine:

    start on runlevel [2345]
    stop on shutdown

Vérifier la syntaxe d'un fichier:

    $ init-checkconf /etc/init/testjob.conf

Lancer un service:

    $ sudo service <servicename> <control>

Bloc de commandes principal:

    script

        export HOME="/srv"
        echo $$ > /var/run/nodetest.pid
        exec /usr/bin/nodejs /srv/nodetest.js

    end script

Bloc de commandes préliminaire:

    pre-start script
        echo "[`date`] Node Test Starting" >> /var/log/nodetest.log
    end script

Bloc de commandes éxécuté après arrêt:

    pre-stop script
        rm /var/run/nodetest.pid
        echo "[`date`] Node Test Stopping" >> /var/log/nodetest.log
    end script

