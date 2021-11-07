merge-folder-lines
---
Merges a list of unix folder/file names to a set of distinct locations.

input
---
lines each containing a single valid unix path name (or file name)

output
---
merged set of distinct path names, where:
- child folders are dropped, where this folder is contained in another parent folder
- duplicates are removed

original purpose
---
create a distinct set of backup source folder names - originated from different sources

Report bugs to bitmagier@mailbox.org