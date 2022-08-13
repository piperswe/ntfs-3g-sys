#!/bin/bash

set -euxo pipefail

VERSION=2022.5.17
DIRNAME="ntfs-3g_ntfsprogs-$VERSION"
FILENAME="$DIRNAME.tgz"
URL="https://tuxera.com/opensource/$FILENAME"

rm -rf ntfs-3g
wget -O "$FILENAME" "$URL"
tar -xvf "$FILENAME"
mv "$DIRNAME" ntfs-3g
rm "$FILENAME"

patch ntfs-3g/libntfs-3g/Makefile.in dont-move-libraries-to-root.patch
