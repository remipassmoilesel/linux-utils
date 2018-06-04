# -*- coding: utf-8 -*-

import argparse
import subprocess

from MemoContainer import *


def exitProgram(code=0, msg=""):
    if msg != "":
        Logger.error(msg)

    exit(code)


def getAndLoadMemoContainer():
    container = MemoContainer()
    container.loadStorageFile(Configuration.MEMO_FILE_PATH)
    return container


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

    knownArgs, unkArgs = parser.parse_known_args()

    Logger.info()
    container = getAndLoadMemoContainer()

    if knownArgs.edit_all or knownArgs.graphical_editor:

        editor = Configuration.GRAPHICAL_EDITOR if knownArgs.graphical_editor is True else Configuration.CLI_EDITOR
        subprocess.call(editor + " " + Configuration.MEMO_FILE_PATH, shell=True)


    elif knownArgs.categorize:

        if len(unkArgs) < 1:
            raise Exception("You must specify at least one memo id")

        category = knownArgs.categorize
        for memoId in unkArgs:
            Logger.header()
            Logger.header("Add category '" + category + "' to memo " + memoId)

            memo = container.getById(memoId)
            if not memo:
                raise Exception("Unknown memo id: " + memoId)

            memo.categ = category
            container.updateMemo(memo)
            container.persistToStorage()

            Logger.success("Category changed.")


    elif knownArgs.update:

        memoId = knownArgs.update

        if len(unkArgs) < 2:
            raise Exception("You must specify at least a header and a content")

        memo = container.getById(memoId)
        if not memo:
            raise Exception("Unknown memo id: " + memoId)

        if len(unkArgs) > 2:
            memo.header = unkArgs[1]
            memo.content = unkArgs[2]
            memo.categ = unkArgs[0]
        else:
            memo.header = unkArgs[0]
            memo.content = unkArgs[1]
            memo.categ = Configuration.DEFAULT_CATEGORY

        container.updateMemo(memo)
        container.persistToStorage()

        Logger.success("Memo updated.")


    elif knownArgs.delete:

        memoId = knownArgs.delete
        memo = container.getById(memoId)
        if not memo:
            raise Exception("Unknown memo id: " + memoId)

        # FIXME: change delete strategy
        success = container.deleteMemo(memo)

        if success == True:
            Logger.success("Memo deleted.")
            exitProgram(0)
        else:
            exitProgram(1, "Error while deleting memo.")


    elif knownArgs.append:

        if len(unkArgs) < 2:
            raise Exception("You must specify at least a header and a content to add a memo")

        for i, val in enumerate(unkArgs):
            if len(val) < 1:
                raise Exception("You can not specify empty arguments.")

        memo = None
        if len(unkArgs) > 2:
            memo = MemoElement(header=unkArgs[1].strip(),
                               content=unkArgs[2].strip(),
                               categ=unkArgs[0].strip().lower())
        else:
            memo = MemoElement(header=unkArgs[0].strip(),
                               content=unkArgs[1].strip(),
                               categ=Configuration.DEFAULT_CATEGORY)

        # FIXME: change append strategy
        success = container.appendMemo(memo)

        if success:
            Logger.success("Memo added with success.")
            exitProgram(0)
        else:
            exitProgram(1, "Error while adding memo to file: " + Configuration.MEMO_FILE_PATH)


    elif knownArgs.display:

        if knownArgs.filter_category:
            Logger.warning("Display only category: \"" + knownArgs.filter_category + "\"")
            Logger.info()

        categoryFilter = knownArgs.filter_category.strip().lower() if knownArgs.filter_category else None
        for memo in container.getMemoList(categoryFilter):
            Logger.info(str(memo))
            Logger.info()


    elif knownArgs.search:

        if len(unkArgs) < 1:
            raise Exception("You must specify keywords.")

        if knownArgs.filter_category:
            Logger.warning("Display only category: \"" + knownArgs.filter_category + "\"")

        category = knownArgs.filter_category.strip().lower() if knownArgs.filter_category is not None else None
        elements = container.searchByKeywords(unkArgs, category)

        keywordsStr = ",".join(unkArgs)

        if len(elements) == 0:
            Logger.error("Nothing found for: \"" + keywordsStr + "\"")

        else:
            Logger.header("Results for \"" + keywordsStr + "\":")
            Logger.info()

            for m in elements:
                Logger.info(str(m))
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
        Logger.error("Invalid command.")
        Logger.error()
        parser.print_help()
        exitProgram(1)


if __name__ == "__main__":

    try:
        parseArguments()
    except Exception as e:
        Logger.error()
        Logger.error(str(e))

        if Configuration.DEBUG:
            raise e

        exit(1)
