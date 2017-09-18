# Installer et utiliser le SDK Android

## Installation

Télécharger et décompresser le SDK:

	$ cd /opt
	$ wget https://dl.google.com/android/android-sdk_r24.4.1-linux.tgz -O android-sdk-linux.tgz
	$ tar -xvf android-sdk-linux.tgz
	
Exécuter le gestionnaire du SDK:

	$ tar -xvf /opt/android-sdk-linux/tools/android

Sélectionner les plateformes et outils nécéssaires et valider.

Puis ajouter au PATH:

	export PATH="$PATH:/opt/android-sdk-linux/platform-tools"
	export PATH="$PATH:/opt/android-sdk-linux/tools"

Paquets utiles:

	- Android SDK Tools
        - Android Platform-Tools
	- Intel x86 Atom_64 System Image

## Machines virtuelles AVD (Android Virtual Device)

Gestionnaire de machines virtuelles avec GUI:

	$ android avd

Commandes relatives aux machines virtuelles:

	$ android list avd 				# lister les machines disponibles
	$ android create avd -n name -r targetID 	# créer une machine virtuelle
	$ emulator -avd imageId				# lancer une machine virtuelle

Une fois une machine virtuelle lancée, il est possible d'y accéder avec adb:

	$ adb devices
	$ adb install application.apk

Afficher le journal d'une machine:

	$ adb logcat
