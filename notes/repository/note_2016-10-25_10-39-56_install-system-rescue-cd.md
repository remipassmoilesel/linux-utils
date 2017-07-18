# Installer System Rescue CD sur une clef USB

Télécharger et monter l'image ISO:

	$ cd /tmp
	$ wget "http://downloads.sourceforge.net/project/systemrescuecd/sysresccd-x86/4.8.3/systemrescuecd-x86-4.8.3.iso?r=https%3A%2F%2Fwww.system-rescue-cd.org%2FDownload&ts=1477384222&use_mirror=netix"
	$ mkdir -p /tmp/cdrom
	$ sudo mount -o loop,exec systemrescuecd-x86-4.8.3.iso /tmp/cdrom  
	$ cd /tmp/cdrom

Démonter la clef USB cible au préalable:

	$ sudo umount /dev/sdc*

Puis lancer l'installeur:

	$ ./usb_inst.sh
