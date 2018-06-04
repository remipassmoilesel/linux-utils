# -*- coding: utf-8 -*-

import datetime
from Logger import *
from Configuration import *


class MemoElement:
    memoIdCounter = 0

    def __init__(self, header, content, categ, lineNumber=0):
        MemoElement.memoIdCounter = MemoElement.memoIdCounter + 1
        self.id = MemoElement.memoIdCounter

        self.header = header
        self.content = content
        self.lineNumber = lineNumber
        self.categ = categ if categ is not None else Configuration.DEFAULT_CATEGORY

    # TODO use explicit call to self.displayableRepresentation for display, in order to get better debug display
    def __repr__(self):
        return self.displayableRepresentation()

    def displayableRepresentation(self):
        output = Colors.BLUE + Configuration.MEMO_HEADER_MARK + str(self.id) + ": "
        output += "[" + self.categ + "] "
        output += self.header + Colors.ENDC + "\n"
        output += self.content

        return output

    # TODO: extract date and modify it only on demand
    def getWritableRepresentation(self):
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
