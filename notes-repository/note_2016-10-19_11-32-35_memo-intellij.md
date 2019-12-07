

Remettre à zéro la configuration en cours:

	Arrêter IntelliJ
	$ rm ~/.IntelliJIdea2016.1/config/

## Live templates

	CTRL-J:	Liste des templates

	psvm		Main()
	sout 		System.out.printLn()
	
	fori		Iteration loop
	itar		Iterate element of array
	iter		Iterate iterable

## Surround with

	CTRL ALT T (ou CTRL T)

## Code generation

	ALT inser

## Raccourcis

	MAJ F10			Run
	CTRL MAJ F10		Run current
	CTRL o			Surcharger une méthode
	MAJ F6			Refactoring
	CTRL ALT o		Régorganiser les imports
	CTRL i			Implémenter les méthodes

	CTRL q			Quick documentation (Javadoc)

	CTRL ALT gauche		Précédent
	CTRL ALT droite		Suivant

## Activer l'import automatique

	CTRL MAJ a > Auto import > Auto unambigous import on the fly

## Activer l'affichage automatique de la doc

	CTRL MAJ a > Code completion > Autopopup documentation ...

## Onglets sur le coté droit

	CTRL MAJ A > Editor tabs > Tabs placement > Right

## Désactiver la recherche de code dupliqué

	CTRL + MAJ + A
	Options > Editor > Inspections
	General > Duplicated code (enlever la coche)

## Activer la compilation partielle

Pour faire tourner une app même si des fichiers ont des erreurs:

	"Run configurations" > Enlever l'action "Make" et ajouter "Make, no error check"

## Erreurs:

	"Failed to read artifact descriptor"
	Si cette erreur persiste alors qu'un 'mvn compile' en ligne de commande ne créé pas d'erreur, utiliser l'installation 
	Maven du système (et non l'embarquée) peut résoudre ce problème.
	
	CTRL + MAJ + A 
	Maven Settings
	Maven home directory > /usr/share/maven

## Selection multiple
	
	CTRL + ALT + SHIFT + sélection à la souris

## Mode présentation:

	CTRL + MAJ + A / Toggle presentation mode

## Exclure les dossiers node_modules

	Settings > Editor > File types > Ignore files and folders (en bas) -> Mais ils ne sont plus accessibles en lecture

	Ou

	Settings > Directories > Exclude files: node_modules

## Bookmarks

	Ajouter: 	F11
	Naviguer: 	SHIFT + F11

## Setup python 3

Project structure CTRL+ALT+SHIFT+S

Project settings -> Facets -> expand Python click on child -> Python Interpreter

Project settings -> Modules -> Expand module -> Python -> Dependencies -> select Python module SDK


## Sauts de ligne bizarres

Si le symbole ^M apparait après un reformatage:

	Settings > Code Style > Line Separator > Unix


## Java: String is not defined

	Project Structure > SDK > Supprimer puis réimporter le JDK


## Désactiver les improts Java wildcard

  CTRL MAJ A > Settings > Editor > Code style > Java > Imports

  Class count to use import *: 99
  Names count to use static import with *: 99
