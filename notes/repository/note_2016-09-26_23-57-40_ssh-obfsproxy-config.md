# Utiliser SSH à travers obfsproxy

Installation:

	$ sudo apt-get install obfsproxy

Coté serveur, pour écouter le port 88 sur toutes les interfaces
 réseaux et rediriger le trafic vers le port 22 local:

	$ sudo obfsproxy --log-min-severity=info obfs3 --dest=127.0.0.1:22 server 0.0.0.0:88

Coté client ouvrir un sock pour dé-obfuscation, par exemple 9999:

	$ sudo obfsproxy --log-min-severity=info obfs3 socks 127.0.0.1:9999

Puis se connecter:

	$ ssh -o "ProxyCommand /bin/nc.openbsd -x 127.0.0.1:9999 %h %p" -p 88 user@server

Optimisation: utiliser --shared-secret="foobarfoo" pour renforcer encore l'obsfuscation. 
Longue chaine conseillée (superieure à 32 char, voir commande generate-key)

Source: https://www.void.gr/kargig/blog/2012/10/05/bypassing-censorship-devices-by-obfuscating-your-traffic-using-obfsproxy/


