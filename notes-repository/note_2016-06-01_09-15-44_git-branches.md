# Travailler avec Git et les branches

Source: https://www.atlassian.com/git/tutorials/using-branches/git-merge

    # Start a new feature
    git checkout -b new-feature master

    # Edit some files
    git add <file>
    git commit -m "Start a feature"

    # Edit some files
    git add <file>
    git commit -m "Finish a feature"

    # Develop the master branch
    git checkout master

    # Edit some files
    git add <file>
    git commit -m "Make some super-stable changes to master"

    # Merge in the new-feature branch
    git merge --no-ff new-feature
    git branch -d new-feature
