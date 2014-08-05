#!/bin/bash -e
if pkg-config --exists tickit
then
    exit 0
fi

if test -e tickit-c
then
    echo 'Error: package not installed, but residue found' >&2
    exit 1
fi

. ./install.conf.sh
make="${make} PREFIX=${prefix}"

git clone --depth 1 git://github.com/o11c/libtickit.git -b o11c tickit-c
${make} -j2 -C tickit-c
${make} -j2 -C tickit-c test
${sudo} ${make} -j2 -C tickit-c install
${sudo} ${ldconfig}
