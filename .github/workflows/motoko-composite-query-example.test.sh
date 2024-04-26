#!/bin/bash
dfx start --background
  pushd motoko/composite_query
  make test
  popd