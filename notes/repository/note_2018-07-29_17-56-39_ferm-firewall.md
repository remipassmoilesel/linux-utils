# Ferm Firewall

Permet de créer des règles pare-feu claires que l'on peut versionner.

Voir: 

- http://ferm.foo-projects.org/download/2.1/ferm.html#introduction
- https://github.com/MaxKellermann/ferm/blob/master/examples

Installation:

	$ sudo apt install ferm


Exemple de règle pour le port SSH et HTTP strict, limité à certaines un certain réseau uniquement:

	$ vim /etc/ferm/ferm.d/firewall.ferm

	# Non testé !
	
	@def $NET_TRUSTED = 192.168.0.1/29;

	table filter {
	    chain INPUT {
		policy DROP;

		# connection tracking
		mod state state INVALID DROP;
		mod state state (ESTABLISHED RELATED) ACCEPT;

		# allow local connections
		interface lo ACCEPT;

		# respond to ping
		proto icmp icmp-type echo-request ACCEPT;

		# remote administration on ssh
		saddr $NET_TRUSTED proto tcp dport (22 1022) ACCEPT;

		# our services to the world
		proto tcp dport (443 80) ACCEPT;

		# the rest is dropped by the above policy
	    }

	    # outgoing connections are not limited
	    chain OUTPUT policy ACCEPT;

	    # this is not a router
	    chain FORWARD policy DROP;
	}


	$ sudo systemctl reload ferm
