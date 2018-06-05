# Memo

Simple utility used to memorize short commands.

Usage:


        remipassmoilesel@CriCriCriCri: ~/linux-utils/memo master ⚡
        $ 🙂 memo -h                                                                                                                                                                                             [13:15:02]
        usage: main.py [-h] [-s] [-f FILTER_CATEGORY] [-l] [-c CATEGORIZE] [-a]
                       [-u UPDATE] [-k DELETE] [-e] [-g] [-d] [-m]

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

        All informations are stored in file: /home/remipassmoilesel/linux-utils/memo/src/../memo-storage.txt

        optional arguments:
          -h, --help            show this help message and exit
          -s, --search          searh a memoy
          -f FILTER_CATEGORY, --filter-category FILTER_CATEGORY
                                use a category filter
          -l, --list-categories
                                display availables categories
          -c CATEGORIZE, --categorize CATEGORIZE
                                change categories of specified memo: 'category' 'id'
          -a, --append          append a memo: ['category'] 'header' 'content'
          -u UPDATE, --update UPDATE
                                update a memo: 'id' ['category'] 'header' 'content'
          -k DELETE, --delete DELETE
                                delete a memo: 'id'
          -e, --edit-all        edit whole memo file
          -g, --graphical-editor
                                use graphical editor
          -d, --display         display all
          -m, --modify          modify a memo
