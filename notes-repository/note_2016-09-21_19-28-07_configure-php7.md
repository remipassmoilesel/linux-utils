# Configurer PHP 7

Fichier de configuration:

    /etc/php/7.0/apache2/php.ini

Mémoire max:

    memory_limit = 128M

Afficher les erreurs pour développement:

    error_reporting = E_ALL 
    
Obtenir des informations sur l'installation:

    <?php
    
    echo "<p>posix_geteuid(): ";
    print_r(posix_geteuid());
    
    echo "<p>posix_getpwuid(posix_geteuid()): ";
    
    echo "<pre>";
    print_r(posix_getpwuid(posix_geteuid()));
    echo "</pre>";
    
    echo "<p>exec('whoami'): "; 
    echo exec('whoami'); 
    
    phpinfo();
    
    
    ?>

