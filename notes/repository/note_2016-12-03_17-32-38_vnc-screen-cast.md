# Caster un écran avec VNC

## Avec X11VNC

Installer le serveur:

	$ sudo apt install x11vnc

Générer un fichier password:

	$ x11vnc -storepasswd "azerty" ~/.vnc_passwd 

Commencer une session:

	$ x11vnc -many -rfbauth ~/.vnc_passwd  

/!\ Ne nécéssite pas les droits ROOT pour caster.

Une session X doit déjà exister pour se connecter. Si elle n'esiste pas, il est possible
d'utiliser l'option -create - mais c'est un peu brut ...

## Récupérer une session VNC vec deux tunnels SSH

Sur le post où est lancé VNC:

	$ ssh -R 5900:localhost:5900 user@vps.net 

Sur le poste où on veut récupérer la session:

	$ ssh -L 5900:localhost:5900 user@vps.net 

Ensuite:

	$ jtightvncviewer localhost:0 

## Avec TighVnc

/!\ Nécéssite les droits root pour caster

Installer un serveur X et un environnement de bureau:

	$ sudo apt-get install mate-desktop-environment lightdm xinit

Installer TighVNC server:

	$ sudo apt-get install tightvncserver 

Lancer une session:
	
	$ vncserver 
	$ sudo netstat -ltnp

Coté client:

	$ sudo apt-get install tighvnc-java
	$ jtightvncviewer 192.168.0.50:1 # ou 1 est le numéro de la session

Pour arrêter une session:

	$ vncserver -kill :1

## Avec tigervnc

	$ sudo apt install tigervnc-standalone-server

