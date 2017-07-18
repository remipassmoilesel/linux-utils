# Connecter deux machines linux avec un cable croisé

Machine 1:

    ifconfig # repérer l'interface réseau
    sudo ifconfig eth0 192.168.10.10 netmask 255.255.255.0 up # affecter une adresse ip

Machine 2:
    
    ifconfig # repérer l'interface réseau
    sudo ifconfig eth0 192.168.10.11 netmask 255.255.255.0 up # affecter une adresse ip

Quelquefois le ping ne fonctionne pas tout de suite, et débrancher/rebrancher 
le cable réseau peut résoudre ce problème.

