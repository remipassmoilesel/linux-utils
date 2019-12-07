# Sauvegarder une base de données Postgres avec autopostgresqlbackup

## Objectif

Effectuer des dumps Postgres tous les jours, toutes les semaines, et tous les mois.

## Installation

	$ sudo apt install autopostgresqlbackup

## Fichiers

Configuration:

	/etc/default/autopostgresqlbackup

Tache cron:

	/etc/cron.daily/autopostgresqlbackup

Script et documentation:

	/usr/sbin/autopostgresqlbackup

Emplacement des sauvegardes:

	/var/lib/autopostgresqlbackup
	
## Restoration

Commande recommandée:

	$ gunzip file.gz (or bunzip2 file.bz2)
	$ psql --host dbserver --dbname database < /path/file.sql



