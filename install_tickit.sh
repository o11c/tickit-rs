#!/bin/sh
if pkg-config --exists tickit
then
    exit 0
fi

echo 'Install tickit NYI'
exit 1
