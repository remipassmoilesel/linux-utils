# Créer une clef USB à partir d'une machine virtuelle existante 

... avec VirtualBox.

Conversion au format raw:

	$ VBoxManage internalcommands converttoraw lubuntu.vdi lubuntu.raw

Changer la taille d'un fichier vdi (taille en Mo):

	$ VBoxManage modifyhd ubuntu32-vs-wild.vdi --resize 16000

Ecriture (attention au sync):

	$ sudo dd if=lubuntu.iso of=/dev/sdc bs=4M && sync

Pour ajouter un drapeau de boot (normalement non nécéssaire):

	$ sudo fdisk /dev/sdx
	> a
	> [numéro de la partition à rendre bootable]
	> w
	
	
