# -*- coding: utf-8 -*-

import os


class Configuration:
    DEBUG = True

    MEMO_FILE_NAME = "memo-storage.txt"
    SCRIPT_PATH = os.path.dirname(os.path.realpath(__file__))
    MEMO_FILE_PATH = os.path.join(SCRIPT_PATH, '..', MEMO_FILE_NAME)

    PROGRAM_DESCRIPTION = '''
Simple memo utility, useful to store commands.

Examples: 

    Add a memo in category k8s:
    $ memo -a "k8s" "helm dependency build" "Update chart dependencies"

    Search a memo, and filter by category:
    $ memo -f k8s -s helm

    Edit all memos with CLI editor:
    $ memo -e

    Delete memo:
    $ memo -k 121

    Categorize memo:
    $ memo -c k8s 536

All informations are stored in file: ''' + MEMO_FILE_PATH

    GRAPHICAL_EDITOR = "xdg-open"
    CLI_EDITOR = "vim"

    DEFAULT_CATEGORY = "default".lower()

    MEMO_HEADER_MARK = "#"

    MEMO_CATEGORY_MARK = "::"
