# Installer et utiliser Apache Tomcat

## Installation et démarrrage

Télécharger et décompresser tomcat:

	$ cd /opt
	$ wget http://apache.mediamirrors.org/tomcat/tomcat-9/v9.0.0.M13/bin/apache-tomcat-9.0.0.M13.zip  
	$ unzip apache-tomcat...zip
	$ mv apache-tomcat... tomcat

Créer un utilisateur et un groupe:

	$ sudo groupadd tomcat
	$ sudo useradd -s /bin/false -g tomcat -d /opt/tomcat tomcat

Donner les droits nécéssaires au dossier d'installation:

	$ sudo chown -R tomcat:tomcat /opt/tomcat
	$ sudo chmod +x /opt/tomcat/*.sh

Créer un fichier de service:

	$ sudo update-java-alternatives -l # repérer le chemin de Java
	$ sudo vim /etc/systemd/tomcat.service


	[Unit]
	Description=Apache Tomcat Web Application Container
	After=network.target

	[Service]
	Type=forking

	Environment=JAVA_HOME=/usr/lib/jvm/java-1.8.0-openjdk-amd64/jre
	Environment=CATALINA_PID=/opt/tomcat/temp/tomcat.pid
	Environment=CATALINA_HOME=/opt/tomcat
	Environment=CATALINA_BASE=/opt/tomcat
	Environment='CATALINA_OPTS=-Xms128M -Xmx512M -server -XX:+UseParallelGC'
	Environment='JAVA_OPTS=-Djava.awt.headless=true -Djava.security.egd=file:/dev/./urandom'

	ExecStart=/opt/tomcat/bin/startup.sh
	ExecStop=/opt/tomcat/bin/shutdown.sh

	User=tomcat
	Group=tomcat
	UMask=0007
	RestartSec=10
	Restart=always

	[Install]
	WantedBy=multi-user.target


Activer le service:

	$ sudo systemctl daemon-reload
	$ sudo systemctl start tomcat
	$ sudo systemctl status tomcat

Pour démarrer le serveur manuellement à partir des scripts fournis:

	$ ./catalina.sh stop
	$ ./catalina.sh start

A l'écoute sur le port 8080 par défaut.

## Déploiement

Les déploiements se font sous forme d'archives ou dézippés dans le dossiers 'webapps'.

## Administration

Interface disponible à l'adresse http://127.0.0.1:8080.
Pour ajouter un utilisateur:

	$ vim conf/tomcat-users.xml

	Exemple d'utilisateur autorisé à utiliser l'application de management (manager-gui):
	
	<user username="tomcat" password="tomcat" roles="manager-gui"/>

## Erreurs courantes

	L'application à été déployée mais le contexte n'a pas pu être démarré ...

	$ mvn tomcat7:redeploy

	Ou:

	$ mvn clean
	$ mvn tomcat7:undeploy
	$ mvn tomcat7:deploy

## Configuration

Configuration standard:

	<service>
		<connector>	<!-- Connexion possible vers l'extérieur -->
		<engine>	<!-- Moteur de traitement de requête -->
			<host>	<!-- -->
	</service>

	<!-- Globalement tous les attributs "name" doivent être uniques. -->

Exemple de déploiement d'une app sur un port spécifique, avec son propre service, et modification du 
context path (application servie à la racine):

	<Service name="service-unique-name">
           <Connector port="8085" protocol="org.apache.coyote.http11.Http11NioProtocol" connectionTimeout="20000" />
           <Engine name="engine-unique-name" defaultHost="host.ovh.net">
              <Host name="host.ovh.net"  appBase="webapps"			<!-- Engine sert le dossier "CATALINA/webapps" pour les requêtes d' "host.ovh.net"
                        unpackWARs="true" autoDeploy="true">
                <Context docBase="app-name" path="" reloadable="true" /> 	<!-- Modification du context path ("") pour "app-name" -->
              </Host>
           </Engine>
        </Service>


Changer le port ou l'adresse d'écoute:

	$ sudo vim /opt/tomcat/conf/server.xml
	
	<Server address="127.0.0.1" port="8005" ...>

	<!-- Element à l'écoute de clients" -->
	<Connector address="127.0.0.1" port="8080" protocol="HTTP/1.1"
               connectionTimeout="20000"/>

	<!-- Element à l'écoute de clients --> 
	<Connector port="8009" protocol="AJP/1.3" address="127.0.0.1"/>

Accéder au manager web:

	$ sudo vim /etc/tomcat/conf/tomcat-users.xml

	<role rolename="manager-gui"/>
		<user username="remipassmoilesel" password="secret" roles="manager-gui" />


Accéder au manager web à partir d'une autre machine:

	$ sudo vim /etc/tomcat/conf/Catalina/localhost/manager.xml
	
	<Context privileged="true" antiResourceLocking="false" 
	         docBase="${catalina.home}/webapps/manager">
	    <Valve className="org.apache.catalina.valves.RemoteAddrValve" allow="^.*$" />
	</Context>

## Augmenter le cache pour améliorer les performances

La valeur de cache maximum est en kilobytes:

	$ vim /opt/tomcat/conf/context.xml
	
	<Context>
		...
		 
		 <Resources cachingAllowed="true" cacheMaxSize="120000" />

		...
	</Context>

	


