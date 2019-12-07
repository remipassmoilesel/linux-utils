# Convertir des documents avec pandoc

Installation:

	$ sudo apt-get install pandoc

Conversation markdown / html, en spécifiant les formats:

	$ pandoc test1.md -f markdown -t html -s -o test1.html

-s: standalone document (pas seulement un fragment)
-o: output file

Les formats peuvent êtres déduit grâce aux extensions:

	$ pandoc test1.md -s -o test1.tex

Source: http://pandoc.org/getting-started.html
