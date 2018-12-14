#!/bin/bash

set -eu

outdir=~/Pictures/Screenshots
mkdir -p "${outdir}"

file=$(mktemp --suffix .png)
trap "rm \"${file}\"" EXIT

leanshot -o "${file}" $@

out="${outdir}/$(sha1sum "${file}" | cut -d ' ' -f 1).png"
cp "${file}" "${out}"
echo "${out}"
