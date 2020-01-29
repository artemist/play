#!/usr/bin/env bash

name=$(ykman oath list | wofi -k /dev/null -i -d)

line=$(ykman oath code $name | head -n 1)
if test -n "$line"
then
	echo $line | awk '{print $NF}' | wl-copy
fi
