#! /bin/sh
#
# This should be run when there is relevant updates in kth-style to this project.
# The relevant file(s) from kth-style is commited in the source of the kpm
# project, to keep track of when things change.
#
set -e

SCRIPT=`realpath $0`
KPM=`dirname "$SCRIPT"`
KPM=`dirname "$KPM"`
TMP=`mktemp --tmpdir -d social-style-XXXXXXXXXX`
ARCHIVE="kth-style-scss.zip"
DISTDIR="$TMP/node_modules/kth-style/dist"

(cd "$TMP" && npm i --no-fund --no-optional --only=prod --no-save --ignore-scripts --no-audit kth-style)

cp "$TMP/node_modules/kth-style/dist/css/kth-bootstrap.css" "$KPM/style/"

#rm -rf "$KPM/style/kth-style"
#mkdir "$KPM/style/kth-style"
#(cd "$KPM/style/kth-style" && unzip -q "$DISTDIR/$ARCHIVE")
rm -rf "$TMP"
