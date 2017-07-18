# Formater un disque en ligne de commande

1. Lister les disques
    
    ```
    # fdisk -l | grep '^Disk'
    ```

1. Partionner le disque

    ```
    # fdisk /dev/sdb
    ```

Commandes disponibles:

    m - print help
    p - print the partition table
    n - create a new partition
    d - delete a partition
    q - quit without saving changes
    w - write the new partition table and exit

Toujours utiliser w avant de quitter, pour écrire les changements effectués.

Formater le disque, en ext4 par exemple:

    ```
    mkfs.ext4 /dev/sdb1
    ```

Source: http://www.cyberciti.biz/faq/linux-disk-format/
