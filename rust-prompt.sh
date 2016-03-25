#!/bin/bash

__rust_prompt() {
  local rustp=$(multirust-version-name) 
  if [ -n "$rustp"  ] && [ "$rustp" != "default"  ]
  then
    echo " ${P_RUBY}$rustp${P_NO_COLOUR}"
  fi
}

__rust_prompt
