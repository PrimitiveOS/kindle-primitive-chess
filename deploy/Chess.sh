#!/bin/sh
# Name: Primitive Chess
# Author: Primitive AI
# DontUseFBInk

app=/mnt/us/chess/bin/chess-kindle
[ -x "$app" ] || exit 1
work=$(mktemp -d /var/tmp/primitive-chess.XXXXXX) || exit 1
trap 'rm -rf "$work"' EXIT
cp "$app" "$work/run" && chmod 755 "$work/run" || exit 1
cd / || exit 1
"$work/run"
