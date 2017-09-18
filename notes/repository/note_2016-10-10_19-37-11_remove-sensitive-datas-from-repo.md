# Enlever des données sensibles d'un dépôt Git avec BFG

Télécharger et installer BFG:

	$ cd /opt
	$ wget http://repo1.maven.org/maven2/com/madgag/bfg/1.12.13/bfg-1.12.13.jar

	$ vim /some/dir/in/path/bfg

	#!/bin/bash
	java -jar /path/to/bfg $@

	$ chmod +x /some/dir/in/path/bfg

Enlever un fichier de l'historique. Seul le nom du fichier sera recherché, ne pas utiliser de chemins :

	$ bfg --delete-files file-name

Si nécéssaire supprimer ensuite physiquement le fichier:

	$ rm file-name

Nettoyer ensuite l'historique:

	$ git reflog expire --expire=now --all && git gc --prune=now --aggressive


