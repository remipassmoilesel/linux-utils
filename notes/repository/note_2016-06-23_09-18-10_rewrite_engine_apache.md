# Mémo sur le rewrite engine d'Apache 2

Doc utile: https://httpd.apache.org/docs/current/fr/rewrite/intro.html#regex

Ne pas oublier d'inclure mod_rewrite soit:
	
	$ a2enmod rewrite
	
	ou dans virtual host:
	
	LoadModule rewrite_module /usr/lib/apache2/modules/mod_rewrite.so

Surtout ne pas oublier avant les règles:
	
	RewriteEngine on
	
Activer la journalisation pour debug (à retirer en prod sous peine de ralentissements):
	
	LogLevel alert rewrite:trace8

Conditions d'execution d'une règle:

	RewriteCond1
	RewriteCond2
	RewriteRule modele substitution drapeaux
	
$1, $2, ... sont les élements capturés dans rewrite rule, accessible dans les rewritecond précédents

Exemple d'utilisation avec mod_proxy (flag P)

    ProxyVia On
    ProxyRequests Off
    ProxyPreserveHost on
    
    RewriteEngine on
    
    RewriteRule "/etherpad/p/(.*)$" "http://localhost:9001/p/$1" [P,L]
    RewriteRule "/etherpad/(.*)$" "http://localhost:9001/$1" [P,L]
    ProxyPassReverse ....

Exemple de redirection de toutes les requêtes vers une page statique:

    DocumentRoot /var/www/djoe/
    
    RewriteEngine On
    RewriteCond %{REQUEST_URI} !pre-https.html$
    RewriteRule ^ /pre-https.html [R=302]
