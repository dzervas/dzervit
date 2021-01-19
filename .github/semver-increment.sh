#!/bin/bash
if [ "$#" -lt 1  ]; then
	echo "Usage: $0 <old_version> [new_version|increment_strategy]"
	exit 1
fi

export OLD_VERSION=$1
export MAJOR=$(echo -n "$OLD_VERSION" | sed -E "s/([0-9]+)\.([0-9]+)\.([0-9]+)/\1/")
export MINOR=$(echo -n "$OLD_VERSION" | sed -E "s/([0-9]+)\.([0-9]+)\.([0-9]+)/\2/")
export PATCH=$(echo -n "$OLD_VERSION" | sed -E "s/([0-9]+)\.([0-9]+)\.([0-9]+)/\3/")

if [[ "$2" == "major" ]]; then
	MAJOR=$(($MAJOR + 1))
	MINOR=0
	PATCH=0
elif [[ "$2" == "minor" ]]; then
	MINOR=$(($MINOR + 1))
	PATCH=0
elif [[ "$2" == "patch" ]]; then
	PATCH=$(($PATCH + 1))
elif [ "$#" -lt 2  ]; then
	# No strategy/new version was given. Increment patch
	PATCH=$(($PATCH + 1))
else
	# Custom version, print it and exit
	echo -n "$2"
	exit 0
fi

echo -n "$MAJOR.$MINOR.$PATCH"
