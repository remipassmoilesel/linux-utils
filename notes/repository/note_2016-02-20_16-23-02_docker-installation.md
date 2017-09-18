# Installer Docker et l'utiliser docker sans commande sudo

Installation:

	$ sudo apt-get update
	$ sudo apt-get install apt-transport-https ca-certificates
	$ sudo apt-key adv --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys 58118E89F3A912897C070ADBF76221572C52609D
	$ rm /etc/apt/sources.list.d/docker.list
	$ echo "deb https://apt.dockerproject.org/repo ubuntu-wily main" > /etc/apt/sources.list.d/docker.list
	$ sudo apt-get update
	$ sudo apt-get install docker-engine
 	$ sudo service docker start
	$ sudo docker run hello-world

Configuration pour utilisation en tant qu'utilisateur normal:

	# une déconnexion/reconnexion sera peut-être nécéssaire après cette commande
	$ sudo usermod -aG docker remipassmoilesel
	$ docker run hello-world 



