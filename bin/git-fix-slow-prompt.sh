#!/bin/bash

# These commands disable prompts status checks from oh my zsh plugins.
# In large or special repositories, status checks can take a long time.

git config oh-my-zsh.hide-info 1
git config oh-my-zsh.hide-status 1
git config oh-my-zsh.hide-dirty 1

