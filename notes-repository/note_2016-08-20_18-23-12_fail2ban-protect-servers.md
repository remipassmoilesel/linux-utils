# Utiliser fail2ban pour protéger un serveur

'fail2ban' analyse les journaux d'authentification et fixe des rèbles iptables en conséquence

    $ sudo apt-get install fail2ban

Copier le fichier /etc/fail2ban/jail.conf en jail.local
Toutes les modifications du fichier .local seront prises en compte.

    $ sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local

Ensuite modifier le fichier de conf en conséquence, notamment le temps qui est trop court par défaut (600ms)

