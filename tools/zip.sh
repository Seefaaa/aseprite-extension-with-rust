#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root_dir="$(cd "$script_dir/.." && pwd)"
lua_dir="$root_dir/lua"
dist_dir="$root_dir/dist"
zip_path="$dist_dir/ext.zip"

case "$(uname -s)" in
Linux) artifact=libtest.so ;;
Darwin) artifact=libtest.dylib ;;
*)
	echo "error: unsupported platform $(uname -s)" >&2
	exit 1
	;;
esac

(cd "$root_dir" && cargo build --release)

if [ ! -d "$lua_dir" ]; then
	echo "error: $lua_dir not found" >&2
	exit 1
fi

cp -f "$root_dir/target/release/$artifact" "$lua_dir/$artifact"

mkdir -p "$dist_dir"
rm -f "$zip_path"
(cd "$lua_dir" && zip -r "$zip_path" .)

echo "created $zip_path"
