# Tester le multicast IP avec OmPing

Entre deux conteneurs Docker:

Sur 172.17.0.3:

    $ omping -m 230.0.0.4 -p 45688 172.17.0.3 172.17.0.4

    -m 		Adresse multicast
    -p 		Port de découverte
    172...3 	Adresse locale
    172...4 	Adresse distante

Sur 172.17.0.4:
    
    $ omping -m 230.0.0.4 -p 45688 172.17.0.4 172.17.0.3
    
Exemple de sortie avec communication réussie:

	172.17.0.3 : multicast, seq=23, size=69 bytes, dist=0, time=0.138ms
	172.17.0.3 :   unicast, seq=24, size=69 bytes, dist=0, time=0.078ms
	172.17.0.3 : multicast, seq=24, size=69 bytes, dist=0, time=0.102ms
	172.17.0.3 :   unicast, seq=25, size=69 bytes, dist=0, time=0.081ms
	172.17.0.3 : multicast, seq=25, size=69 bytes, dist=0, time=0.104ms
	172.17.0.3 :   unicast, seq=26, size=69 bytes, dist=0, time=0.079ms
	172.17.0.3 : multicast, seq=26, size=69 bytes, dist=0, time=0.092ms
	172.17.0.3 :   unicast, seq=27, size=69 bytes, dist=0, time=0.126ms

Exemple de sortie en échec:

	172.17.0.4 : waiting for response msg
	172.17.0.4 : waiting for response msg
	172.17.0.4 : waiting for response msg
	172.17.0.4 : waiting for response msg
	172.17.0.4 : waiting for response msg
	^C
	172.17.0.4 : response message never received

Exemple de sortie sans multicast:

	10.233.90.17 :   unicast, seq=188, size=69 bytes, dist=2, time=0.661ms
	10.233.90.17 :   unicast, seq=189, size=69 bytes, dist=2, time=0.669ms
	10.233.90.17 :   unicast, seq=190, size=69 bytes, dist=2, time=0.756ms
	10.233.90.17 :   unicast, seq=191, size=69 bytes, dist=2, time=0.598ms
	10.233.90.17 :   unicast, seq=192, size=69 bytes, dist=2, time=0.639ms

