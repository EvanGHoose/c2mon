#C2mon-Core

C2mon-Core is a tool that take a list of IP addresses and ports as input, and
then outputs a file that shows whether or not those connections were 
successful. It is designed to run as a service, so that you can have another
program read in the file it outputs and display the information in a more 
friendly manner.

This repository also includes install and uninstall scripts.

#File Locations
Input file: /etc/c2mon/c2mon-core.targets
Output file: /etc/c2mon/c2mon-core.output

#Installing/Uninstalling

##Install Script
The install script needs to be run with root privleges.

The install script will move c2mon-core into /sbin, create the directory
/etc/c2mon , and move the targets and uninstall files into /etc/c2mon .

##Uninstall Script
Also needs to be run with root privleges.

Assuming the install script has been run, this will clean up all files
moved/created by the install script, in addition to the output file created
by c2mon-core.

##Manual Install/Uninstall
c2mon-core has three expectations.

1) c2mon-core.targets to be in /etc/c2mon
2) to be able to write to /etc/c2mon
3) to be able to make outbound TCP connections

Any way you can provide it these three things, it should work. As far as
uninstalling is concerned, only you know how to uninstall what you manually
set up.
