# merge-folder-names

CLI tool to merge folder names.

Creates a merged, distinct list of unix file/folder names from any list of file/folder names.
Overlapping locations and duplicates are merged, so that a distinct set remains, that covers all input locations.

## Input

Lines, each containing a single valid unix path/file name

Example input:
```
/home/foo/Documents
/home/foo/Documents/important-documents
/home/foo/.ssh/id_rsa
/home/foo/.ssh
/home/foo/.gnupg
```

## Output

Merged set of distinct path names, where:
- child folder names are elided, when folder is contained in a parent folder
- duplicates are removed

Example output:
```
/home/foo/Documents
/home/foo/.ssh
/home/foo/.gnupg
```

## Purpose

Create a distinct set of backup source folder names (input comes from different files)
This is just my use case. There may be many others.

Report bugs to bitmagier@mailbox.org
