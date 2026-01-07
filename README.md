# Advent of code solutions.

My implementation of advent of code stuff.

## Running

```shell
$ make
```

## Building with podman

If container environment is your thing

```shell
podman build . -t rustadvent
podman run -it --rm -v $(pwd):$(pwd):rw -w $(pwd) rustadvent \
  cargo build

or 

../../cargowrapper.sh test
../../cargowrapper.sh build
```

## Copying

A BSD-style license.
