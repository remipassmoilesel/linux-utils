# Caster un bureau sur un Raspberry PI

/!\ Utiliser plutôt TigerVNC !


## Avec X11VNC

Installer le serveur:

	$ sudo apt install x11vnc

Générer un fichier password:

	$ x11vnc -storepasswd "azerty" ~/.vnc_passwd 

Commencer une session:

	$ x11vnc -many -rfbauth ~/.vnc_passwd  

/!\ Ne nécéssite pas les droits ROOT pour caster.

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


