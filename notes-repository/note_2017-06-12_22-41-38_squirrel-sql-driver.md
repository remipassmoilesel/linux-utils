# Installer et utiliser SquirrelSql

Installation:

	$ cd /opt
	$ wget https://sourceforge.net/projects/squirrel-sql/files/1-stable/3.7.1-plainzip/...
	$ unzip squirrel.zip
	$ cd squirrel

Le logiciel à besoin de drivers pour se connecter aux BDD. Exemple d'installation de driver avec MySQL:

	$ cd /opt/squirrel
	$ cd lib/
	$ wget https://dev.mysql.com/get/Downloads/Connector-J/mysql-connector-java-5.1.42.zip
	$ unzip mysql-connector.zip
	$ mv mysql-connector-java-5.1.42/mysql-connector-java-5.1.42-bin.jar .   
	$ rm -rf mysql-connector/ mysql-connector.zip

Puis redémarrer SquirrelSQL si nécéssaire.


