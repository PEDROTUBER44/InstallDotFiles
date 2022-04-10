#!/bin/sh

echo "Log in as root" ;
sleep 5 ;
sudo su &&
cp app/installdotfiles /usr/bin/ &&
chmod +x /usr/bin/installdotfiles &&
echo "InstallDotFiles has been installed" ;