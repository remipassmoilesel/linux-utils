# -*- coding: utf-8 -*-
import re

from Logger import *
from MemoElement import *


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
