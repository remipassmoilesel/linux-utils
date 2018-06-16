# Postgres: installation rapide et memo

/!\ Attention, lors de la création de base de donnée, si le nom est spécifié entre double quote il faudra 
spécifier systèmatiquement les double quote et le nom deviendra sensible à la casse

## Installation

Installer:

    $ sudo apt-get update
    $ sudo apt-get install postgresql pgadmin3

Démarrer:

    $ sudo service postgresql start

Configurer pour écouter toutes les adresses:

    $ vim /etc/postgresql/9.5/main/postgresql.conf

    listen_addresses = '*'

Journaliser les connexions:

    log_connections = on

    $  tail -f /var/log/postgresql/postgresql-9.5-main.log 

## Memo shell PSQL

    \h                  Aide
    \q                  Quitter
    \c keycloak         Utiliser une base (USE)
    \d+ table_name      Décrire une table (DESCRIBE)    
    \l                  Lister les bases de données
    \dt                 Lister les tables d'une db
    \du			Lister tous les utilisateurs

## Utilisation

Executer une commande directement:

    $ sudo -u postgres psql -c "CREATE DATABASE somename"

Ouvrir un shell en tant que postgres

    $ sudo -iu postgres

Créer un super user au nom de l'utilisateur courant, sans mot de passe:
    
    > createuser -s -w remipassmoilesel

Ou en mode interactif:
    
    > createuser --interactive

Créer une bdd au nom de l'user unix:
    
    $ createdb remipassmoilesel

Ouvrir une bdd:

    $ psql remipassmoilesel

Créer un dump:

    $ pg_dump dbname > outfile

Importer à partir d'un dump
    
    $ psql databasename < data_base_dump

Pour supprimer une base de donnée et un utilisateur
    
    > dropdb bdname
    > dropuser username
    
Modifier le mot de passe d'un utilisateur:

    $ sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';"    

## Divers

Purger postgres:

	$ sudo apt-get --purge remove postgre\*

Si erreurs de creation de sockets lors du lancement:

	$ sudo usermod -aG aid_inet postgres
	# ou android_inet si necessaire

Ajouter un utilisateur et lui donner des droits:

	$ sudo adduser userlogin
	$ sudo passwd userlogin
	$ sudo -u postgres psql -c "CREATE USER  userlogin WITH PASSWORD 'password'";
	$ ...... "GRANT ALL PRIVILEGES ON DATABASE dbname TO userlogin" 

Pour installer les drivers JDBC:

	$ sudo apt install libpostgresql-jdbc-java


