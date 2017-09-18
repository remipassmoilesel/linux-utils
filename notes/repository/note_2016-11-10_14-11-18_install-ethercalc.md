# Installer Ethercalc

/!\ Ne fonctionne pas pour l'instant, l'option 'basepath' de Ethercalc n'ayant aucun effet.

Nécéssite NodeJS.

	$ cd /opt
	$ npm install ethercalc
	$ mv node_modules/ethercalc .

Créer un script de contrôle:

	$ sudo vim /opt/ethercalc/ethercalc-ctl

	#!/bin/sh
	# kFreeBSD do not accept scripts as interpreters, using #!/bin/sh and sourcing.
	if [ true != "$INIT_D_SCRIPT_SOURCED" ] ; then
	    set "$0" "$@"; INIT_D_SCRIPT_SOURCED=true . /lib/init/init-d-script
	fi
	### BEGIN INIT INFO
	# Provides:          etherpad
	# Required-Start:    $remote_fs $syslog
	# Required-Stop:     $remote_fs $syslog
	# Default-Start:     2 3 4 5
	# Default-Stop:      0 1 6
	# Short-Description: Etherpad control
	# Description:       Etherpad control
	### END INIT INFO

	# Author: Rémi Passmoilesel <remi.passmoilesel@gmail.com>
	#

	DESC="Ethercalc"

	# Get function from functions library
	# . /etc/init.d/functions


	PATH="$PATH:/opt/nodejs4/bin/"

	do_start() {
		echo "Starting Ethercalc ..."

		/opt/ethercalc/bin/ethercalc --host 127.0.0.1 --port 8000 --basepath ethercalc

		PID=$(pgrep -f '/opt/ethercalc/bin/ethercalc --host 127.0.0.1 --port 8000 --basepath ethercalc')
		echo "Process ID: $PID"
	}

	do_stop() {

		echo "Stopping Ethercalc ..."

		PID=$(pgrep -f '/opt/ethercalc/bin/ethercalc --host 127.0.0.1 --port 8000 --basepath ethercalc')
		kill "$PID"

	}

	case "$1" in
	  start)
		do_start
		;;
	  stop)
		do_stop
		;;
	  restart|reload|condrestart)
		do_stop
		do_start
		;;
	  *)
		echo "Usage: $0 {start|stop|restart|reload|status}"
		exit 1

	esac

	exit 0

Créer un service:

	$ sudo vim /lib/systemd/system/ethercalc.service    

	[Unit]
	Description=Ethercalc

	[Service]
	ExecStart=/opt/ethercalc/ethercalc-ctl start
	ExecStop=/opt/ethercalc/ethercalc-ctl stop
	ExecReload=/opt/ethercalc/ethercalc-ctl reload

	[Install]
	WantedBy=multi-user.target

Activer et démarrer le service:

	$ sudo systemctl daemon-reload
	$ sudo systemctl enable ethercalc
	$ sudo systemctl start ethercalc


