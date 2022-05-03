#!/bin/sh

cargo build --release ;
rm -rf app/* ;
cp target/release/installdotfiles app/ ;
rm -rf target/ ;
echo "InstallDotFiles successfully compiled" ;