# Changer les répertoires de stockage de Docker

Editer le fichier /etc/default/docker. Attention: le changement de répertoire
efface toutes les données des images et conteneurs précedemment utilisés.

Exemple de configuration:
    
    # Docker Upstart and SysVinit configuration file                                   
                                                                                       
    # Customize location of Docker binary (especially for development testing).        
    #DOCKER="/usr/local/bin/docker"                                                    
                                                                                       
    # Use DOCKER_OPTS to modify the daemon startup options.                            
    #DOCKER_OPTS="--dns 8.8.8.8 --dns 8.8.4.4"                                         
    DOCKER_OPTS="--storage-driver=overlay -D -g /mnt/hdd1/docker-files/"               
                                                                                       
    # If you need Docker to use an HTTP proxy, it can also be specified here.          
    #export http_proxy="http://127.0.0.1:3128/"                                        
                                                                                       
    # This is also a handy place to tweak where Docker's temporary files go.           
    export TMPDIR="/mnt/hdd1/docker-files/temp-files/"
