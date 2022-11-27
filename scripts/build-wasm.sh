#!/usr/bin/env bash

# clean
rm -rf pkg

# build
pushd libcorn || exit
wasm-pack build --out-name index --out-dir ../pkg -- --features wasm
popd || exit

# patch
pushd pkg || exit

mv index.js index.mjs

jq '.type="module"' package.json | jq . > tmp && mv tmp package.json
jq '.module="./index.mjs"' package.json | jq . > tmp && mv tmp package.json
jq '.files += ["index.mjs"]' package.json | jq . > tmp && mv tmp package.json

popd || exit
