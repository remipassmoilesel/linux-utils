#!/usr/bin/python3
# -*- coding: utf-8 -*-
import unittest
from MemoContainer import *
from testData import *


class MemoContainerTest(unittest.TestCase):

    def getTestContainer(self):
        container = MemoContainer()
        lines = sampleMemoStorage.split('\n')
        container.loadTextLines(lines)
        return container

    def test_parsingShouldWork(self):
        container = self.getTestContainer()
        memos = container.getMemoList()

        self.assertEqual(memos[0].getCategory(), "memo1-category1")
        self.assertEqual(memos[0].getHeader(), "memo1-header1")
        self.assertEqual(memos[0].getContent(), "memo1-l1memo1-l2")

        self.assertEqual(memos[1].getCategory(), "default")
        self.assertEqual(memos[1].getHeader(), "memo2-header1")
        self.assertEqual(memos[1].getContent(), "memo2-l1memo2-l2")

        self.assertEqual(memos[2].getCategory(), "default")
        self.assertEqual(memos[2].getHeader(), "memo3-header1")
        self.assertEqual(memos[2].getContent(), "memo3-l1memo3-l2")

        self.assertEqual(len(memos), 3)

    def test_searchWithSingleLetterShouldFail(self):
        container = self.getTestContainer()

        memos = container.searchByKeywords(["a"])
        self.assertEqual(len(memos), 0)

    def test_searchWithWordShouldSucced(self):
        container = self.getTestContainer()

        memos = container.searchByKeywords(["memo1"])
        self.assertEqual(len(memos), 1)

    def test_getByIdShouldSucceed(self):
        container = self.getTestContainer()
        self.assertEqual(container.getMemoById(1).header, "memo1-header1")

    def test_getByIdShouldFail(self):
        container = self.getTestContainer()
        with self.assertRaises(Exception):
            container.getMemoById(0)

    def test_updateShouldSucced(self):
        container = self.getTestContainer()

        memo0 = container.getMemoById(1)
        memo0.header = "test updated header"

        container.updateMemo(memo0)

        updatedMemo = container.getMemoById(1)
        self.assertEqual(updatedMemo.header, "test updated header")

if __name__ == '__main__':
    unittest.main()