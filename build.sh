#!/bin/sh

cargo build --release ;
rm -rf app/* ;
cp target/release/installenv app/ ;
cp -r rpms/* app/ ;
rm -rf target/ ;
echo "<<<>>>" ;