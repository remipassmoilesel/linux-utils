# Utiliser TighVNC avec un Raspberry PI

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


