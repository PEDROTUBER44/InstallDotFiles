#!/bin/sh

cargo build --release ;
rm -rf app/* ;
cp target/release/installdotfiles app/ ;
cp -r rpms/* app/ ;
rm -rf target/ ;
echo "<<<>>>" ;
