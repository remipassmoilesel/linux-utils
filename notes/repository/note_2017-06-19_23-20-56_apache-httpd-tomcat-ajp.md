# Mettre en place Tomcat et HTTPD en reverse proxy

Le proxy par protocole AJP est optimisé pour les communications entre Tomcat et Apache httpd. Cela peut résoudre 
les problèmes de sessions non fonctionnelles, etc...

Cette installation supposera que plusieurs sites sont hébergé grâce à HTTPD et Tomcat, et chacun doit avoir un 
chemin de contexte vide ( '/' )

Nécéssite des serveurs HTTPD et Apache2 fonctionnels.

## Configuration de Tomcat

Créer un connecteur AJP dans la configuration de Tomcat:

	$ sudo vim /opt/tomcat/conf/server.xml

        <Service name="service-localhost">
           
           <Connector address="127.0.0.1" port="8039" enableLookups="false" protocol="AJP/1.3" URIEncoding="UTF-8" />

           <Engine name="engine-localhost" defaultHost="domain.fr">
              <Host name="domain.fr"  appBase="webapps/appname" unpackWARs="true" autoDeploy="true">
              </Host>
           </Engine>

        </Service>

Créer un dossier appname, y placer le WAR renommé en ROOT.war pour que son chemin de contexte soit '/':

	$ mkdir /opt/tomcat/webapps/appname
	$ cd /tmp && wget https://tomcat.apache.org/tomcat-6.0-doc/appdev/sample/sample.war -O appname.war
	$ mv target/appname.war /opt/tomcat/webapps/appname/ROOT.war

## Configuration d'HTTPD

Activer le proxy dans la configuration Httpd:

	<VirtualHost *:443>

		ServerName domain.fr
		ServerAlias www.domain.fr

		LogLevel info

		ErrorLog ${APACHE_LOG_DIR}/appname-error.log
		CustomLog ${APACHE_LOG_DIR}/appname-access.log combined

		# Attention aux / à la fin du chemin
		ProxyRequests off
		ProxyPreserveHost on
		ProxyPass           /   ajp://localhost:8009/
		ProxyPassReverse    /   ajp://localhost:8009/
	
		# Peut être nécéssaire pour conserver les session et préserver le chemin du JSESSIONID
		# ProxyPassReverseCookiePath /app-name /
		# ProxyPassReverseCookieDomain localhost domain.fr

	</VirtualHost>

Activer le module de proxy AJP Httpd:

	$ sudo a2enmod proxy proxy_http proxy_ajp

## Redémarrer

Redémarrer les serveurs:

	$ sudo systemctl restart apache2
	$ sudo systemctl restart tomcat


