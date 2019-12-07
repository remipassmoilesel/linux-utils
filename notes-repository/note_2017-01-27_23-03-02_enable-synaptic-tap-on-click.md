# Activer la gestion fine des touchpad

Installer le pilote:

	$ sudo apt install xserver-xorg-input-synaptic
	$ sudo reboot now

Pour lister les options disponibles:

	$ synclient -l

Activer le click on tap:

	$ synclient TapButton1=1
	$ synclient TapButton2=3

Pour que les changements soient persistants: 

	$ sudo mkdir -p /etc/X11/xorg.conf.d  
	$ sudo cp /usr/share/X11/xorg.conf.d/70-synaptics.conf /etc/X11/xorg.conf.d/60-touchpad-settings.conf 
	$ sudo vim /etc/X11/xorg.conf.d/60-touchpad-settings.conf

	Section "InputClass"
        	Identifier      "Touchpad"                      # required
	        MatchIsTouchpad "yes"                           # required
	        Driver          "synaptics"                     # required
	        Option          "MinSpeed"              "0.5"
	        Option          "MaxSpeed"              "1.0"
	        Option          "AccelFactor"           "0.075"
	        Option          "TapButton1"            "1"
	        Option          "TapButton2"            "3"     # multitouch
		Option          "VertTwoFingerScroll"   "1"     # multitouch
		Option          "HorizTwoFingerScroll"  "1"     # multitouch
		Option          "VertEdgeScroll"        "1"
		Option          "CoastingSpeed"         "8"
		Option          "CornerCoasting"        "1"
		Option          "CircularScrolling"     "1"
		Option          "CircScrollTrigger"     "7"
		Option          "EdgeMotionUseAlways"   "1"
		Option          "LBCornerButton"        "8"     # browser "back" btn
		Option          "RBCornerButton"        "9"     # browser "forward" btn
	EndSection

Source: ftp://www.x.org/pub/X11R7.5/doc/man/man4/synaptics.4.html

