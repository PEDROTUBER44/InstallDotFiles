#!/bin/sh

echo "Log in as root" ;
sleep 5 ;
sudo su &&
rm -r /usr/bin/installdotfiles &&
echo "InstallDotFiles was successfully removed" ;