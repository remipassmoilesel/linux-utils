# Demarrer une commande avec le système

Utiliser le fichier /etc/rc.local pour ajouter des commandes. 
Attention a rajouter un "&" a la fin pour les commandes longues.

Il est préférable d'utiliser un script Systemd.

Extrait de stackoverflow:

    The script /etc/rc.local is for use by the system administrator. It is executed after all the normal system services 
    are started, at the end of the process of switching to a multiuser runlevel. You might use it to start a custom service, 
    for example a server that's installed in /usr/local. Most installations don't need /etc/rc.local, it's provided for the 
    minority of cases where it's needed.
