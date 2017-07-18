# Prendre le contrôle d'une machine virtuelle Windows avec TightVNC

## Machine hôte

Créer une machine virtuelle Windows 7

Dans les paramètres réseau de la machine virtuelle, activer l'accès par pont (bridged mode)

## Machine virtuelle

Télécharger et installer TightVNC server:

	http://www.tightvnc.com/download.php

Configurer un mot de passe d'accès.

Configurer le port d'accès si nécéssaire, par défaut 5900.

Au survol de la souris sur l'icône de la barre des tâches, plusieurs adresses IP apparaissent.

## Machine hôte

Sur la machine hôte, télécharger et lancer le client Java:

	$ java -jar tvnc#####.jar

	

