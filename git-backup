#!/usr/bin/python2.7
# encoding: utf-8

#
# Clone several repositories in a backup place, or pull master if repositories exist
#
#
#

import os
import subprocess
import datetime
import json
import argparse

# editors
GRAPHICAL_EDITOR = "xdg-open"
CLI_EDITOR = "vim"

CONFIG_FILE_NAME = "git-backup_config.json"

# hour of call
date = datetime.datetime.now().strftime("%Y-%m-%d-%H-%M")

# script location
scriptDir = os.path.dirname(os.path.abspath(__file__))

# config file path
configFilePath = scriptDir + "/" + CONFIG_FILE_NAME

# racine des journaux
logRoot = os.path.join(scriptDir, "log/")

# descirption du programme
pgrmdesc = '''
git-backup allow to save several repositories in a backup place.

All configuration is available in git-backup_config.json
'''


def exitProgram(code, text=""):
    print(text)
    exit(code)


def parseConfigurationFile():
    return json.load(open(configFilePath))


if __name__ == "__main__":

    # parse arguments
    parser = argparse.ArgumentParser(description=pgrmdesc)

    # edit crontab
    parser.add_argument("-ec", "--edit-crontab",
                        action="store_true",
                        help="edit crontab")

    # edit configuration file
    parser.add_argument("-e", "--edit-config",
                        action="store_true",
                        help="edit configuration")

    # set backup location
    parser.add_argument("-sl", "--set-backup-location",
                        action="store_true",
                        help="set backup location where repositories are cloned")

    # add a repo
    parser.add_argument("-a", "--append-repo",
                        action="store_true",
                        help="add a repo to clone")

    # display configuration
    parser.add_argument("-d", "--display-configuration",
                        action="store_true",
                        help="display configuration file")

    # use a graphical editor
    parser.add_argument("-g", "--use-graphical-editor",
                        action="store_true",
                        help="use graphical editor for editing")

    # backup all repositories
    parser.add_argument("-b", "--backup",
                        action="store_true",
                        help="backup repositories")

    # parse args
    knownArgs, otherArgs = parser.parse_known_args()

    # edit crontab
    if knownArgs.edit_crontab:
        subprocess.call("crontab -e", shell=True)
        exitProgram(0)

    # edit configuration
    if knownArgs.edit_config:
        editor = GRAPHICAL_EDITOR if knownArgs.use_graphical_editor == True else CLI_EDITOR
        subprocess.call(editor + " " + configFilePath, shell=True)
        exitProgram(0)

    # add repo
    if knownArgs.append_repo:

        # check if url is given
        if (len(otherArgs) < 2):
            exitProgram(1, "You must specify name and url for the repository.")

        # get configuration file
        try:
            data = parseConfigurationFile()
        except Exception as e:
            exitProgram(1, "Cannot open configuration file: " + str(e))

        # add repo
        data["repositories"].append({"name": otherArgs[0], "url": otherArgs[1]})

        # write new configuration
        try:
            jsonfile = open(configFilePath, "w")
            jsonfile.write(json.dumps(data, sort_keys=True, indent=4, separators=(',', ': ')))
        except:
            exitProgram(1, "Unable to write configuration: " + configFilePath)

        exitProgram(0, "Repository added.")

    # add repo
    if knownArgs.set_backup_location:

        # check if url is given
        if (len(otherArgs) < 1):
            exitProgram(1, "You must specify a valid location.")

        if os.path.isdir(otherArgs[0]) == False:
            exitProgram(1, "Invalid location: " + otherArgs[0])

        # get configuration file
        try:
            data = parseConfigurationFile()
        except Exception as e:
            exitProgram(1, "Cannot open configuration file: " + str(e))

        # set location
        data["backup-location"] = otherArgs[0]

        # write new configuration
        try:
            jsonfile = open(configFilePath, "w")
            jsonfile.write(json.dumps(data, sort_keys=True, indent=4, separators=(',', ': ')))
        except:
            exitProgram(1, "Unable to write configuration: " + configFilePath)

        exitProgram(0, "Backup location changed.")

    if knownArgs.display_configuration:
        subprocess.call("cat " + configFilePath, shell=True)
        exitProgram(0)

    if knownArgs.backup:

        print("Starting backup ...")

        data = parseConfigurationFile();

        cdCommand = "cd " + data["backup-location"] + " && "

        # iterate repos
        for rep in data["repositories"]:
            try:
                # repo already exist
                if os.path.isdir(data["backup-location"] + "/" + rep["name"]):
                    print(" -- Pull modifications from repo '" + rep["name"] + "' ... ")
                    subprocess.call(cdCommand + "cd " + rep["name"] + " && git pull origin master", shell=True)

                # repo not existing
                else:
                    print(" -- Cloning repo '" + rep["name"] + "' ... ")
                    subprocess.call(cdCommand + "git clone " + rep["url"] + " " + rep['name'], shell=True)
            except Exception as e:
                print("Error while backing-up repo '" + rep["name"] + "': " + str(e))

        exitProgram(0)

        # no valid command, print help
        print("Invalid command.")
        print("")
        parser.print_help()
        exitProgram(1)
