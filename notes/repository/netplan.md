# Netplan: configuration réseau pour Bionic et supérieures

Voir https://netplan.io/

## Commandes principales

  $ sudo netplan try
  $ sudo netplan apply

## Configuration

Laisser au Network Manager le contrôle des interfaces:

  root@host: /etc/netplan
  # cat 01-network-manager-all.yaml.bak                                                                                                                                                                                                                               [18:06:38]
  # Let NetworkManager manage all devices on this system
  network:
    version: 2
    renderer: NetworkManager

  $ netplan try
  
  
