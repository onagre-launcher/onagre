#!/usr/bin/env bash
set -euo pipefail

export ONAGRE_HOME="$(cd "$(dirname "$0")/.." && pwd)"

echoerr() {
   echo "$@" 1>&2
}

release() {

   TAR_DIR="${ONAGRE_HOME}/target/tar"

   target="${1:-}"
   if [[ $target == *"osx"* ]]; then
      echoerr "OSX cross-compile is impossible. Fallbacking to cargo..."
      target=""
   fi

   cd "$ONAGRE_HOME"

   rm -rf "${ONAGRE_HOME}/target" 2> /dev/null || true

   if [ -n "$target" ]; then
      cargo install --version 0.1.16 cross 2> /dev/null || true
      cross build --release --target "$target"
      bin_folder="${target}/release"
   else
      cargo build --release
      bin_folder="release"
   fi

   bin_path="${ONAGRE_HOME}/target/${bin_folder}/onagre"
   chmod +x "$bin_path"
   mkdir -p "$TAR_DIR" 2> /dev/null || true

   cp "$bin_path" "$TAR_DIR"
   cp "$ONAGRE_HOME/LICENSE" "$TAR_DIR"

   cd "$TAR_DIR"
   tar -czf onagre.tar.gz *

}

cmd="$1"
shift

release "$@"