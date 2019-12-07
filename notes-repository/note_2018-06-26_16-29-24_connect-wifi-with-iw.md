# Se connecter au Wifi avec IW

Lister les interfaces disponibles:

	$ ip a

Activer l'interface désirée:

	$ ip link set <interface> up

Lister les réseaux accessibles:

	$ iw dev <interface> scan

Se connecter à un réseau sans authentification:

	$ iw dev connect <interface> <ssid>

Demander une adresse IP:

	$ dhclinet <interface>


