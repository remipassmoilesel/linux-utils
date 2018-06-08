# -*- coding: utf-8 -*-

import argparse

from CliHandlers import CliHandlers
from Configuration import Configuration
from Logger import Logger


class ArgumentParser:

    def parseArguments(self):

        parser = argparse.ArgumentParser(description=Configuration.PROGRAM_DESCRIPTION,
                                         formatter_class=argparse.RawTextHelpFormatter)

        parser.add_argument("-s", "--search",
                            action="store_true",
                            help="searh a memoy")

        parser.add_argument("-f", "--filter-category",
                            help="use a category filter")

        parser.add_argument("-l", "--list-categories",
                            action="store_true",
                            help="display availables categories")

        parser.add_argument("-c", "--categorize",
                            help="change categories of specified memo: 'category' 'id'")

        parser.add_argument("-a", "--append",
                            action="store_true",
                            help="append a memo: ['category'] 'header' 'content'")

        parser.add_argument("-u", "--update",
                            help="update a memo: 'id' ['category'] 'header' 'content'")

        parser.add_argument("-k", "--delete",
                            help="delete a memo: 'id'")

        parser.add_argument("-e", "--edit-all",
                            action="store_true",
                            help="edit whole memo file")

        parser.add_argument("-g", "--graphical-editor",
                            action="store_true",
                            help="use graphical editor")

        parser.add_argument("-d", "--display",
                            action="store_true",
                            help="display all")

        parser.add_argument("-m", "--modify",
                            action="store_true",
                            help="modify a memo")

        knownArgs, unknownArgs = parser.parse_known_args()

        cliHandlers = CliHandlers()

        Logger.info()

        if knownArgs.edit_all or knownArgs.graphical_editor:
            cliHandlers.openEditor(knownArgs.graphical_editor)

        elif knownArgs.categorize:
            category = knownArgs.categorize
            if not isinstance(category, str):
                raise Exception("You must specify category first, then memo ids")

            memoIds = unknownArgs
            if len(memoIds) < 1:
                raise Exception("You must specify at least one memo id")

            for memoId in memoIds:
                try:
                    int(memoId)
                except:
                    raise Exception("Invalid memo id: " + str(memoId))

            cliHandlers.categorizeMemo(category, memoIds)

        elif knownArgs.update:

            memoId = knownArgs.update

            if len(unknownArgs) < 2:
                raise Exception("You must specify at least a header and a content")

            if len(unknownArgs) > 2:
                category = unknownArgs[0]
                header = unknownArgs[1]
                content = unknownArgs[2]
            else:
                category = Configuration.DEFAULT_CATEGORY
                header = unknownArgs[0]
                content = unknownArgs[1]

            cliHandlers.updateMemo(memoId, category, header, content)

        elif knownArgs.delete:

            memoId = knownArgs.delete
            cliHandlers.deleteMemo(memoId)

        elif knownArgs.append:

            if len(unknownArgs) < 2:
                raise Exception("You must specify at least a header and a content to add a memo")

            for i, val in enumerate(unknownArgs):
                if len(val) < 1:
                    raise Exception("You can not specify empty arguments.")

            if len(unknownArgs) > 2:
                category=unknownArgs[0].strip().lower()
                header=unknownArgs[1].strip()
                content=unknownArgs[2].strip()
            else:
                category=Configuration.DEFAULT_CATEGORY
                header=unknownArgs[0].strip()
                content=unknownArgs[1].strip()

            cliHandlers.appendMemo(category, header, content)


        elif knownArgs.display:

            categoryFilter = knownArgs.filter_category.strip().lower() if knownArgs.filter_category else None
            cliHandlers.displayMemos(categoryFilter)

        elif knownArgs.search:

            keywords = unknownArgs
            if len(unknownArgs) < 1:
                raise Exception("You must specify keywords.")

            categoryFilter = knownArgs.filter_category.strip().lower() if knownArgs.filter_category else None

            if categoryFilter:
                Logger.warning("Display only category: \"" + categoryFilter + "\"")

            cliHandlers.searchAndDisplay(keywords, categoryFilter)

        elif knownArgs.list_categories:

            cliHandlers.listCategories()

        else:
            raise Exception("Invalid command. Try --help")


