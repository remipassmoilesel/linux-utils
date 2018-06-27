# Désactiver les boutons d'arrêt avec XFCE


Pour limiter le reboot aux utilisateurs root et remipassmoilesel:

	$ sudo mkdir -p /etc/xdg/xfce4/kiosk/
	$ sudo vim /etc/xdg/xfce4/kiosk/kioskrc

	[xfce4-session]
	Shutdown=root,remipassmoilesel

Pour limiter l'utilisation en ligne de commande:

	$ sudo chmod o-x /sbin/shutdown
	$ sudo chmod o-x /sbin/reboot

Pour lightdm-gtk-greeter, utiliser lightdm-gtk-greeter-settings pour enlever le menu de gestion d'alimentation.
