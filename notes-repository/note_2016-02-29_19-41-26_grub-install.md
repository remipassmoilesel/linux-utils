# Installer / réinstaller grub

Exemple pour sdx à partir d'un système fonctionnel:

    $ sudo grub-install /dev/sdx

Exemple pour sdx à partir d'un live cd

    # monter les partitions nécéssaire
    $ cd /tmp
    $ mkdir sdx/
    $ sudo mount /dev/sdx sdx
    
    $ sudo mount --bind /dev/ sdx/dev/
    $ sudo mount --bind /sys/ sdx/sys/
    $ sudo mount --bind /proc/ sdx/proc/

    # Changement de système
    $ chroot /mnt/sdx /bin/bash
    
    # Mise à jour / réparation de la configuration (ajout de toutes les partions bootable)
    $ sudo update-grub  
    
    # installation
    $ grub-install /dev/sdx
