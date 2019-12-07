# Utilisation de mod_proxy avec Apache

Source: http://www.apachetutor.org/admin/reverseproxies

Tester la configuration:

	$ apachectl configtest

Inclusion:

	LoadModule  proxy_module         modules/mod_proxy.so
	LoadModule  proxy_http_module    modules/mod_proxy_http.so
	LoadModule  headers_module       modules/mod_headers.so
	LoadModule  deflate_module       modules/mod_deflate.so
	LoadModule  xml2enc_module       modules/mod_xml2enc.so
	LoadModule  proxy_html_module    modules/mod_proxy_html.so
	
Possibilité de réécrire les liens absolus grace à proxy_http_module (voir doc ci-dessus)

Exemple pour Etherpad:

	ProxyPass /etherpad/p/ http://127.0.0.1:9001/p/
	ProxyPassReverse /etherpad/p/ http://127.0.0.1:9001/p/

	ProxyPass /etherpad/ http://127.0.0.1:9001/
	ProxyPassReverse /etherpad/ http://127.0.0.1:9001/
	
	
Exemple en proxy HTTPS/HTTPS:

	SSLProxyEngine on
    # SSLProxyCheckPeerCN off    	# necessaire si erreur 500: SSL Proxy: Peer certificate does not match for .....
    # SSLProxyCheckPeerName off

    ProxyVia On
    ProxyRequests Off
    ProxyPreserveHost on

    ProxyPass /http-bind https://im.silverpeas.net/http-bind
    ProxyPassReverse /http-bind https://im.silverpeas.net/http-bind


/!\ Certains cas de proxy peuvent se compliquer avec ProxyPass (ex: requete POST transformée en GET)
Dans ce cas utiliser mod_rewrite avec le flag P
