#!/bin/sh -e
echo '#[link_args = "'$(pkg-config --libs tickit) $(pkg-config --libs-only-L tickit | sed 's/-L/-Wl,-rpath=/g')'"] extern {}' > src/generated_link.rs
