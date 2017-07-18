# Déployer une application web sur serveur Tomcat avec Maven

Source: 	http://tomcat.apache.org/maven-plugin-trunk/tomcat7-maven-plugin/usage.html
		https://www.mkyong.com/maven/how-to-deploy-maven-based-war-file-to-tomcat/

Déclarer un utilisateur autorisé à publier, par exemple avec les identifiants par défaut du plugin Maven:
	
	$ vim /opt/apache-tomcat/conf/tomcat-users.xml

	 <user username="admin" password="" roles="manager-script"/>

	$ catalina.sh stop
	$ catalina.sh start

Créer une application web avec Maven:

	$  mvn archetype:generate -DgroupId={project-packaging}
		-DartifactId={project-name}
		-DarchetypeArtifactId=maven-archetype-webapp
		-DinteractiveMode=false

Ajouter le plugin Tomcat au pom (section build):


	<plugin>
		<groupId>org.apache.tomcat.maven</groupId>
		<artifactId>tomcat7-maven-plugin</artifactId>
		<version>2.2</version>
		<configuration>
			<url>http://localhost:8080/manager/text</url>
			<server>TomcatServer</server>
			<path>/mkyongWebApp</path>
		</configuration>
	</plugin>

Déployer:

	$ mvn tomcat7:deploy
	$ mvn tomcat7:undeploy
	$ mvn tomcat7:redeploy

Par défaut l'application sera non compréssée (war)

Personnaliser la configuration du plugin (section plugin Tomcat du pom):

	<build>
		<plugin>
			<configuration>
				<!-- Serveur distant -->
		        	<url>http://www.mydomain.com:1234/mymanager</url>

				<!-- Identifiants -->
				<server>myserver</server>

				<!-- Context path -->
				<path>/mycontext</path>
		        </configuration>
		</plugin>
	</build>

	Puis dans %MAVEN_PATH%/conf/settings.xml:
	
	<settings>
	  ...
	  <servers>
	    ...
	    <server>
	      <id>myserver</id>
	      <username>myusername</username>
	      <password>mypassword</password>
	    </server>
	    ...
	  </servers>
	  ...
	</settings>

Pour ajouter des ressources (lib par exemple), les placer dans le dossier WEB-INF
