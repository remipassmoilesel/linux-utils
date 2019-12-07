# Memo sur le script bash/zsh


## Flags

A utiliser sans modération:

	set -o nounset 	# erreur si utilisation d'une variable non déclarée
	
	set -o 		# afficher les autres options disponibles

	set -x 		# mode debug

	set -e 		# faire échouer un script au premier retour non zéro


## Variables

/!\ Attention aux espaces

	VAR="val" 


Variables spéciales:

	echo $# # contient le nombre de paramètres ;
	echo $0 # contient le nom du script exécuté (ici ./variables.sh) ;
	echo $1 # contient le premier paramètre ;
	echo $2 # contient le second paramètre ;
	echo # + commande shift


Substitution pour systemd:

	$ cat ~/.ssh/autossh/${TUNNEL_NAME}/autossh.service | envsubst > /lib/systemd/system/${SERVICE_NAME}


## Boucles

	# boucles
	while [ $i -lt 4 ]; do
		echo $i
		i=$[$i+1]
	done

	while true; do
		eject
		espeak Yooooooloooooooo
		sleep 1s;
	done


## Fonctions

	# Attention à l'espace après function 

	# fonction, paramètres et retours
	function dosomething {
		OUTPUT="That's done: $@ $1"
		echo $OUTPUT
	}

	# executer la fonction avec argument, et récuperer le retour
	RESULT=$(dosomething "do something today")
	
	# ou executer tout simplement
	dosomething


## Conditions et tests

Pour les tests toujours préferer [[ à [ (spécifique bash)

Exemple:

	if [ -z "$VAR" ]; then
	
	fi
	
Condition inversée:
	
	if ! [ -z "$VAR" ]; then

    fi
	
Opérateurs numériques:

	# Comparer des entiers. Attention aux espaces.
	if [ "$a" -eq "$b" ]; then

	fi

	-eq: =
	-ne: !=
	-gt: >
	-ge: >=
	-lt: >
	-le: >=

Autres:

    -z "string": vrai si non vide
    -x "string": vrai si est un fichier éxecutable
    -e "string": vrai si est un fichier 
    -d "string": vrai si est un répertoire

Voir: http://tldp.org/LDP/Bash-Beginners-Guide/html/sect_07_01.html

Simili try catch:

	{
		# try
	} || {
		# catch
	}


### Exemples de conditions

Itérer une liste de commandes:

	COMMANDS=(
	  "kubectl cluster-info"
	  "kubectl get componentstatuses"
	  "kubectl get nodes"
	  "kubectl get pods -n kube-system"
	);

	for cmd in "${COMMANDS[@]}"; do
	  echo ""
	  echo "========== Executing: $cmd =========="
	  echo ""
	  $cmd || :
	done

	for i in $(seq 5); do
		echo $i
	done

Tester si une commande existe dans une expression conditionnelle:

	if ! type "$foobar_command_name" > /dev/null; then
  		# install foobar here
	fi

Tester si une commande existe:

	 if [[ $(which curl) ]]; then
	    curl -fL --retry 3 --keepalive-time 2 "${url}" -o "${download_path}/${file}"
	  elif [[ $(which wget) ]]; then
	    wget "${url}" -O "${download_path}/${file}"
	  else
	    echo "Couldn't find curl or wget.  Bailing out." >&2
	    exit 4
	  fi


Tester le code de retour d'une commande:

	if [[ "$?" -ne "0" ]]; then
	    echo
	    echo "Error while sending keys to target."
	    echo
	    exit $?
	fi

Tester si une variable est non vide:

	if [[ -z "$HEYHEY" ]] ; then
		echo "$HEYHEY must exist !"
		exit 1
	fi
	
Tester si une variable est non définie, mais peut être vide:

	if [[ -z ${variable+x} ]] ; then
		echo "$HEYHEY exist"
	fi
	
Tester si une variable est indéfinie

	if ! [[ $GH_USER ]] ; then
            echo You must specify a user name.
            echo Example: clone-all-from-github remipassmoilesel
            exit 1
	fi

Tester si une commande existe:

	if hash gdate 2>/dev/null; then
		# gdate existe
		gdate "$@"
	else
		# gdate n'existe pas
		date "$@"
	fi

Tester si un paquet Debian est présent et l'installer au besoin:

	dpkg -s package &> /dev/null

	if [ $? -ne 0 ]; then
			echo "Installation"
			apt-get install package
	fi
	
Tester si un fichier est executable

	if [[ -x "$file" ]]
	then
		echo "File '$file' is executable"
	else
		echo "File '$file' is not executable or found"
	fi

Expression ternaire:

	MESSAGE=$([ -z "$@" ] && echo "Alerte !" || echo $@);
	
Tester si root:

	if [ "$(whoami)" != 'root' ]; then
			echo "You have no permission to run $0 as non-root user."
			exit 1;
	fi

Tester si un fichier existe:

	if [ -e "$FILE" ]; then
		echo "$FILE exist"
	fi

Tester si un répertoire existe:

	if [ -d "$FILE" ]; then
		echo "$FILE exist"
	fi

Comparer des chaines

	if [ "$a" = "$b" ]; then
        echo $a is equal to $b
	fi
	if [ "$a" != "$b" ]; then
        echo $a is not equal to $b
	fi

Vérifier si une chaine est vide,

	if [ -z "$VAR" ]; then
			echo "$VAR is empty"
			exit 1
	fi

## Divers


Incrémenter une variable

	export VAR=1
	echo $((++VAR))
	
Alias avec arguments:

	function ka() { kubectl "$@" --all-namespaces ;} 

Fixer les locales sur Debian:

	echo -e 'LANG="fr_FR.UTF-8"\nLANGUAGE="fr_FR:fr"\nLC_ALL="fr_FR"\nLC_TYPE="fr_FR.UTF-8"\n' > /etc/default/locale
	echo "fr_FR.UTF-8 UTF-8" > /etc/locale.gen
	dpkg-reconfigure -f noninteractive locales

Changer un mot de passe sans prompt:

	echo root:azerty | chpasswd

Désactiver les interactions utilisateurs:

	export DEBIAN_FRONTEND=noninteractive 

Créer une liste à partir d'une sortie console:

	DOCKER_CTR=($(docker ps -aq))

Substitution de commande (deux syntaxes similaires):

	export DATE=`date`;
	export DATE=$(date);

Chiffre aleatoire entre 2 et 6:

	RANDOMNBR=`shuf -i 2-6 -n 1`
	sleep $RANDOMNBR

Date formatée:

	date +'%Y-%m-%d_%H-%M-%S'
	
Multi-ligne:

	cat << EndOfMessage
	This is line 1.
	This is line 2.
	Line 3.
	EndOfMessage

Obtenir le chemin du script courant, ne fonctionne pas si l'appel se fait à partir d'un lien symbolique:

	DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
	
	# Changer les droits et executer en une commande, sans risque d'échec grâce à "sync"
	cd /opt/dependencies && chmod +x get-dependencies.sh && sync && ./get-dependencies.sh
	
	# Activer / desactiver le mode debuggage
	set -x
	set +x

	
## Entrées et sorties

	# Ecrire en bleu
	echo -e "\e[34mHello\e[0m World"

	# lire l'entrée utilisateur dans une variable
	read -e ONE_LINE

	# récupérer l'entrée standard
	STDIN=$(cat)

	# équivalent à 
	STDIN=$(cat /dev/stdin)

	# récupérer l'entrée standard (pipe)
	if [ -t 0 ]; then
		echo "No stdin"
	else
		STDIN=$(cat)
		echo "Stdin: $STDIN"
	fi
	
	# demander une confirmation
	if [[ $RESPONSE =~ ^(yes|y| ) ]]; then
		echo "Yes"
        break
	elif [[ $RESPONSE =~ ^(quit|q) ]]; then
		echo "Quit"
	else
		echo "No"
	fi



## Historique ZSH

Ne pas sauvegarder des commandes en fonction d'une regex:

	# Load regex extension module
	zmodload zsh/pcre

	# Hook executed before saving in history
	zshaddhistory() { 
		
		msg="Command will not be saved in history: $1"

		[[ $1 -pcre-match "^.*rm.*-rf.*$" ]] && echo $msg && return 1
		[[ $1 -pcre-match "^.*git.*reset.*$" ]] && echo $msg && return 1
		
		return 0
	}


