## Done 

- List snapshots
- Remove a snapshot
- Copy data from snapshot 
- Check if recovery directory does not exist
- Support for UTF-8 characters in paths
- Multi-thread 

## Pending

- Mount snapshot on directory (fuse?)
- Store passed root directory in order to check added/removed/modified files between snapshots
- Store database version and a command to migrate
- List files in a snapshot
- Difference between two snapshots
- Maintain block table instead of having pointers at the start of each block
- Show repository stats (size, compressed/uncompressed, files, directories, snapshots,...)
- Add metadata (tag, hostname,…) to snapshots
- Create a lock file to protect from already started process. Provide a way to unlock the repository.
- Ensure that file deduplication works as expected
- Purge repository from orphan blocks
- Fancy progress log (probably using crossterm)
- Check consistency of repository
- Find a file or directory by name or reference ID
- Manage password and chunk encryption AES-256
- Network interface (socket? REST?)
- Better database (sqlite3 replacement)
- Add network based backends (Backblaze, S3,…)
- Web UI for management
