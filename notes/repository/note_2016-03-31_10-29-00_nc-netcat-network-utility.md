# NC / Netcat utilitaires réseaux

Source: http://www.computerhope.com/unix/nc.htm

Exemple client / serveur:

Dans un terminal:

	$ nc -l 1234

Dans un autre terminal:

	$ nc 127.0.0.1 1234

Tout ce qui sera entré dans un sera affiché dans l'autre (après validation de la touche Entrée)

Autres exemples:

	$ nc -p 31337 -w 5 host.example.com 42
	Opens a TCP connection to port 42 of host.example.com, using port 31337 as the source port, with a timeout of 5 seconds.


	$ nc -u host.example.com 53
	Opens a UDP connection to port 53 of host.example.com.


	$ nc -s 10.1.2.3 host.example.com 42
	Opens a TCP connection to port 42 of host.example.com using 10.1.2.3 as the IP for the local end of the connection.

	$ nc -lU /var/tmp/dsocket
	Creates and listens on a UNIX-domain stream socket.


	$ nc -x10.2.3.4:8080 -Xconnect host.example.com 42
	Connects to port 42 of host.example.com via an HTTP proxy at 10.2.3.4, port 8080. This example could also be used by ssh.


	$ nc -x10.2.3.4:8080 -Xconnect -Pruser host.example.com 42
	The same as the above example, but this time enabling proxy authentication with username "ruser" if the proxy requires it.
