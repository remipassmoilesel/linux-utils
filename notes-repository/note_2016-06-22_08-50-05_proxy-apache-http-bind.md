# Proxy Apache / BOSH Http-bind

Source: https://prosody.im/doc/setting_up_bosh

Tester la configuration:

	$ apachectl configtest

Vérifier que les modules nécéssaires soient installés:
$ sudo a2enmod rewrite proxy proxy_http

Configuration:
    
       <Location /http-bind>
           Order allow,deny
           Allow from all
       </Location>
       RewriteEngine On
       RewriteRule ^/http-bind$ http://example.com:5280/http-bind [P,L]

