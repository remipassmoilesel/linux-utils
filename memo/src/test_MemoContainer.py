#!/usr/bin/python3
# -*- coding: utf-8 -*-
import unittest
from MemoContainer import *
from testData import *


class MemoContainerTest(unittest.TestCase):

    def test_parsingShouldWork(self):
        container = MemoContainer()
        lines = sampleMemoStorage.split('\n')

        memos = container.parseLines(lines)

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
        container = MemoContainer()
        lines = sampleMemoStorage.split('\n')
        container.loadTextLines(lines)

        memos = container.searchByKeywords(["a"])
        self.assertEqual(len(memos), 0)

    def test_searchWithWordShouldSucced(self):
        container = MemoContainer()
        lines = sampleMemoStorage.split('\n')
        container.loadTextLines(lines)

        memos = container.searchByKeywords(["memo1"])
        self.assertEqual(len(memos), 1)

if __name__ == '__main__':
    unittest.main()