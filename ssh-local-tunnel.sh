#!/bin/bash

# jeudi 25 janvier 2018, 17:07:51 (UTC+0100)

# Publish a local port to a distant server
# You may need to registrate your SSH keys before
#
# Example: TARGET_HOST=user@server.domain.net TARGET_PORT=10022 LOCAL_PORT=22 ./ssh-local-tunnel.sh
#

# set -x

if ! [[ $TARGET_HOST ]] ; then
	echo You must specify a target host.
	exit 1
fi

if ! [[ $LOCAL_PORT ]] ; then
	echo You must specify a local port.
	exit 1
fi

if ! [[ $TARGET_PORT ]] ; then
	echo You must specify a target port.
	exit 1
fi

autossh -f -N -M 30068 -L $LOCAL_PORT:localhost:$REMOTE_PORT $TARGET_HOST
