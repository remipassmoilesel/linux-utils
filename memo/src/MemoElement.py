# -*- coding: utf-8 -*-

import datetime

from Logger import Colors
from Configuration import Configuration


class MemoElement:

    DATE_PREFIX = "Date:"

    def __init__(self, id, category, header, content):
        self.id = id
        self.category = category if category is not None else Configuration.DEFAULT_CATEGORY
        self.header = header
        self.dateLine = self.getDateLine(content)
        self.content = self.stripDateLine(content)

    def __repr__(self):
        return "<MemoElement.MemoElement: " + ", ".join([
            str(self.id),
            str(self.category),
            str(self.header),
            str(self.content)
        ]) + ">"

    def getDisplayRepresentation(self):
        output = Colors.BLUE + Configuration.MEMO_HEADER_MARK + str(self.id) + ": "
        output += "[" + self.category + "] "
        output += self.header + Colors.ENDC + "\n"
        if self.dateLine is not None:
            output += self.dateLine + "\n"
        output += self.content

        return output

    def getWritableRepresentation(self):
        output = "\n"
        output += Configuration.MEMO_HEADER_MARK + " " + self.category + " " + Configuration.MEMO_CATEGORY_MARK + " " + self.header + " \n"
        if self.dateLine is not None:
            output += self.dateLine + "\n"
        output += self.content + "\n"

        return output

    def updateDate(self):
        self.dateLine = MemoElement.DATE_PREFIX + " " + datetime.datetime.now().strftime("%Y-%m-%d %H:%M")

    def getDateLine(self, content):
        contentLines = content.split("\n")
        if contentLines[0].startswith(MemoElement.DATE_PREFIX):
            return contentLines[0]
        else:
            return None

    def stripDateLine(self, content):
        contentLines = content.split("\n")
        if contentLines[0].startswith(MemoElement.DATE_PREFIX):
            modifiedContent = '\n'.join(contentLines[1:])
        else:
            modifiedContent = '\n'.join(contentLines)
        return modifiedContent


    def getHeader(self):
        return self.header

    def getContent(self):
        return self.content

    def getCategory(self):
        return self.category
