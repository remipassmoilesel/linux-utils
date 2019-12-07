# Utiliser btools pour créer des bookmarks de répertoire

Autojump est une alternative efficace et plus adaptable.

Installation:

	$ sudo pip2 install btools
	$ vim ~/.zshrc

	if [ -f '/usr/local/lib/python2.7/dist-packages/usr/share/bm/bm.bash' ]; then
	    source /usr/local/lib/python2.7/dist-packages/usr/share/bm/bm.bash
	    alias cm="cdbm"
	fi

Ensuite: 

	$ cd /one/directory
	$ bm -a bk1		# bk1 = nom du bookmark
	$ cm bk1	




