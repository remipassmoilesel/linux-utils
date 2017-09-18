Utiliser chroot pour réparer un système

/!\ Les systèmes cibles et sources doivent partager les mêmes architectures et distributions.

Vérifier les partitions disponibles et leurs points de montage:
    
    $ lsblk

Créer un dossier pour le montage:
    
    $ cd /tmp
    $ mkdir sdax/

Monter la partition cible:
    
    $ sudo mount /dev/sdax sdax/

Monter les dossier systèmes:

    $ sudo mount --bind /dev sdax/dev
    $ sudo mount --bind /proc sdax/proc
    $ sudo mount --bind /sys sdax/sys

Monter les autres partitions si le système est réparti en plusieurs partitions (/home, ...):
    
    $ mkdir sdax/home/
    $ sudo mount /dev/sday sdax/home/

Pour un accés internet, copier /etc/resolv.conf:
    
    $ sudo cp /etc/resolv.conf sdax/etc/resolv.conf

Changer de racine:
    
    sudo chroot sdax/

Attention: les logs sont effectues sur le système cible. Au besoin monter /var/log, + autres logs du système:
    
    sudo mount --bind /var/log sdax/var/log


