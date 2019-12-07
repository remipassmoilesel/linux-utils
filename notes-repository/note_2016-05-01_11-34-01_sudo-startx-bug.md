# Bug après l'éxécution de 'sudo startx'

La pire idée du siècle: essayer de faire un 'sudo startx'.

Ainsi tous les fichiers de configuration de l'utilisateur appartiennent ensuite à root. 

Pour essayer de restaurer:

    $ sudo chown -R user:group /home/user




