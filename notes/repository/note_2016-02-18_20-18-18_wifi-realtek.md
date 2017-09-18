# Installer les derniers pilotes wifi Realtek

    $ sudo apt-get install linux-headers-generic build-essential git
    $ git clone https://github.com/lwfinger/rtlwifi_new
    $ cd rtlwifi_new
    $ make clean
    $ sudo make install
    $ sudo modprobe rtl8723be 


