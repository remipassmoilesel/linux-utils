# Linux utilities for Debian and derivated

AKA a third of my brain :)


Misc notes on GNU/Linux availables here:

https://github.com/remipassmoilesel/linux-utils/tree/master/notes-repository



Usage:

```
    $ git clone http://github.com/remipassmoilesel/linux-utils
```


## notes

Long notes management.

Usage:

```

usage: notes [-h] [-k] [-n] [-e] [-s] [-t] [-l] [-d] [-a] [-g]

Small utility to save notes. Notes are saved in: /...

optional arguments:
  -h, --help            show this help message and exit
  -k, --encrypt         encrypt with gpg (symetric) after edit
  -n, --newnote         create a new note
  -e, --editnote        edit exisiting note
  -s, --search          search in all notes
  -t, --edittemplate    edit template
  -l, --list            list all notes
  -d, --display         display a note
  -a, --displayall      display all notes
  -g, --graphicaleditor
                        use graphical editor

```


## search-commandfu

Search online command examples on commandfu.com

	$ search-commandfu grep                                                                                                                                                             
	
		http://www.commandlinefu.com/commands/matching/grep/Z3JlcA==/sort-by/votes/plaintext

		# commandlinefu.com - questions/comments: tech@commandlinefu.com

		# a function to find the fastest DNS server
		curl -s http://public-dns.info/nameserver/br.csv| cut -d, -f1 | xargs -i timeout 1 ping -c1 -w 1 {} | grep time | sed -u "s/.* from \([^:]*\).*time=\([^ ]*\).*/\2\t\1/g" | sort -n | head -n1

		# Find the package that installed a command
		whatinstalled () { local cmdpath=$(realpath -eP $(which -a $1 | grep -E "^/" | tail -n 1) 2>/dev/null) && [ -x "$cmdpath" ] && dpkg -S $cmdpath 2>/dev/null | grep -E ": $cmdpath\$" | cut -d ":
		" -f 1; }

		# Search some text from all files inside a directory
		grep -Hrn "text" .

		# find all active IP addresses in a network
		nmap -sP 192.168.1.0/24; arp-scan --localnet  | grep "192.168.1.[0-9]* *ether"

		....


## git-backup

Clone and pull from distant repositories, possibly with cron.

```
	$ git-backup -h
	usage: git-backup [-h] [-ec] [-e] [-sl] [-a] [-d] [-g] [-b]

	git-backup allow to save several repositories in a backup place. All
	configuration is available in git-backup_config.json

	optional arguments:
	  -h, --help            show this help message and exit
	  -ec, --edit-crontab   edit crontab
	  -e, --edit-config     edit configuration
	  -sl, --set-backup-location
							set backup location where repositories are cloned
	  -a, --append-repo     add a repo to clone
	  -d, --display-configuration
							display configuration file
	  -g, --use-graphical-editor
							use graphical editor for editing
	  -b, --backup          backup repositories
```

## switch-sys

Interactive chroot script, allow to repair a broken Debian/derivated system.

Usage:

```
	$ sudo switch-sys  

	Switch sys !

	NAME   MAJ:MIN RM   SIZE RO TYPE MOUNTPOINT
	sda      8:0    0 931,5G  0 disk
	└─sda2   8:2    0 931,5G  0 part /mnt/data
	sdb      8:16   0 119,2G  0 disk
	├─sdb1   8:17   0   237M  0 part /boot/efi
	├─sdb2   8:18   0  32,6G  0 part /
	├─sdb3   8:19   0   8,4G  0 part [SWAP]
	└─sdb4   8:20   0    78G  0 part /home
	sr0     11:0    1  1024M  0 rom  

	Nom de la partition cible: (ex: sda1, q = quit)
	...

```

## backup-sys

Interactive script to backup and compress a whole GNU/Linux system with RSYNC


```
	$ sudo backup-sys
```

## installation-script.py

Manage multiple packages installation.

Usage:

```
	usage: installation-script.py [-h] [-i] [-e] [-ai] [-au] [-d]

	Script d'installation. Execute des commandes, installe et désinstalle des
	paquets spécifiés dans le fichier : /.../installation.json

	optional arguments:
	  -h, --help            show this help message and exit
	  -i, --install         lancer le script
	  -e, --edit            editer le fichier d'installation
	  -ai, --append-packet-to-install
							ajouter un paquet à installer
	  -au, --append-packet-to-uninstall
							ajouter un paquet à désinstaller
	  -d, --display         montrer les commandes et paquets du fichier
```


## extract-ip

Extract IP adresses from stdin.

Example:

```
	$ echo `ip a | extract-ip`
```

## git-stats-java

Display stats on Java sources.

Usage:

```
	$ git-stats-java    

	Statistiques:
	2467 fichiers
	608 fichiers .java
	74575 lignes de fichiers source .java
	34966 caractères de fichiers source .java
	Soit 10489 cm de lignes de code !

```

## screenshot-1s

Screen capture by interval.
