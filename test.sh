#!/usr/bin/env sh

set -e

FILE=""
TEMP_FILE="temp.txt"

if [ -z "$1" ]; then
  >&2 echo "Missing input file argument"
  exit
else
  FILE="$1"
fi

if [ -f "$FILE" ]; then
  # NOTE: CHANGE THESE SED 2p to 2,3p for 2 lines of input etc
  INPUT=`sed -n '2p' "$FILE"`
  EXPECTED_OUTPUT=`sed -n '4p' "$FILE"`
  # Write temporary file
  echo "$INPUT" > "$TEMP_FILE"
  truncate -s -1 "$TEMP_FILE"
  OUTPUT=`cargo run "$TEMP_FILE"`
  rm "$TEMP_FILE"

  if [ "$OUTPUT" = "$EXPECTED_OUTPUT" ]; then
    echo "Yay, it passed!"
  else
    >&2 echo "Nay, you shall not pass!"
    >&2 echo "Expected: $EXPECTED_OUTPUT"
    >&2 echo "But got:  $OUTPUT"
    exit 1
  fi
else
  >&2 echo "File: ${FILE} does not exist"
  exit
fi