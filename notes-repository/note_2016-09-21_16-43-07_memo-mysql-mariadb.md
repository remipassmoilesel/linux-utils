# Memo Mysql / MariaDB

Se connecter en tant que root:

    $ sudo mysql -u root

Créer un utilisateur:

    > CREATE USER 'newuser'@'localhost' IDENTIFIED BY 'password';

Changer un mot de passe:

    > SET PASSWORD FOR 'jeffrey'@'localhost' = PASSWORD('litteral-password');

Afficher les utilisateurs:

   > SELECT user FROM mysql.user;

Créer une base de données:

    > CREATE DATABASE abcmapfrnopgrm

Autoriser l'accès à une base de données:

    > GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, ALTER, CREATE TEMPORARY TABLES, LOCK TABLES ON piwik_db.* TO 'piwik'@'localhost';
    -- ou
    > GRANT ALL PRIVILEGES ON dbname.* TO 'newuser'@'localhost';

Créer un dump:

    $ sudo mysqldump -u root -p[root_password] database_name table_name > dumpfilename.sql
    $ sudo mysqldump -u root -p[root_password] --databases database_name > dumpfilename.sql

Restorer un dump:

    $ sudo mysql -u root -p[root_password] [database_name] < dumpfilename.sql

Sur Android, en cas d'erreurs au lancement:

	$ sudo usermod -aG aid_inet mysql

Pour installer le driver JDBC de manière globale:

	$ sudo apt install libmysql-java

Suppression user / db:

	> DROP USER 'abcmapfr'@'localhost';
	> DROP DATABASE abcmapfr;


Taille des bases de données:

  > SELECT table_schema "database", sum(data_length + index_length)/1024/1024 "size in MB" FROM information_schema.TABLES GROUP BY table_schema;

Taille des tables:

  > SELECT table_schema AS DB_NAME, TABLE_NAME, (DATA_LENGTH+INDEX_LENGTH)/1024/1024 AS TABLE_SIZE_in_MB FROM information_schema.TABLES;


