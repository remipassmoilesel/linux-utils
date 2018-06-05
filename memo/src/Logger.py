# -*- coding: utf-8 -*-

from Configuration import Configuration


class Colors:
    PURPLE = '\033[95m'
    BLUE = '\033[94m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    ENDC = '\033[0m'
    UNDERLINE = '\033[4m'


class Logger:

    @staticmethod
    def header(line=""):
        Logger.printColor(line, Colors.UNDERLINE)

    @staticmethod
    def success(line=""):
        Logger.printColor(line, Colors.GREEN)

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
