# Installation minimale de Browser Sync

Dans un terminal: 
    
    $ npm init
    $ npm install -g gulp
    $ npm install --save-dev browser-sync gulp 

Gulpfile.js:


var gulp = require('gulp');
var browserSync = require('browser-sync');

gulp.task('browser-sync', function() {
        browserSync({
server: {
      baseDir: "./"
          
}
  
                });

        });

gulp.task('bs-reload', function () {
  browserSync.reload();

        });

gulp.task('default', ['browser-sync'], function(){
  gulp.watch("stylesheets/**/*.css", ['bs-reload']);
    gulp.watch("javascripts/**/*.js", ['bs-reload']);
      gulp.watch("*.html", ['bs-reload']);

        });
