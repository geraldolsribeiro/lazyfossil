#!/bin/bash

if [ ! -f .mailmap ]; then
  echo "Geraldo Ribeiro <geraldolsribeiro@gmail.com> geraldo" >.mailmap
fi

fossil git export ~/git/geraldolsribeiro/lazyfossil --autopush git@github.com:geraldolsribeiro/lazyfossil.git
