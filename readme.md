Git PR script

```shell
gh pr create --base master --head <branch-name> --title "Closes Issue #<issue-number>" --body "Closes #<issue-number>."
gh pr merge <issue-number> --merge
git checkout master
git pull
git push origin --delete <issue-number>
```

Example:

```shell
gh pr create --base master --head issue_18 --title "Closes Issue #18" --body "Closes #18."
gh pr merge issue_18 --merge
git checkout master
git pull
git push origin --delete issue_18
```

Autogen example:

```shell
../target/debug/aarya_cli autogen \
--course-id 1002 \
--course-name "AP Computer Science A" \
--chapter-name "Primitive Types" \
--chapter-id 1010 \
--topic-name "Mathematical Operations" \
--topic-id 1004 \
--count 10 \
--prompt-path ../.prompts/prompt.txt
```

Batchgen example 1:

```shell
../target/debug/aarya_cli batchgen \
--course-id 1002 \
--count 2 \
--prompt-path ../.prompts/prompt.txt
```

Batchgen example 2:

```shell
../target/debug/aarya_cli batchgen \
--course-id 1002 \
--chapter-id 1014 \
--count 5 \
--prompt-path ../.prompts/prompt.txt
```

Batchgen example 3:

```shell
../target/debug/aarya_cli batchgen \
--course-id 1002 \
--chapter-id 1016 \
--count 1 \
--prompt-path ../.prompts/prompt.txt \
--screenshot-path ./.temp-data/.screenshots
```

Batchupload example:

```shell
../target/debug/aarya_cli batch-upload \
--schema-file ../.schema/question-schema.json \
--directory ./.temp-data/course-1002-chapter-1018-42697000
```
