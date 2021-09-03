# reels 

### Initializing the repository

```
$ reels init ./repo
```

### Create a snapshot from a directory

```
$ reels create -r ./repo ~/Documents
```


### Remove a snapshot

```
$ reels delete 7ff9e792349cb2397a7391 -r ./repo
```


### List snapshots

```
$ reels list snapshots
```
   

### Recover from snapshot

```
$ reels recover -r ./repo -o ./recovered  /Documents
```
