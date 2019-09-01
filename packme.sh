#!/bin/zsh 

apt_extra_version=0.0.2 #Change me every release
script_full_path=$(dirname "$0")

cd $script_full_path/add-apt-repositories
cargo build --release 2> /dev/null
printf "Packaging Serna APT Extra $apt_extra_version"
cd ..
mkdir "serna-apt-extra_"$apt_extra_version"_darwin-amd64" && cd "serna-apt-extra_"$apt_extra_version"_darwin-amd64"
mkdir ./usr ./DEBIAN && mkdir ./usr/local && mkdir ./usr/local/bin
cp ../add-apt-repositories/target/release/add-apt-repositories ./usr/local/bin
touch DEBIAN/control
find . -name .DS_Store -type f -delete
chmod 0755 ./DEBIAN ./DEBIAN/* ./usr/local/bin/add-apt-repositories
SIZE=$(du -sk ./usr | awk '{print $1}')
echo "Package: serna-apt-extra
Version: "$apt_extra_version"
Architecture: darwin-amd64
Priority: optional
Section: Addons
Maintainer: Nikan Radan (SmushyTaco)
Installed-Size: "$SIZE"
Depends: apt
Homepage: https://sernarepo.com
Description: Additional tools for MacPT/Project Serna, that provide essential CLI features.
" >> ./DEBIAN/control
cd .. && find . -name .DS_Store -type f -delete
dpkg-deb -b "serna-apt-extra_"$apt_extra_version"_darwin-amd64" 1> /dev/null
