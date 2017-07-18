# Monter et/ou réparer une partition endommagée

Sources:
* https://www.flynsarmy.com/2013/06/how-to-mount-a-broken-ext4-drive/
* https://justcheckingonall.wordpress.com/2010/07/18/howto-repair-broken-ext4-partitions/	  
* https://linuxexpresso.wordpress.com/2010/03/31/repair-a-broken-ext4-superblock-in-ubuntu/

Si ce message apparait:

    mount: wrong fs type, bad option, bad superblock on /dev/sdi1,
    missing codepage or helper program, or other error
    In some cases useful info is found in syslog – try
    ...	

Et que 'dmesg | tail':

    [ 213.962722] EXT4-fs (sdi1): no journal found

On peut monter la partition en mode lecture seule:

	$ sudo mount -o loop,ro,noexec,noload /dev/sdi1 /mnt/your_broken_partition/


On peut également réparer un système endommagé à l'aide de cette commande:


	# Attention: ne pas exécuter sur un système monter, très dangereux
	# Eventuellement éxécuter à partir d'une clef live

	# commande de réparation mais qui efface les fichiers deffectueux
	$ sudo fsck -C 0 /dev/sdxxx
	
	# autre commande conseillée 
	$ fsck.ext4 -cDfty -C 0 /dev/sd**

	
