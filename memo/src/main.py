# -*- coding: utf-8 -*-

import argparse

from CliHandlers import CliHandlers
from MemoElement import MemoElement
from Configuration import Configuration
from Logger import Logger

def parseArguments():
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

    # FIXME: remove me after end of refacto (handlers)
    container = cliHandlers.getAndLoadMemoContainer()

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

        if len(unknownArgs) < 1:
            raise Exception("You must specify keywords.")

        if knownArgs.filter_category:
            Logger.warning("Display only category: \"" + knownArgs.filter_category + "\"")

        category = knownArgs.filter_category.strip().lower() if knownArgs.filter_category is not None else None
        foundElements = container.searchByKeywords(unknownArgs, category)

        keywordsStr = ",".join(unknownArgs)

        if len(foundElements) == 0:
            Logger.error("Nothing found for: \"" + keywordsStr + "\"")

        else:
            Logger.header("Results for \"" + keywordsStr + "\":")
            Logger.info()

            for memo in foundElements:
                Logger.info(memo.getDisplayRepresentation())
                Logger.info()


    elif knownArgs.list_categories:

        Logger.header()
        Logger.header("Categories: ")
        Logger.header()

        categories = {}
        for memo in container.getMemoList():
            cat = memo.getCategory()
            val = categories.get(cat)
            val = val if val != None else 0
            categories[cat] = val + 1

        colLen = 25
        sortedKeys = sorted(categories.keys())

        for cat in sortedKeys:
            spaces = ""
            for i in range(colLen - len(cat)):
                spaces += " "

            Logger.info(cat + spaces + " (" + str(categories[cat]) + ")")

    else:
        raise Exception("Invalid command. Try --help")


if __name__ == "__main__":

    try:
        parseArguments()
    except Exception as e:
        Logger.error(str(e))

        if Configuration.DEBUG:
            raise e

        exit(1)
