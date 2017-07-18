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
    $ chroot sdx
    
    # installation
    grub-install /dev/sdx
    


