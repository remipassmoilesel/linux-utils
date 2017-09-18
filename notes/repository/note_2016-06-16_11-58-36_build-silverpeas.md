# Build Silverpeas pour développement

Instructions disponibles sur: https://www.silverpeas.org/installation/installationV6.html

Nécéssite l'utilisation d'Openjdk 8:

    $ sudo apt-get install -y openjdk-8-jdk
    $ java -version

Nécéssite l'installation d'une base de donnée et de dépendances:

    $ sudo apt-get install -y postgresql postgresql-client pgadmin3
    $ sudo apt-get install -y imagemagick
    $ sudo apt-get install -y ghostscript

Optionnel:

    $ wget http://www.swftools.org/swftools-0.9.2.tar.gz
    $ tar -xvf swftools-0.9.2.tar.gz
    $ cd swftools-0.9.2
    $ ./configure
    $ sudo apt-get install libjpeg62-dev
    $ sudo apt-get install giflib-tools
    $ sudo apt-get install libfreetype6-dev
    $ sudo apt-get install libfreetype6
    $ ./configure
    $ make
    $ make install

    # En cas de probleme pendant le build, enlever l'instruction de suppression des anciens fichiers
    $ find . -name Makefile | xargs grep default_viewer.swf
    $ vi swfs/Makefile
    $ sudo make install

    $ mkdir pdf2json
    $ wget https://github.com/flexpaper/pdf2json/releases/download/v0.68/pdf2json-0.68.tar.gz
    $ cd pdf2json
    $ tar -xvf pdf2json-0.68.tar.gz
    $ ./configure
    $ make
    $ sudo make install


Télécharger les archives sur https://www.silverpeas.org/installation/installationV6.html:

    $ cd /opt
    $ mkdir silverpeas && cd silverpeas
    $ wget https://www.silverpeas.org/files/wildfly-10.0.0.Final.zip
    $ wget https://www.silverpeas.org/files/silverpeas-6.0-SNAPSHOT-wildfly10.zip
    $ unzip silverpeas-6.0-SNAPSHOT-wildfly10.zip
    $ unzip wildfly-10.0.0.Final.zip

Créer une base de données:

    $ sudo service postgresql start
    $ sudo -u postgres psql -c "alter user postgres with password 'postgres'"
    $ sudo -u postgres psql -c 'create database "Silverpeas"'
    
Adapter le fichier de configuration:

    $ cd $SILVERPEAS_HOME/configuration
    $ cp sample_config.proerties config.properties
    $ vim config.properties

Créer un ou plusieurs scripts avec variables d'environnement:

    $ new-script launch.sh

        #!/bin/bash

        # jeudi 16 juin 2016, 10:04:44 (UTC+0200)

        export SILVERPEAS_HOME=/opt/silverpeas-6.0-SNAPSHOT-wildfly10
        export JBOSS_HOME=/opt/wildfly-10.0.0.Final
        export JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64

        # Mode developpement
        export SILVERPEAS_DEV_MODE=true

        # Choisir une des trois commandes:

Les principales commandes

	# Installation fraiche
	$SILVERPEAS_HOME/bin/silverpeas clean install

	# Installation sans mise à jour du cache
	$SILVERPEAS_HOME/bin/silverpeas install

	# Démarrage
	$SILVERPEAS_HOME/bin/silverpeas start

	# pour profiter des variables d'environnement dans le shell:
	# . ./launch.sh

En cas d'erreur: Unsupported major/minor version:

	# Vérifier la version de Java
	$ java -version
	
	# changer de version
	$ sudo update-alternatives --config java
	$ sudo update-alternatives --config javac

Dossiers importants (base $SILVERPEAS_HOME):

    $ ls data
    $ ls data/web
    $ ls configuration
    $ ls deployments
    $ ls log
    $ ls configuration/jboss

    # Exemple de fichier de configuration JBOSS
    $ cat configuration/jboss/jackrabbit-ds.cli

    # Fichier de configuration Silverpeas et JB
    $ cat configuration/sample_config.properties

    # Fichier de configuration Silverpeas
    $ cat configuration/silverpeas/00-SilverpeasSettings.xml

Mode développement: décommenter les lignes appropriées dans bin/silverpeas.gradle

Récupérer les sources:

    $ mkdir sources
    $ cd sources
    $ git clone https://github.com/remipassmoilesel/Silverpeas-Core -b master
    $ git clone https://github.com/remipassmoilesel/Silverpeas-Components -b master

Créer un projet IntelliJ vide puis ajouter les modules Core et Components (Option importer un projet existant Maven)

Après une modification, build sans tests:

	# dans le dossier de sources modifiées
    $ mvn clean install -Dmaven.test.skip=true
    
    # puis dans l'installation silverpeas
    $ bin/silverpeas status
    $ bin/silverpeas reload

Deploiement de modifications rapide pour développement:
    
    # Si nécéssaire installer le projet
    $ cd sources
    $ mvn clean install -Dmaven.test.skip=true

    # Démarrer Silverpeas en mode debug pour ouvrir le port 5005
    bin/silverpeas debug

    # Compiler au moins une fois le projet
    Build / Make project

    # Si des erreurs surviennent, dans l'onglet Maven cliquer sur l'icone rafraichir
    puis recommencer

    # Aller dans Run / Configurations / Ajouter

    # Choisir 'Remote host'

    # Adapter le nom de l'hote, le nom de la config et le port

    Après une modification 



En cas de modification des fichiers de configuration faire un "restart"

Journaux disponibles (à partir de SILVERPEAS_HOME):

    $ vim /log/build-20160616_103430.log
    $ vi log/silverpeas-trace.log
    $ vi log/jboss_output.log
    $ vi ../wildfly-10.0.0.Final/standalone/log/server.log
    $ vi log/build-20160616_114503.log

Désactiver la minification des fichiers javascript pour développement:
Créer le fichier suivant:
    
    $ cat /opt/silverpeas-6.0-SNAPSHOT-wildfly10/configuration/silverpeas/CustomerSettings.xml                                                                                    [16:37:52]
    
    <?xml version="1.0" encoding="UTF-8"?>
    <silverpeas-settings product="Silverpeas">
    
      <fileset root="${SILVERPEAS_HOME}/properties/org/silverpeas/">
          <configfile name="general.properties">
                <parameter key="web.resource.js.get.minified.enabled">false</parameter>
                <parameter key="web.resource.css.get.minified.enabled">false</parameter>
          </configfile>
      </fileset>
    
    </silverpeas-settings>

Après il suffit d'executer rsync à destination de $SILVERPEAS_hOME/bin/build/...


