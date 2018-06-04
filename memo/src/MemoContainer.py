# -*- coding: utf-8 -*-
import re

from MemoElement import *

# TODO:
# - Make persistence happen on demand, on write() call

class MemoContainer:

    def __init__(self):
        self.memoList = []

    def createEmptyStorage(self):
        with open(self.path, "a") as inFile:
            inFile.write(os.linesep)
            inFile.close()
            Logger.info("File have been created at: " + self.path)

    def loadStorage(self, path):
        self.path = path
        if not os.path.isfile(self.path):
            Logger.warning("Memo file does not exist ...")
            self.createEmptyStorage()

        with open(self.path, "r") as inFile:
            lines = inFile.readlines()
            self.memoList = self.parseLines(lines)

    def parseLines(self, lines):

        lines.append(Configuration.MEMO_HEADER_MARK + " Fake header")
        memoList = []

        category = ""
        header = ""
        content = ""

        headerRegex = "^ *" + Configuration.MEMO_HEADER_MARK + " *(?:(.+)" + Configuration.MEMO_CATEGORY_MARK + ")? *(.+)"

        for lineNumber, line in enumerate(lines):
            matcher = re.match(headerRegex, line)

            # this line is a memo header
            if matcher:

                if content and category:
                    memoList.append(MemoElement(header.strip(), content.strip(), category.strip(), lineNumber))

                groups = matcher.groups()
                category = (groups[0] if groups[0] is not None else Configuration.DEFAULT_CATEGORY)
                header = groups[1]

                content = ""

            # this line is part of memo content
            elif re.search("\\w+", line):
                content += line

        return memoList

    def searchByKeywords(self, keywords, categ=None):

        rslt = []

        # create a regex
        regexArray = []
        for w in keywords:
            regexArray.append(re.sub("[^a-z]", ".?", w, re.IGNORECASE))

        regex = "(" + "|".join(regexArray) + ")+"

        categ = categ.strip().lower() if categ != None else ""

        for memo in self.memoList:

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

    def getById(self, id):

        for memo in self.memoList:
            if int(memo.id) == int(id):
                return memo

        return None

    def getContent(self, categ=None):

        if not categ:
            return self.memoList

        else:
            categ = categ.strip().lower()
            result = []
            for memo in self.memoList:
                if memo.getCategory() == categ:
                    result.append(memo)

            return result

    def modifyMemo(self, memo):
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
        try:
            inFile = open(self.path, "a")
            inFile.write(memo.writableRepresentation())
            inFile.close()
            return True
        except Exception as e:
            Logger.debug(e)
            return False
