
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


    def getAndLoadMemoContainer(self):
        container = MemoContainer()
        container.loadStorageFile(Configuration.MEMO_FILE_PATH)
        return container
