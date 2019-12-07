# Utiliser SSH à travers HTTP

Fonctionne avec d'autres protocoles.

Installation coté serveur:

	$ sudo apt-get install httptunnel
	
Pour écouter le traffic sur le port 40443 et rediriger vers le serveur OpenSSH local:

	$ hts --forward-port localhost:22 40443 

Installation coté client:

	$ sudo apt-get install httptunnel

Pour encapsuler le trafic du port 1443 vers le serveur distant:

	$ htc -F 1443 vps303506.ovh.net:40443         

Ensuite pour se connecter au serveur en SSH:

	$ ssh -p 1443 remipassmoilesel@localhost 
