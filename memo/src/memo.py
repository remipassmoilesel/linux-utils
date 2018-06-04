# -*- coding: utf-8 -*-

import argparse
import datetime
import os
import re
import subprocess


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


class Colors:
    PURPLE = '\033[95m'
    BLUE = '\033[94m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    ENDC = '\033[0m'


class Logger:

    @staticmethod
    def info(line=""):
        Logger.printColor(line)

    @staticmethod
    def warning(line=""):
        Logger.printColor(line, Colors.YELLOW)

    @staticmethod
    def error(line=""):
        Logger.printColor(line, Colors.RED)

    @staticmethod
    def printColor(line="", color=Colors.ENDC):
        print(color + str(line) + Colors.ENDC)

    @staticmethod
    def debug(data):
        if Configuration.DEBUG:
            Logger.printColor('Debug: ', Colors.YELLOW)
            print(data)


def exitProgram(code=0, msg=""):
    if msg != "":
        Logger.error(msg)

    exit(code)


class MemoElement:
    memoIdCounter = 0

    def __init__(self, header, content, categ, lineNumber=0):
        MemoElement.memoIdCounter = MemoElement.memoIdCounter + 1
        self.id = MemoElement.memoIdCounter

        self.header = header
        self.content = content
        self.lineNumber = lineNumber
        self.categ = categ if categ is not None else Configuration.DEFAULT_CATEGORY

    def __repr__(self):
        return self.displayableRepresentation()

    def displayableRepresentation(self):
        output = Colors.BLUE + Configuration.MEMO_HEADER_MARK + str(self.id) + ": "
        output += "[" + self.categ + "] "
        output += self.header + Colors.ENDC + "\n"
        output += self.content

        return output

    def writableRepresentation(self):
        contentLines = self.content.split("\n")
        if contentLines[0].startswith("Date:"):
            modifiedContent = '\n'.join(contentLines[1:])
        else:
            modifiedContent = '\n'.join(contentLines)

        output = "\n\n"
        output += Configuration.MEMO_HEADER_MARK + " " + self.categ + " " + Configuration.MEMO_CATEGORY_MARK + " " + self.header + " \n"
        output += "Date: " + datetime.datetime.now().strftime("%Y-%m-%d %H:%M") + "\n"
        output += modifiedContent + "\n"

        return output

    def getHeader(self):
        return self.header

    def getContent(self):
        return self.content

    def getCategory(self):
        return self.categ


class MemoContainer:

    def __init__(self, path):
        self.path = path
        self.content = []
        self.load()

    def createEmptyMemo(self):
        with open(self.path, "a") as inFile:
            inFile.write("Memo" + os.linesep)
            inFile.write("----" + os.linesep + os.linesep)
            inFile.close()
            Logger.info("File have been created at: " + self.path)

    def load(self):
        if not os.path.isfile(self.path):
            Logger.warning("Memo file does not exist ...")
            self.createEmptyMemo()

        with open(self.path, "r") as inFile:

            lines = inFile.readlines()
            lines.append("##")

            category = ""
            header = ""
            content = ""

            headerRegex = "^ *" + Configuration.MEMO_HEADER_MARK + " *(?:(.+)" + Configuration.MEMO_CATEGORY_MARK + ")? *(.+)"

            for lineNumber, line in enumerate(lines):
                matcher = re.match(headerRegex, line)

                # this line is a memo header
                if matcher:
                    groups = matcher.groups()
                    category = (groups[0] if groups[0] is not None else Configuration.DEFAULT_CATEGORY)
                    header = groups[1]

                    print("category")
                    print(category)
                    print("header")
                    print(header)

                    self.content.append(MemoElement(header.strip(), content.strip(), category.strip(), lineNumber))
                    content = ""

                # this line is part of memo content
                elif re.search("\\w+", line):
                    content += line

    def searchByKeywords(self, keywords, categ=None):

        rslt = []

        # create a regex
        regexArray = []
        for w in keywords:
            regexArray.append(re.sub("[^a-z]", ".?", w, re.IGNORECASE))

        regex = "(" + "|".join(regexArray) + ")+"

        categ = categ.strip().lower() if categ != None else ""

        for memo in self.content:

            if categ and memo.getCategory() != categ:
                continue

            # Search in header and category
            inHeader = re.search(regex, memo.getHeader(), re.IGNORECASE)
            inCategory = re.search(regex, memo.getCategory(), re.IGNORECASE)

            # Search in content
            # Here we do not search in 'date' line
            contentLines = memo.getContent().split('\n')
            inContent = False

            # Content is one line
            if len(contentLines) < 2:
                inContent = re.search(regex, memo.getContent(), re.IGNORECASE)

            # Content is multi line
            else:
                for l in range(1, len(contentLines)):
                    inContent = re.search(regex, contentLines[l], re.IGNORECASE)
                    if inContent:
                        break

            if inHeader or inContent or inCategory:
                rslt.append(memo)

        return rslt

    def getById(self, id, categ=None):
        """
        Return a memo corresponding to the specified ID
        """

        for memo in self.content:
            if int(memo.id) == int(id):
                return memo

        return None

    def getContent(self, categ=None):

        if not categ:
            return self.content

        else:
            categ = categ.strip().lower()
            output = []
            for memo in self.content:
                if memo.getCategory() == categ:
                    output.append(memo)

            return output

    def modifyMemo(self, memo):
        """
        Modify a memo in storage file
        """
        memoReadFile = open(self.path, "r")
        fileLines = memoReadFile.readlines()
        memoReadFile.close()

        writableLines = memo.writableRepresentation().split('\n')

        i = memo.lineNumber
        l = 2  # skip firsts \n
        end = i + len(writableLines) - l
        while i < end:
            if re.search("\\w+", writableLines[l]):  # replace only non empty lines
                fileLines[i] = writableLines[l] + "\n"
            i += 1
            l += 1

        memoWriteFile = open(self.path, "w")
        memoWriteFile.writelines(fileLines)
        memoWriteFile.close()

    def deleteMemo(self, memo):
        """
        Delete a memo from storage file
        """
        memoReadFile = open(self.path, "r")
        fileLines = memoReadFile.readlines()
        memoReadFile.close()

        # first, delete memo content
        i = memo.lineNumber
        end = False
        while end != True and i < len(fileLines):
            if re.search("\\w+", fileLines[i]):
                del fileLines[i]
            else:
                end = True

        # then delete blank lines
        end = False
        while end != True and i < len(fileLines):
            if re.search("^\\s*$", fileLines[i]):
                del fileLines[i]
            else:
                end = True

        memoWriteFile = open(self.path, "w")
        memoWriteFile.writelines(fileLines)
        memoWriteFile.close()

    def appendMemo(self, memo):
        """
        Add a memo to path. Return false if an error occur.
        """
        try:
            inFile = open(self.path, "a")
            inFile.write(memo.writableRepresentation())
            inFile.close()
            return True
        except Exception as e:
            Logger.debug(e)
            return False


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

    if knownArgs.edit_all or knownArgs.graphical_editor:
        editor = Configuration.GRAPHICAL_EDITOR if knownArgs.graphical_editor == True else Configuration.CLI_EDITOR

        subprocess.call(editor + " " + Configuration.MEMO_FILE_PATH, shell=True)

        exitProgram(0)

    elif knownArgs.categorize:

        if len(unkArgs) < 1:
            exitProgram(1, "You must specify at least one memo id")

        category = knownArgs.categorize
        container = MemoContainer(Configuration.MEMO_FILE_PATH)

        error = False

        for memoId in unkArgs:

            Logger.info("Add category '" + category + "' to memo " + memoId)

            memo = container.getById(memoId)
            if not memo:
                Logger.error("Unknown memo id: " + memoId)
                error = True

            memo.categ = category
            success = container.modifyMemo(memo)

            if success == True:
                Logger.error("Category changed.")
            else:
                Logger.error("Error while changing category.")
                error = True

        if error:
            exitProgram(1)
        else:
            exitProgram(0)

    elif knownArgs.update:

        memoId = knownArgs.update

        if len(unkArgs) < 2:
            exitProgram(1, "You must specify at least a header and a content")

        container = MemoContainer(Configuration.MEMO_FILE_PATH)
        memo = container.getById(memoId)

        if not memo:
            Logger.error("Unknown memo id: " + memoId)
            exitProgram(1)

        if len(unkArgs) > 2:
            memo.header = unkArgs[1]
            memo.content = unkArgs[2]
            memo.categ = unkArgs[0]
        else:
            memo.header = unkArgs[0]
            memo.content = unkArgs[1]
            memo.categ = "default"

        success = container.modifyMemo(memo)

        if success == True:
            Logger.info("Memo updated.")
            exitProgram(0)
        else:
            exitProgram(1, "Error while changing category.")

    elif knownArgs.delete:

        memoId = knownArgs.delete

        container = MemoContainer(Configuration.MEMO_FILE_PATH)
        memo = container.getById(memoId)
        if not memo:
            Logger.error("Unknown memo id: " + memoId)
            exitProgram(1)

        success = container.deleteMemo(memo)

        if success == True:
            Logger.info("Memo deleted.")
            exitProgram(0)
        else:
            exitProgram(1, "Error while deleting memo.")

    elif knownArgs.append:

        if len(unkArgs) < 2:
            exitProgram(1, "You must specify at least a header and a content to add a memo")

        for i, val in enumerate(unkArgs):
            if len(val) < 1:
                exitProgram(1, "You can not specify empty arguments.")

        container = MemoContainer(Configuration.MEMO_FILE_PATH)

        memo = None
        if len(unkArgs) > 2:
            memo = MemoElement(header=unkArgs[1].strip(),
                               content=unkArgs[2].strip(),
                               categ=unkArgs[0].strip().lower())
        else:
            memo = MemoElement(header=unkArgs[0].strip(),
                               content=unkArgs[1].strip(),
                               categ=Configuration.DEFAULT_CATEGORY)

        success = container.appendMemo(memo)

        if success:
            Logger.info("Memo added with success.")
            exitProgram(0)
        else:
            exitProgram(1, "Error while adding memo to file: " + Configuration.MEMO_FILE_PATH)

    elif knownArgs.display:

        container = MemoContainer(Configuration.MEMO_FILE_PATH)

        Logger.info()

        if knownArgs.filter_category:
            Logger.info("Display only category: \"" + knownArgs.filter_category + "\"")
            Logger.info()

        for memo in container.getContent(knownArgs.filter_category):
            Logger.info(str(memo))
            Logger.info()

        exitProgram(0)

    elif knownArgs.search:

        if len(unkArgs) < 1:
            Logger.error("You must specify keywords.")
            Logger.error()
            parser.print_help()
            exitProgram(1)

        container = MemoContainer(Configuration.MEMO_FILE_PATH)

        Logger.info()

        if knownArgs.filter_category:
            Logger.warning("Display only category: \"" + knownArgs.filter_category + "\"")

        elements = container.searchByKeywords(unkArgs, knownArgs.filter_category)

        keywordsStr = ",".join(unkArgs)

        if len(elements) == 0:
            Logger.error("Nothing found for: \"" + keywordsStr + "\"")

        else:
            Logger.info("Results for \"" + keywordsStr + "\":")
            Logger.info()

            for m in elements:
                Logger.info(str(m))
                Logger.info()

        exitProgram(0)

    elif knownArgs.list_categories:

        container = MemoContainer(Configuration.MEMO_FILE_PATH)

        Logger.info()
        Logger.info("Categories: ")
        Logger.info()

        categories = {}
        for memo in container.getContent():
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

        exitProgram(0)

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
        Logger.debug(e)
        exit(1)
