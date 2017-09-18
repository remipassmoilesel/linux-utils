# Installer et utiliser Apache Cordova

## Installation

Nécéssite NodeJS et un JDK.

Installation:

	$ npm install -g cordova

Pour Android nécéssite l'installation d'Android-SDK avec ces paquets:

	- Android SDK Tools
	- Android Platform-Tools
	- SDK Platform API 23
	- Intel x86 Atom_64 System Image (optionnel, accélère les machines virtuelles)

Vérifier que tous les pré-requis sont installés:

	$ cordova requirements

Les dossier ANDROID_SDK/tools et ANDROID_SDK/platform-tools doivent être accessibles dans le PATH.
La variable JAVA_HOME doit contenir le chemin du JDK.

## Créer un projet

Créer un projet:

	$ cordova create project_folder_path org.package.name ProjectName

Ajouter des plateformes:

	$ cordova platform add android
	$ cordova platform add ios
	$ cordova platform remove ios

	$ cordova platform ls

Le dossier `www` contient l'application.
Pour la construire:

	$ cordova build

## Lancer un projet

Pour lancer un projet sur émulateur Android (une machine virtuelle Android doit déjà être crée):

	$ cordova emulate android
	$ cordova emulate --target=emulator-5554 android

Pour utiliser Chromium comme émulateur, installer l'extension Ripple:

	http://emulate.phonegap.com/

Ensuite visiter le dossier du projet avec le navigateur Chromium. Attention, visiter d'abord la racine du projet, activer 
Ripple puis cliquer sur www (ou alors la configuration ne sera pas chargée)

## Plugin

Ajouter un plugin: 

	$ cordova plugin add cordova-plugin-geolocation
	$ cordova plugin add cordova-plugin-splashscreen

## Erreurs courantes

:processDebugResources FAILED

Le dossier `www` contient un ou plusieurs fichiers .gz, qu'il faut supprimer.

## Tricks

Index.html:

	Pour activer la détection de liens clickable, supprimer:
	<meta name="format-detection" content="telephone=no">





