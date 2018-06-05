
from Logger import Logger
from MemoContainer import MemoContainer
from MemoElement import MemoElement
from Configuration import Configuration
import subprocess

class CliHandlers:

    def __init__(self):
        self.container = self.getAndLoadMemoContainer()

    def openEditor(self, graphicalEditor=False):

        editor = Configuration.GRAPHICAL_EDITOR if graphicalEditor is True else Configuration.CLI_EDITOR
        subprocess.call(editor + " " + Configuration.MEMO_FILE_PATH, shell=True)

    def exitProgram(code=0, msg=""):

        if msg != "":
            Logger.error(msg)

        exit(code)

    def categorizeMemo(self, category, memoIds):

        for memoId in memoIds:
            Logger.header("Add category '" + category + "' to memo " + memoId)

            memo = self.container.getMemoById(memoId)
            if not memo:
                raise Exception("Unknown memo id: " + memoId)

            memo.categ = category
            self.container.updateMemo(memo)
            self.container.persistToStorage()

            Logger.success("Category changed.")
            Logger.success()

    def updateMemo(self, memoId, category, header, content):

        memo = self.container.getMemoById(memoId)
        if not memo:
            raise Exception("Unknown memo id: " + memoId)

        memo.categ = category
        memo.header = header
        memo.content = content

        self.container.updateMemo(memo)
        self.container.persistToStorage()

        Logger.success("Memo updated.")

    def deleteMemo(self, memoId):

        memo = self.container.getMemoById(memoId)
        if not memo:
            raise Exception("Unknown memo id: " + memoId)

        self.container.deleteMemo(memo)
        self.container.persistToStorage()

        Logger.success("Memo deleted.")

    def appendMemo(self, category, header, content):

        memo = MemoElement(id=None,
                           category=category,
                           header=header,
                           content=content)

        self.container.appendMemo(memo)
        self.container.persistToStorage()

        Logger.success("Memo added with success.")

    def displayMemos(self, categoryFilter=None):

        if categoryFilter:
            Logger.warning("Display only category: \"" + categoryFilter + "\"")
            Logger.info()

        for memo in self.container.getMemoList(categoryFilter):
            Logger.info(memo.getDisplayRepresentation())
            Logger.info()

    def searchAndDisplay(self, keywords, categoryFilter):

        foundElements = self.container.searchByKeywords(keywords, categoryFilter)

        keywordsStr = ", ".join(keywords)

        if len(foundElements) == 0:
            Logger.error("Nothing found for: \"" + keywordsStr + "\"")

        else:
            Logger.header("Results for \"" + keywordsStr + "\":")
            Logger.info()

            for memo in foundElements:
                Logger.info(memo.getDisplayRepresentation())
                Logger.info()


    def getAndLoadMemoContainer(self):

        container = MemoContainer()
        container.loadStorageFile(Configuration.MEMO_FILE_PATH)
        return container
