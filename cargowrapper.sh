#!/bin/bash
# pre-prepare with:
# podman build . -t rustadvent

podman run -it --rm -v $(pwd):$(pwd):rw -w $(pwd) rustadvent \
  cargo "$@"
