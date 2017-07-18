#!/bin/bash

echo "Start update: `date`" >> /git-rep/update.log

sudo -u git bash -c "/home/remipassmoilesel/linux-utils/git-backup -b >> /git-rep/update.log"

echo "Update done: `date`" >> /git-rep/update.log
