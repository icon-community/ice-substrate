#!/bin/bash

CURTAG=`git describe --abbrev=0 --tags`;

if [ -z "$CURTAG" ]
then
    CURTAG="v0.0.0"
    echo "\$CURTAG is empty"
else
    echo "\$CURTAG is NOT empty"
fi

CURTAG="${CURTAG/v/}"
IFS='.' read -a vers <<< "$CURTAG"

MAJ=${vers[0]}
MIN=${vers[1]}
BUG=${vers[2]}
echo "Current Tag: v$MAJ.$MIN.$BUG"

for cmd in "$@"
do
	case $cmd in
		"--major")
			((MAJ+=1))
			MIN=0
			BUG=0
			echo "Incrementing Major Version#"
			;;
		"--minor")
			((MIN+=1))
			BUG=0
			echo "Incrementing Minor Version#"
			;;
		"--bug")
			((BUG+=1))
			echo "Incrementing Bug Version#"
			;;
	esac
done

NEWTAG="v$MAJ.$MIN.$BUG"
echo $NEWTAG

echo "::set-output name=tag_name::$NEWTAG"

exit