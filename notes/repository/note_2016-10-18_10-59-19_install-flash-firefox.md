# Installer Flash sur Firefox

Télécharger Flash player.

Puis le décompresser et l'installer:

	$ cd Downloads
	$ tar -xvf install-flash.####.tar
	$ sudo cp libflashplayer.so /usr/lib/mozilla/plugins

	$ sudo chmod +x libflashplayer.so

Il est possible de placer le plugin dans un autre dossier et de le lier grâce à la commande:

	$ sudo ln -s /.../libflashplayer.so .

Ensuite redémarrer le navigateur et vérifier le fonctionnnement du plugin sur:

	about:plugins
