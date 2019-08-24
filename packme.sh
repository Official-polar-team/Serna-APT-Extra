#!/bin/bash

script_full_path=$(dirname "$0")
cd $script_full_path
VERSION=0.0.1
echo "Packaging Serna-APT-Extra"
mkdir $script_full_path/release && mkdir $script_full_path/release/usr && mkdir $script_full_path/release/usr/local && mkdir $script_full_path/release/usr/local/bin
make install DESTDIR=$script_full_path/release
cd $script_full_path/release
mkdir ./DEBIAN
SIZE=$(du -sk ./usr | awk '{print $1}')
echo "Package: Serna-APT-Extra
Version: "$VERSION"
Architecture: darwin-amd64
Priority: optional
Section: Addons
Maintainer: Diego Magdaleno
Installed-Size: "$SIZE"
Depends: apt
Homepage: https://sernarepo.com
Description: Add repos with ease.
" >> ./DEBIAN/control
mkdir "Serna-APT-Extra_"$VERSION"_darwin-amd64"
cp -r DEBIAN $script_full_path/"Serna-APT-Extra_"$VERSION"_darwin-amd64" && cp -r usr $script_full_path/"Serna-APT-Extra_"$VERSION"_darwin-amd64"
find . -name .DS_Store -type f -delete
dpkg-deb -b "Serna-APT-Extra_"$VERSION"_darwin-amd64" 1> /dev/null
echo "Done."
rm -rf "Serna-APT-Extra_"$VERSION"_darwin-amd64" 
cd ..
rm -rf release