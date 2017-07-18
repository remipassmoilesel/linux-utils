# Construire Syslinux

	$ sudo apt-get install build-essential nasm uuid-dev 
	$ wget https://www.kernel.org/pub/linux/utils/boot/syslinux/syslinux-6.03.tar.gz 
	$ tar -xvf syslinux-6.03.tar.gz 
	$ cd syslinux-6.03.tar.gz 
	$ make -j 4 all

