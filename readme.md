Git PR script

```
gh pr create --base master --head <branch-name> --title "Closes Issue #<issue-number>" --body "Closes #<issue-number>."
gh pr merge <issue-number> --merge
git checkout master
git pull
git push origin --delete <issue-number>
```

Example:

```
gh pr create --base master --head issue_18 --title "Closes Issue #18" --body "Closes #18."
gh pr merge issue_18 --merge
git checkout master
git pull
git push origin --delete issue_18
```
