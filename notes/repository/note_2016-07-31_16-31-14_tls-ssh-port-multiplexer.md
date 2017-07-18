# Utiliser le port 443 pour plusieurs utilisation: SSH, TLS, ...

Télécharger et installer sslh

    $ sudo apt-get update && sudo apt-get install sslh

Paramétrage
    
    $ sudo vim /etc/default/sslh

Mettre RUN=yes, l'adresse d'écoute, les redirections ...
Pour Apache2, mettre sslh à l'écoute sur 0.0.0.0:443 et Apache2 sur 127.0.0.1:445

Exemple de fichier:

    # Default options for sslh initscript
    # sourced by /etc/init.d/sslh

    # Disabled by default, to force yourself
    # to read the configuration:
    # - /usr/share/doc/sslh/README.Debian (quick start)
    # - /usr/share/doc/sslh/README, at "Configuration" section
    # - sslh(8) via "man sslh" for more configuration details.
    # Once configuration ready, you *must* set RUN to yes here
    # and try to start sslh (standalone mode only)

    RUN=yes

    # binary to use: forked (sslh) or single-thread (sslh-select) version
    DAEMON=/usr/sbin/sslh

    DAEMON_OPTS="--user sslh --listen 0.0.0.0:443 --ssh 127.0.0.1:22 --ssl 127.0.0.1:443 --pidfile /var/run/sslh/sslh.pid"

	# Ou avec OpenVPN:
	DAEMON_OPTS="--user sslh --listen 137.74.161.106:443 --ssh 127.0.0.1:10022 --openvpn 137.74.161.106:1443 --ssl 127.0.0.1:443 --pidfile /var/run/sslh/sslh.pid"

En cas de problème de log:

Vérifier la configuration des redirections: désactiver ssl si non nécessaire, ...
Essayer une règle rsyslog, par exempe dans le fichier "/etc/rsyslog.d/10-disable-sslh.conf"
    
        # Discard all messages from sslh
        :msg, contains, "sslh[" stop 

        # Tester la configuration:
        $ rsyslog -N1


Configuration de SSH pour connexion et utilisation de RSYNC sur un autre port (Avec clef privée):

		Host 212.83.142.6
		  IdentityFile ~/.ssh/vm
		  Port 443
		
		Host im.silverpeas.net
		  IdentityFile ~/.ssh/vm
		  Port 443


Configuration Apache2 (possible sur 127.0.0.1:443 mais dans ce cas limiter sslh à l'adresse IP publique):
    
    Listen 127.0.0.1:445
    <VirtualHost 127.0.0.1:445>
    ...
    </VirtualHost>
    

Pour utiliser OpenVPN sur 443 modifier la configuration .ovpn comme suit:

		-- /!\ ne fonctionne qu'avec TCP, pas avec UDP
		remote vps303506.ovh.net 443 tcp


