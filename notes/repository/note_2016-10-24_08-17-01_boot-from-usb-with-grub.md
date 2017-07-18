# Boot à partir d'une clef USB avec Grub

Pour faire apparaitre le menu Grub au démarrage, appuyer sur Echap, Shift, ou espace.

Pour lister les périphériques:

	grub> ls

Boot à partir de hd1:
	grub> linux (hd1,msdos1)/install/vmlinuz root=/dev/sdb1
	grub> initrd (hd1,msdos1)/install/initrd.gz
	grub> boot

