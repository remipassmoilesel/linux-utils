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
