#!/bin/bash

__rust_prompt() {
  local rustp=$(rustup-version-name)
  if [ -n "$rustp"  ] && [ "$rustp" != "default"  ]
  then
    echo " ${P_RUBY}$rustp${P_NO_COLOUR}"
  fi
}

__rust_prompt
