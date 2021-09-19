#!/bin/sh

BIN_PATH='./target/debug/reels'
REPO_PATH='./sandbox/repo'
RUST_LOG=${log_level}

echo "- Recreating sandbox…"
rm -r ./sandbox/origin
mkdir -p ./sandbox/origin
mkdir -p ./sandbox/origin/empty_dir
mkdir -p ./sandbox/origin/dir1 && touch ./sandbox/origin/dir1/file.txt
mkdir -p ./sandbox/origin/dir2 && echo "CONTENT" > ./sandbox/origin/dir2/file.txt
dd if=/dev/urandom of=./sandbox/origin/file1.bin bs=1024 count=1024 2> /dev/null
dd if=/dev/urandom of=./sandbox/origin/file2.bin bs=1024 count=512 2> /dev/null
dd if=/dev/urandom of=./sandbox/origin/file3.bin bs=1024 count=256 2> /dev/null
echo ---------------------------------------------------------------------

echo "- Show version"
${BIN_PATH} --version
echo ---------------------------------------------------------------------

echo "- Initialize repository"
${BIN_PATH} init $REPO_PATH
echo ---------------------------------------------------------------------

echo "- Create a first snapshot"
${BIN_PATH} create --repo $REPO_PATH -c ./sandbox ./sandbox/origin 
echo ---------------------------------------------------------------------

echo "- Create snapshot including a new large file" \
  && dd if=/dev/urandom of=./sandbox/origin/random_large.bin bs=1048576 count=16 2> /dev/null
${BIN_PATH} create --repo $REPO_PATH -c ./sandbox ./sandbox/origin 
echo ---------------------------------------------------------------------

echo "- Create snapshot including created directory and file" \
  && mkdir ./sandbox/origin/new_empty_dir \
  && dd if=/dev/urandom of=./sandbox/origin/random_small.bin bs=1024 count=5 2> /dev/null
${BIN_PATH} create --repo $REPO_PATH -c ./sandbox ./sandbox/origin 
echo ---------------------------------------------------------------------

echo "- Create snapshot after removing a file" \
  && rm ./sandbox/origin/file1.bin
${BIN_PATH} create --repo $REPO_PATH -c ./sandbox ./sandbox/origin 
echo ---------------------------------------------------------------------

echo "- Listing created snapshots"
${BIN_PATH} list snapshots -r $REPO_PATH 
echo ---------------------------------------------------------------------

echo "- Recover the latest created snapshot"
LATEST_SNAPSHOT=`${BIN_PATH} list snapshots -r $REPO_PATH | cut -c 2-13 | tail -n 1`
${BIN_PATH} recover --repo $REPO_PATH -o ./sandbox/recovered ${LATEST_SNAPSHOT}
echo ---------------------------------------------------------------------

echo "- Delete the latest snapshot"
${BIN_PATH} delete --repo $REPO_PATH ${LATEST_SNAPSHOT}
echo ---------------------------------------------------------------------

echo "- List snapshots again"
${BIN_PATH} list snapshots -r $REPO_PATH 
echo ---------------------------------------------------------------------

echo "- Differences between source and recovery: " \
  && diff -r ./sandbox/origin ./sandbox/recovered/origin 
echo ---------------------------------------------------------------------

echo All finished.
