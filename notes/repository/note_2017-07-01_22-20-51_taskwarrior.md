# Installer et utiliser

Sur Ubuntu:

	$ sudo apt install taskwarrior

Ajouter une tâche, avec ou sans projet attribué:

	$ task add Read Taskwarrior documents later
	$ task add priority:H Pay bills

Lister les tâches, avec ou sans filtre de projet:

 	$ task
	$ task project:projectName

Marquer comme fait, supprimer:

	$ task 2 done
	$ task 2 delete

Afficher les tâches d'hier ou de moins de 4 jours:

	$ task entry:yesterday list
	$ task entry.after:today-4days list

Afficher la liste des projets:

	$ task projects

Exporter une tâche au format JSON:

	$ task export 1

Voir: https://taskwarrior.org/docs/examples.html
