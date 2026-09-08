#!/usr/bin/env bash
set -euo pipefail

artifact_dir=${1:?npm artifact directory is required}
tag=${2:?npm dist-tag is required}

publish_package() {
  local archive=$1
  local metadata name version encoded status
  metadata=$(tar -xOf "$archive" package/package.json)
  name=$(node -e 'const p=JSON.parse(process.argv[1]); process.stdout.write(p.name)' "$metadata")
  version=$(node -e 'const p=JSON.parse(process.argv[1]); process.stdout.write(p.version)' "$metadata")
  encoded=$(node -e 'process.stdout.write(encodeURIComponent(process.argv[1]))' "$name")
  if [[ "$version" == *-* && "$tag" == latest ]]; then
    echo "refusing to publish prerelease $name@$version with the latest dist-tag" >&2
    exit 1
  fi
  status=$(curl --silent --show-error --output /dev/null --write-out '%{http_code}' \
    "https://registry.npmjs.org/$encoded/$version")

  case "$status" in
    200) echo "$name@$version already exists on npm; skipping" ;;
    404) npm publish "$archive" --ignore-scripts --access public --tag "$tag" ;;
    *) echo "npm registry returned HTTP $status for $name@$version" >&2; exit 1 ;;
  esac
}

main_package=''
package_count=0
for archive in "$artifact_dir"/*.tgz; do
  test -f "$archive"
  package_count=$((package_count + 1))
  metadata=$(tar -xOf "$archive" package/package.json)
  name=$(node -e 'const p=JSON.parse(process.argv[1]); process.stdout.write(p.name)' "$metadata")
  if [[ "$name" == '@stdbr/stdbr' ]]; then
    main_package=$archive
  else
    publish_package "$archive"
  fi
done

if [[ $package_count -gt 1 ]]; then
  test -n "$main_package"
  publish_package "$main_package"
fi
