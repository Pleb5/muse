#!/usr/bin/env bash

# Check if user provided a valid argument
if [ "$#" -ne 1 ]; then
	printf "Use only 1 parameter, the input absolute file path.\n Exiting..."
	exit 1
fi


input_file="$1"

sed -E "s/\bnsec[a-zA-Z0-9]{28}\b/**** # MODIFY ME!!!/g" "$input_file"
