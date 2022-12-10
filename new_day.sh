#!/bin/bash

set -uex

SRC=$(dirname "${BASH_SOURCE[0]}")/src

day=$1
mod_name="day_$(printf '%02d' "$1")"

mkdir "$SRC/$mod_name"
sed "s|xx|$day|g" "$SRC/day_xx/mod.rs" > "$SRC/$mod_name/mod.rs"
sed -i -zE "s/(.*)(\naoc_lib.*)/\1pub mod $mod_name;\n\2/" "$SRC/lib.rs"
sed -i -zE 's/(.*)(\n\s*println!\("done.*)/\1    run_day_with_generator!('"$mod_name, \"$day\");\n\2/" "$SRC/bin/bin.rs"
