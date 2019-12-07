# Mémo MariaDB


## Fonctions JSON / MariaDB 

MariaDB peut sélectionner, ajouter ou supprimer des objets JSON.

Voir: 
- https://mariadb.com/kb/en/library/json-functions/
- https://federico-razzoli.com/working-with-json-objects-in-mysql-mariadb

Exemple:
    
    > select JSON_EXTRACT(payload, "$.start"), JSON_EXTRACT(payload, "$.end"), aggregate_id, date_time, payload  from iotf_event_store where aggregate_id = "DrivingDataImportedEvent" order by date_time asc;
    > select JSON_EXTRACT(payload, "$.start"), JSON_EXTRACT(payload, "$.end"), aggregate_id, date_time, payload  from iotf_event_store where aggregate_id = "DrivingDataImportedEvent" order by date_time asc;
