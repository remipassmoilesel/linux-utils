# Changer ou ajouter une passphrase à une clef privée

Attention a ne pas utiliser un algo trop faible (ex: des)

    $ sudo openssl rsa -aes256 -in srvstage.key -out srvstage.key.pwd

Source: http://www.entrust.net/knowledge-base/technote.cfm?tn=5898

