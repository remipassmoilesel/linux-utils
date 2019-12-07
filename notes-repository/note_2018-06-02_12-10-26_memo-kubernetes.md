# Mémo Kubernetes

## Installation

### Désactiver le swap est indispensable

	$ sudo vim /etc/fstab

	--> Commenter la ligne swap

### Installation sur Bionic avec Kubespray (02/06/2018)

Commenter le role docker, installer docker.io explicitement 
