# Exemple de mise en place de règles IPTables pratiques

Mise en place d'un firewall simple

Notes sur IPTables:

    -j jump : cible de la règle
    -A append : ajout à la règle
    -P policy : politique par défaut
    -p protocol : udp, tcp

## /etc/init.d/firewall
        
    #!/bin/sh
    ### BEGIN INIT INFO
    # Provides:          firewall
    # Required-Start:    $remote_fs $syslog
    # Required-Stop:     $remote_fs $syslog
    # Default-Start:     2 3 4 5
    # Default-Stop:      0 1 6
    # Short-Description: Start firewall at boot time
    # Description:       Enable service provided by daemon.
    ### END INIT INFO
    
    DAEMON_LOG=/var/log/firewall.log
    
    case "$1" in
      start)
            echo -n "$0 : "
                    sh /etc/init.d/firewall.disable
                    sh /etc/init.d/firewall.enable
            echo "[Started]"
            ;;
      stop)
            echo -n "$0 : "
                    sh /etc/init.d/firewall.deactivate
                echo "[Stopped]"
            ;;
      *)
            echo "Usage: $0 {start stop}"
            ;;
    esac
    
    exit 0


## /etc/init.d/firewall.enable
    
    #!/bin/sh
    ### BEGIN INIT INFO
    # Provides:          Firewall
    # Required-Start:    $local_fs $remote_fs $network $syslog
    # Required-Stop:     $local_fs $remote_fs $network $syslog
    # Default-Start:
    # Default-Stop:
    # X-Interactive:     false
    # Short-Description: Firewall 
    ### END INIT INFO
    
    iptables  -P INPUT DROP
    iptables  -P FORWARD DROP
    iptables  -P OUTPUT DROP
    echo "- Disable all trafic"
    
    # Do not interrupt exisitng connexions
    iptables  -A INPUT -m state --state RELATED,ESTABLISHED -j ACCEPT
    iptables  -A OUTPUT -m state --state RELATED,ESTABLISHED -j ACCEPT
    
    # Loopback (127.0.0.1)
    iptables  -A INPUT -i lo -j ACCEPT
    iptables  -A OUTPUT -o lo -j ACCEPT
    echo "- Enable Loopback : [OK]"
    
    # ICMP (ping)
    iptables  -A INPUT -p icmp -j ACCEPT
    iptables  -A OUTPUT -p icmp -j ACCEPT
    echo "- Enable ICMP : [OK]"
    
    # SSH IN/OUT
    iptables  -A INPUT -p tcp --dport 22 -j ACCEPT
    iptables  -A OUTPUT -p tcp --dport 22 -j ACCEPT
    echo "- Enable SSH : [OK]"
    
    # DNS In/Out
    iptables  -A OUTPUT -p tcp --dport 53 -j ACCEPT
    iptables  -A OUTPUT -p udp --dport 53 -j ACCEPT
    iptables  -A INPUT -p tcp --dport 53 -j ACCEPT
    iptables  -A INPUT -p udp --dport 53 -j ACCEPT
    echo "- Enable DNS : [OK]"
    
    # HTTP + HTTPS Out
    iptables  -A OUTPUT -p tcp --dport 80 -j ACCEPT
    iptables  -A OUTPUT -p tcp --dport 443 -j ACCEPT
    
    # HTTP + HTTPS In
    iptables  -A INPUT -p tcp --dport 80 -j ACCEPT
    iptables  -A INPUT -p tcp --dport 443 -j ACCEPT
    echo "- Enable HTTP(s) : [OK]"
    
    # Flood protection
    iptables -A FORWARD -p tcp --syn -m limit --limit 1/second -j ACCEPT
    iptables -A FORWARD -p udp -m limit --limit 1/second -j ACCEPT
    iptables -A FORWARD -p icmp --icmp-type echo-request -m limit --limit 1/second -j ACCEPT
    echo "- Flood protection: [OK]"
    
    iptables -v -L


## /etc/init.d/firewall.disable

	#!/bin/sh
	iptables -F
	iptables -P INPUT ACCEPT
	iptables -P OUTPUT ACCEPT
	iptables -P FORWARD ACCEPT
	exit


