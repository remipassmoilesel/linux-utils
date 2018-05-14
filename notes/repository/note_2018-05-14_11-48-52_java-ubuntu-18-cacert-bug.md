# Fixer le problème de ca cert avec Ubuntu 18 et Openjdk

En cas d'erreur, par exemple avec Maven:
	
	[ERROR] Plugin org.apache.maven.plugins:maven-clean-plugin:2.5 or one of its dependencies could not be resolved: Failed to read artifact descriptor for org.apache.maven.plugins:maven-clean-plugin:jar:2.5: Could no transfer artifact org.apache.maven.plugins:maven-clean-plugin:pom:2.5 from/to central (https://repo.maven.apache.org/maven2): java.lang.RuntimeException: Unexpected error: java.security.InvalidAlgorithmParameter Exception: the trustAnchors parameter must be non-empty -> [Help 1]

Cette solution peut être appliquée:

	# Ubuntu 18.04 and various Docker images such as openjdk:9-jdk throw exceptions when
	# Java applications use SSL and HTTPS, because Java 9 changed a file format, if you
	# create that file from scratch, like Debian / Ubuntu do.
	#
	# Before applying, run your application with the Java command line parameter
	#  java -Djavax.net.ssl.trustStorePassword=changeit ...
	# to verify that this workaround is relevant to your particular issue.
	#
	# The parameter by itself can be used as a workaround, as well.

	# 1. Save an empty JKS file with the default 'changeit' password for Java cacerts.
	#    Use 'printf' instead of 'echo' for Dockerfile RUN compatibility.
	/usr/bin/printf '\xfe\xed\xfe\xed\x00\x00\x00\x02\x00\x00\x00\x00\xe2\x68\x6e\x45\xfb\x43\xdf\xa4\xd9\x92\xdd\x41\xce\xb6\xb2\x1c\x63\x30\xd7\x92' > /etc/ssl/certs/java/cacerts

	# 2. Re-add all the CA certs into the previously empty file.
	/var/lib/dpkg/info/ca-certificates-java.postinst configure

Voir: https://stackoverflow.com/questions/6784463/error-trustanchors-parameter-must-be-non-empty
