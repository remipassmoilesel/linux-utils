# Installation minimale Gulp / Node-Sass

Nécéssite NodeJS.

Première installation globale:

	$ npm install -g gulp 
	
Gulp nécéssite d'être installé deux fois, une fois globalement puis dans chaque projet:

	$ npm install --save-dev gulp gulp-sass

Créer ensuite un fichier Gulpfile.js dans le projet:

	var gulp = require('gulp');
	var sass = require('gulp-sass');

	// transpiler les fichiers sass de 'css/' vers 'sass/'
	gulp.task('styles', function() {
	    gulp.src('sass/**/*.scss')
	        .pipe(sass().on('error', sass.logError))
	        .pipe(gulp.dest('./css/'));
	});


	// surveiller les fichiers sass
	gulp.task('default', ['styles'], function() {
	    gulp.watch('sass/**/*.scss',['styles']);
	});

Source: http://ryanchristiani.com/getting-started-with-gulp-and-sass/
