# Utiliser extundelete pour restorer un fichier supprimer 

Fonctionne unquement sur ext3 et ext4

	$ sudo apt-get install extundelete
	$ sudo extundelete --restore-file filename.txt /dev/sda1


