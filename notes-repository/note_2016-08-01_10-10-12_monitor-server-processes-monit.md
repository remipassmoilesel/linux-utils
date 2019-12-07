# Surveiller des processus serveur avec monit

Installation
    
    $ sudo apt-get update && sudo apt-get install monit

Configuration

    $ sudo vim /etc/monit/monitrc

Etat du service

    $ sudo service monit status

# Exemple de fichier de surveillance Apache / SSLH / SSH:

    check process apache with pidfile /var/run/httpd.pid
        start program = "/etc/init.d/apache2 start"
        stop  program = "/etc/init.d/apache2 stop"

    check process sshd with pidfile /var/run/sshd.pid
        start program = "/etc/init.d/ssh start"
        stop  program = "/etc/init.d/ssh stop"   

    check process sslh with pidfile /var/run/sslh/sslh.pid
        start program = "/etc/init.d/sslh start"
        stop  program = "/etc/init.d/sslh stop"  
