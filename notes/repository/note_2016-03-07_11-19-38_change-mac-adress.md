# Changer d'adresse MAC
    
    $ sudo ifconfig eth0 down
    $ sudo ifconfig eth0 hw ether 00:80:48:BA:d1:30
    $ sudo ifconfig eth0 up
    $ sudo ifconfig eth0 |grep HWaddr

L'adresse sera RAZ au redémarrage.

Source: http://www.aboutlinux.info/2005/09/how-to-change-mac-address-of-your.html

Exemple: 
    
    $ sudo ifconfig wlp3s0 hw ether 00:00:48:BA:d1:12

