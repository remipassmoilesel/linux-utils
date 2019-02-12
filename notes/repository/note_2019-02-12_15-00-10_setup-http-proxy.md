# Utiliser un proxy HTTP d'entreprise

## Pour APT

    $ sudo vim /etc/apt/apt.conf.d/40proxy
    
    Acquire::http::Proxy "http://$USER:$PASSWORD@proxy.intra.fr:8090/";
    Acquire::https::Proxy "http://$USER:$PASSWORD@proxy.intra.fr:8090/";
  
    
## Global au système (hors exceptions)

    $ sudo vim /etc/environment
    
    http_proxy=http://$USER:$PASSWORD@proxy.intra.fr:8090/
    https_proxy=http://$USER:$PASSWORD@proxy.intra.fr:8090/
    ftp_proxy=http://$USER:$PASSWORD@proxy.intra.fr:8090/
    no_proxy=localhost,127.0.0.1,*intra.fr
    

## Docker 
    
Coté client:    
    
    $ vim ~/.docker/config.json
    
    {
      "proxies": {
        "default": {
          "httpProxy": "http://$USER:$PASSWORD@proxy.intra.fr:8090/",
          "httpsProxy": "http://$USER:$PASSWORD@proxy.intra.fr:8090/"
        }
      }
    }
    

Coté serveur:

    $ sudo mkdir -p /etc/systemd/system/docker.service.d
    $ sudo vim /etc/systemd/system/docker.service.d/http-proxy.conf
    
    [Service]
    Environment="HTTP_PROXY=http://$USER:$PASSWORD@proxy.intra.fr:8090/" "NO_PROXY=localhost,127.0.0.1,docker-registry.example.com,.corp"
    
    $ sudo systemctl daemon-reload
    $ sudo systemctl restart docker
    $ systemctl show --property=Environment docker
    

## SSH

Se connecter via un proxy HTTP:

    $ ssh -v destination -o "ProxyCommand=nc -X connect -x proxy.domain.fr:8090 %h %p"    