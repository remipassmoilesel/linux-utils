# Installer et utiliser Vagrant

Permet de mettre en place des machines virtuelles simplement et de manière reproductible.
Pour construire des machines, vagrant utilise des template nommés "boxes".
Un store est disponible ici: https://app.vagrantup.com/boxes/search

## Installation

	$ sudo apt install vagrant virtualbox virtualbox-guest-additions-iso

## Utilisation

Préparer un fichier de vm Vagrantfile:

	$ mkdir project && cd project
	$ vagrant init hashicorp/precise64

Lancer une vm, ou la redémarrer:

	$ vagrant up

Pour mettre à jour une machine après une modif du Vagrantfile:

	$ vagrant reload

Statut:

	$ vagrant status

Eteindre proprement:

	$ vagrant halt

Détruire une vm:

	$ vagrant destroy

Ouvrir une session ssh:

	$ vagrant ssh

Lister les box disponibles:

	$ vagrant box list



## Vagrantfile

Créer un fichier basic:

	$ vagrant init

Squelette de fichier de config (2 est la version):

	Vagrant.configure("2") do |config|
		...
	end

Choisir une box pour commencer:

	config.vm.box = "hashicorp/precise64"
	config.vm.box_version = "1.1.0"

Dossiers partagés: par défaut vagrant monte le dossier courant dans /vagrant.
Mais pour ajouter un dossier:

	config.vm.synced_folder "src/", "/srv/website"

Il est possible de partager des fichiers avec rsync, sans additions invités.



### Réseau

Pour prendre en compte les modifications réseau il est souvent nécéssaire d'arrêter puis de redémarrer les machines
(vagrant halt, vagrant up)

Transfert de port:

	config.vm.network "forwarded_port", guest: 80, host: 8080

Rendre une machine accessible sur un réseau privé:

	config.vm.network "private_network", type: "dhcp" 	# adresse obtenur par dhcp, peut nécéssiter une configuration supplémentaire
	config.vm.network "private_network", ip: "192.168.50.4" # adresse fixe 

Rendre une machine accessible sur un réseau "public":

	config.vm.network "public_network" 			# dhcp
	config.vm.network "public_network", ip: "192.168.0.17"	# adresse fixe

L'accés public se fait par bridge. Vagrant demande sur quelle interface il doit faire le pont. 
Pour spécifier l'interface par défaut:

	config.vm.network "public_network", ip: "192.168.0.17", bridge: "wlan0"

Pour déterminer les adresses ip il est possible de se connecter en ssh (vagrant ssh) pour observer 
les interfaces de la machine virtuelle.

Il est possible de configurer le réseau manuellement en désactivant la conf. auto et en utilisant le 
provisionning par shell.

Configurer le fichier host de l'hôte automatiquement (ajouter une entrée pour la machine virtuelle
par exemple):

	$ vagrant plugin install vagrant-hostmanager


### Provisionning

Pour effectuer des opérations sur la machine, il est possible d'utiliser des scripts.
Par exemple créer un fichier bootsrap.sh avec des instructions puis ajouter:
	
	config.vm.provision :shell, path: "bootstrap.sh"

Ensuite: 
	
	$ vagran up

Pour mettre à jour une machine existante en marche:

	$ vagrant reload --provision

Provisionning en ligne:

	config.vm.provision "shell", inline: <<-SHELL
		apt-get update
		apt-get install -y apache2
	SHELL

Il est possible d'utiliser d'autres outils comme Chef, Ansible, etc ...



## Multi machine

Il est possible de créer plusieurs machines à la fois:

	Vagrant.configure("2") do |config|
	  config.vm.provision "shell", inline: "echo Hello"

	  config.vm.define "web" do |web|
	    web.vm.box = "apache"
	  end

	  config.vm.define "db" do |db|
	    db.vm.box = "mysql"
	  end
	end

Toutes les commandes utilisent ensuite le nom de la machine:

	$ vagrant up web
	$ vagrant halt web
	...



## Ressources

	https://www.vagrantup.com/docs/


