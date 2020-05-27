#!/bin/sh

mkdir /etc/c2mon/
mv ./c2mon-core.targets /etc/c2mon/
mv ./uninstall.sh /etc/c2mon
rustc ./c2mon-core.rs
mv ./c2mon-core /sbin
